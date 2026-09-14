use crate::k4_bytes::{concat, copy, has, range, split, starts_with};
use crate::{k4_coverage_rows, k4_file_checks, k4_rows};
use ckc_spec::check::*;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub fn header() -> (r: Vec<u8>)
    ensures
        r@ == coverage_header(),
{
    let r = copy(
        b"# format: id<TAB>file<TAB>page<TAB>section<TAB>status\n# status: ace(<docid>) | restates(<id>) | uncovered(<class>: <one-clause reason>) | pending\n# uncovered classes: heading | process | \x65xternal | aim | descriptive | notice\n",
    );
    proof {
        reveal_byteslit(
            b"# format: id<TAB>file<TAB>page<TAB>section<TAB>status\n# status: ace(<docid>) | restates(<id>) | uncovered(<class>: <one-clause reason>) | pending\n# uncovered classes: heading | process | \x65xternal | aim | descriptive | notice\n",
        );
        reveal_strlit(
            "# format: id<TAB>file<TAB>page<TAB>section<TAB>status\n# status: ace(<docid>) | restates(<id>) | uncovered(<class>: <one-clause reason>) | pending\n# uncovered classes: heading | process | \x65xternal | aim | descriptive | notice\n",
        );
        reveal(ckc_spec::v1text::ascii);
        assert(r@ == coverage_header());
    }
    r
}

pub fn failure(e: Vec<u8>) -> (r: EVerdict)
    ensures
        r@ == fail("coverage"@, e@),
{
    let c = copy(b"coverage");
    let ghost text = e@;
    proof {
        reveal_byteslit(b"coverage");
        reveal_strlit("coverage");
        reveal(ckc_spec::v1text::ascii);
        assert(c@ == ckc_spec::v1text::ascii("coverage"@));
    }
    let r = EVerdict::Fail(c, e);
    proof {
        assert(r@ == fail("coverage"@, text));
    }
    r
}

pub fn error(e: Vec<u8>) -> (r: Result<ECoverage, EVerdict>)
    ensures
        coverage_result(r) == Result::Err(fail("coverage"@, e@)),
{
    Err(failure(e))
}

pub fn check(
    bytes: &[u8],
    docids: &Vec<Vec<u8>>,
    table: &Vec<(Vec<u8>, EFileSrc)>,
    root: &[u8],
) -> (r: Result<ECoverage, EVerdict>)
    ensures
        coverage_result(r) == coverage(
            bytes@,
            byte_rows(docids@),
            |f: Seq<u8>| source_lookup(source_pairs(table@), f, 0),
            root@,
        ),
{
    proof {
        reveal(ckc_spec::v1text::ascii);
        reveal_byteslit(b"carriage return byte in ledger");
        reveal_strlit("carriage return byte in ledger");
        assert(b"carriage return byte in ledger"@ == ckc_spec::v1text::ascii(
            "carriage return byte in ledger"@,
        ));
        reveal_byteslit(b"ledger lacks final newline");
        reveal_strlit("ledger lacks final newline");
        assert(b"ledger lacks final newline"@ == ckc_spec::v1text::ascii(
            "ledger lacks final newline"@,
        ));
        reveal_byteslit(b"header bytes drift");
        reveal_strlit("header bytes drift");
        assert(b"header bytes drift"@ == ckc_spec::v1text::ascii("header bytes drift"@));
        reveal_byteslit(b"ledger holds no rows");
        reveal_strlit("ledger holds no rows");
        assert(b"ledger holds no rows"@ == ckc_spec::v1text::ascii("ledger holds no rows"@));
        reveal_byteslit(b"docid without a coverage row: ");
        reveal_strlit("docid without a coverage row: ");
        assert(b"docid without a coverage row: "@ == ckc_spec::v1text::ascii(
            "docid without a coverage row: "@,
        ));
    }
    if has(bytes, 0x0d) {
        return error(copy(b"carriage return byte in ledger"));
    }
    if bytes.len() == 0 || bytes[bytes.len() - 1] != 0x0a {
        return error(copy(b"ledger lacks final newline"));
    }
    let h = header();
    if !starts_with(bytes, &h) {
        return error(copy(b"header bytes drift"));
    }
    let body = range(bytes, h.len(), bytes.len());
    let mut lines = split(&body, 0x0a);
    let ghost before = lines@;
    let last = lines.pop();
    proof {
        assert_seqs_equal!(byte_rows(lines@) == byte_rows(before).drop_last());
        assert_seqs_equal!(body@ == bytes@.skip(coverage_header().len() as int));
        assert(byte_rows(lines@) == body_lines(bytes@.skip(coverage_header().len() as int)));
    }
    let rows = match k4_rows::parse(&lines, docids) {
        Ok(rs) => rs,
        Err(e) => return error(e),
    };
    if rows.len() == 0 {
        return error(copy(b"ledger holds no rows"));
    }
    match k4_coverage_rows::restates(&rows) {
        Some(e) => return error(e),
        None => {},
    }
    match k4_coverage_rows::unclaimed(&rows, docids) {
        Some(d) => return error(concat(b"docid without a coverage row: ", &d)),
        None => {},
    }
    let files = k4_coverage_rows::files(&rows);
    let sources = k4_file_checks::read_sources(&files, table);
    let result = k4_file_checks::check(&rows, &files, &sources, root);
    proof {
        assert(k4_file_checks::result_view(result) == files_check(
            crate::k4_payload::rows(rows@),
            byte_rows(files@),
            byte_rows(files@).map_values(|f: Seq<u8>| source_lookup(source_pairs(table@), f, 0)),
            root@,
            0,
        ));
        let rs = crate::k4_payload::rows(rows@);
        let fs = byte_rows(files@);
        let ts = fs.map_values(|f: Seq<u8>| source_lookup(source_pairs(table@), f, 0));
        assert(parse_rows(
            body_lines(bytes@.skip(coverage_header().len() as int)),
            0,
            byte_rows(docids@),
            Seq::empty(),
        ) == Result::Ok(rs));
        assert(restates_check(rs, 0) is None);
        assert(first_unclaimed(rs, byte_rows(docids@), 0) is None);
        assert(fs == files_of(rs, 0, Seq::empty()));
        assert(rs.len() == rows@.len());
        assert(rs.len() > 0);
        assert(!has_byte(bytes@, 0x0d));
        assert(bytes@.len() > 0 && bytes@.last() == 0x0a);
        assert(starts(bytes@, coverage_header()));
        let lookup = |f: Seq<u8>| source_lookup(source_pairs(table@), f, 0);
        assert_seqs_equal!(fs.map_values(|f: Seq<u8>| lookup(f)) == ts);
        reveal(coverage);
        assert(coverage(
            bytes@,
            byte_rows(docids@),
            |f: Seq<u8>| source_lookup(source_pairs(table@), f, 0),
            root@,
        ) == match files_check(rs, fs, ts, root@, 0) {
            Result::Ok(es) => Result::Ok(Coverage { rows: rs, files: fs, evidence: es }),
            Result::Err(e) => Result::Err(fail("coverage"@, e)),
        });
    }
    let evidence = match result {
        Ok(es) => es,
        Err(e) => return error(e),
    };
    Ok(ECoverage { rows, files, evidence })
}

} // verus!
