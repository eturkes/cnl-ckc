use crate::k4_bytes::{append, concat, contains, copy, eq, has, range, split, starts_with};
#[cfg(verus_keep_ghost)]
use crate::k4_payload::rows as row_views;
use crate::k4_scalar::docid;
use crate::k4_search::sub;
use ckc_spec::check::*;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub open spec fn status_result(r: Result<EStatus, Vec<u8>>) -> Result<Status, Seq<u8>> {
    match r {
        Ok(s) => Ok(s@),
        Err(e) => Err(e@),
    }
}

pub open spec fn row_result(r: Result<ECoverageRow, Vec<u8>>) -> Result<Row, Seq<u8>> {
    match r {
        Ok(s) => Ok(s@),
        Err(e) => Err(e@),
    }
}

pub open spec fn rows_result(r: Result<Vec<ECoverageRow>, Vec<u8>>) -> Result<Seq<Row>, Seq<u8>> {
    match r {
        Ok(s) => Ok(row_views(s@)),
        Err(e) => Err(e@),
    }
}

pub fn named(prefix: &[u8], id: &[u8], suffix: &[u8], value: &[u8]) -> (r: Vec<u8>)
    ensures
        r@ == prefix@ + id@ + suffix@ + value@,
{
    let mut r = concat(prefix, id);
    append(&mut r, suffix);
    append(&mut r, value);
    r
}

pub fn wrap(s: &[u8], head: &[u8]) -> (r: Option<Vec<u8>>)
    ensures
        optional_bytes(r) == if starts(s@, head@) && s@.len() > head@.len() && s@.last() == 0x29 {
            Some(s@.skip(head@.len() as int).drop_last())
        } else {
            None
        },
{
    if starts_with(s, head) && s.len() > head.len() && s[s.len() - 1] == 0x29 {
        let r = range(s, head.len(), s.len() - 1);
        proof {
            assert_seqs_equal!(r@ == s@.skip(head@.len() as int).drop_last());
        }
        Some(r)
    } else {
        None
    }
}

pub fn class_ok(c: &[u8]) -> (r: bool)
    ensures
        r == uncovered_class_ok(c@),
{
    let r = eq(c, b"heading") || eq(c, b"process") || eq(c, b"\x65xternal") || eq(c, b"aim") || eq(
        c,
        b"descriptive",
    ) || eq(c, b"notice");
    proof {
        reveal(ckc_spec::v1text::ascii);
        reveal_byteslit(b"heading");
        reveal_strlit("heading");
        assert(b"heading"@ == ckc_spec::v1text::ascii("heading"@));
        reveal_byteslit(b"process");
        reveal_strlit("process");
        assert(b"process"@ == ckc_spec::v1text::ascii("process"@));
        reveal_byteslit(b"\x65xternal");
        reveal_strlit("\x65xternal");
        assert(b"\x65xternal"@ == ckc_spec::v1text::ascii("\x65xternal"@));
        reveal_byteslit(b"aim");
        reveal_strlit("aim");
        assert(b"aim"@ == ckc_spec::v1text::ascii("aim"@));
        reveal_byteslit(b"descriptive");
        reveal_strlit("descriptive");
        assert(b"descriptive"@ == ckc_spec::v1text::ascii("descriptive"@));
        reveal_byteslit(b"notice");
        reveal_strlit("notice");
        assert(b"notice"@ == ckc_spec::v1text::ascii("notice"@));
        reveal(ckc_spec::v1text::ascii);
        assert(r == uncovered_class_ok(c@));
    }
    r
}

pub fn status(id: &[u8], s: &[u8]) -> (r: Result<EStatus, Vec<u8>>)
    ensures
        status_result(r) == status_of(id@, s@),
{
    proof {
        reveal(ckc_spec::v1text::ascii);
        reveal_byteslit(b"pending");
        reveal_strlit("pending");
        assert(b"pending"@ == ckc_spec::v1text::ascii("pending"@));
        reveal_byteslit(b"ace(");
        reveal_strlit("ace(");
        assert(b"ace("@ == ckc_spec::v1text::ascii("ace("@));
        reveal_byteslit(b"restates(");
        reveal_strlit("restates(");
        assert(b"restates("@ == ckc_spec::v1text::ascii("restates("@));
        reveal_byteslit(b"uncovered(");
        reveal_strlit("uncovered(");
        assert(b"uncovered("@ == ckc_spec::v1text::ascii("uncovered("@));
        reveal_byteslit(b": ");
        reveal_strlit(": ");
        assert(b": "@ == ckc_spec::v1text::ascii(": "@));
        reveal_byteslit(b"malformed status for ");
        reveal_strlit("malformed status for ");
        assert(b"malformed status for "@ == ckc_spec::v1text::ascii("malformed status for "@));
        reveal_byteslit(b"ace names invalid docid for ");
        reveal_strlit("ace names invalid docid for ");
        assert(b"ace names invalid docid for "@ == ckc_spec::v1text::ascii(
            "ace names invalid docid for "@,
        ));
        reveal_byteslit(b"empty restates target for ");
        reveal_strlit("empty restates target for ");
        assert(b"empty restates target for "@ == ckc_spec::v1text::ascii(
            "empty restates target for "@,
        ));
        reveal_byteslit(b"uncovered without class and reason for ");
        reveal_strlit("uncovered without class and reason for ");
        assert(b"uncovered without class and reason for "@ == ckc_spec::v1text::ascii(
            "uncovered without class and reason for "@,
        ));
        reveal_byteslit(b"unknown uncovered class for ");
        reveal_strlit("unknown uncovered class for ");
        assert(b"unknown uncovered class for "@ == ckc_spec::v1text::ascii(
            "unknown uncovered class for "@,
        ));
        reveal_byteslit(b"empty uncovered reason for ");
        reveal_strlit("empty uncovered reason for ");
        assert(b"empty uncovered reason for "@ == ckc_spec::v1text::ascii(
            "empty uncovered reason for "@,
        ));
        reveal_byteslit(b"unknown status for ");
        reveal_strlit("unknown status for ");
        assert(b"unknown status for "@ == ckc_spec::v1text::ascii("unknown status for "@));
    }
    if eq(s, b"pending") {
        return Ok(EStatus::Pending);
    }
    if starts_with(s, b"ace(") {
        return match wrap(s, b"ace(") {
            None => Err(named(b"malformed status for ", id, b": ", s)),
            Some(d) => if docid(&d) {
                Ok(EStatus::Ace(d))
            } else {
                Err(named(b"ace names invalid docid for ", id, b": ", &d))
            },
        };
    }
    if starts_with(s, b"restates(") {
        return match wrap(s, b"restates(") {
            None => Err(named(b"malformed status for ", id, b": ", s)),
            Some(t) => if t.len() == 0 {
                Err(concat(b"empty restates target for ", id))
            } else {
                Ok(EStatus::Restates(t))
            },
        };
    }
    if starts_with(s, b"uncovered(") {
        return match wrap(s, b"uncovered(") {
            None => Err(named(b"malformed status for ", id, b": ", s)),
            Some(inner) => {
                let k = sub(&inner, b": ", 0);
                if k >= inner.len() {
                    Err(named(b"uncovered without class and reason for ", id, b": ", s))
                } else {
                    let class = range(&inner, 0, k);
                    if !class_ok(&class) {
                        Err(named(b"unknown uncovered class for ", id, b": ", &class))
                    } else if k + 2 == inner.len() {
                        Err(concat(b"empty uncovered reason for ", id))
                    } else {
                        Ok(EStatus::Uncovered)
                    }
                }
            },
        };
    }
    Err(named(b"unknown status for ", id, b": ", s))
}

pub fn has_id(rs: &Vec<ECoverageRow>, id: &[u8]) -> (r: bool)
    ensures
        r == (exists|j: int| 0 <= j < rs@.len() && row_views(rs@)[j].id == id@),
{
    let mut i = 0usize;
    while i < rs.len()
        invariant
            i <= rs@.len(),
            forall|j: int| 0 <= j < i ==> rs@[j]@.id != id@,
        decreases rs.len() - i,
    {
        if eq(&rs[i].id, id) {
            proof {
                assert(row_views(rs@)[i as int].id == id@);
            }
            return true;
        }
        i += 1;
    }
    false
}

pub fn claimed(rs: &Vec<ECoverageRow>, d: &[u8]) -> (r: bool)
    ensures
        r == claims(row_views(rs@), d@),
{
    let mut i = 0usize;
    while i < rs.len()
        invariant
            i <= rs@.len(),
            forall|j: int| 0 <= j < i ==> ace_docid(rs@[j]@) != Some(d@),
        decreases rs.len() - i,
    {
        let found = match &rs[i].status {
            EStatus::Ace(id) => eq(id, d),
            _ => false,
        };
        if found {
            proof {
                assert(ace_docid(row_views(rs@)[i as int]) == Some(d@));
            }
            return true;
        }
        i += 1;
    }
    proof {
        assert forall|j: int| 0 <= j < row_views(rs@).len() implies ace_docid(row_views(rs@)[j])
            != Some(d@) by {
            assert(row_views(rs@)[j] == rs@[j]@);
        }
        reveal(claims);
    }
    false
}

pub fn row(line: &[u8], seen: &Vec<ECoverageRow>) -> (r: Result<ECoverageRow, Vec<u8>>)
    ensures
        row_result(r) == parse_row(line@, row_views(seen@)),
{
    proof {
        reveal(ckc_spec::v1text::ascii);
        reveal_byteslit(b"row without 5 columns: ");
        reveal_strlit("row without 5 columns: ");
        assert(b"row without 5 columns: "@ == ckc_spec::v1text::ascii("row without 5 columns: "@));
        reveal_byteslit(b"empty region id: ");
        reveal_strlit("empty region id: ");
        assert(b"empty region id: "@ == ckc_spec::v1text::ascii("empty region id: "@));
        reveal_byteslit(b"region id holds a space: ");
        reveal_strlit("region id holds a space: ");
        assert(b"region id holds a space: "@ == ckc_spec::v1text::ascii(
            "region id holds a space: "@,
        ));
        reveal_byteslit(b"file outside source/ for ");
        reveal_strlit("file outside source/ for ");
        assert(b"file outside source/ for "@ == ckc_spec::v1text::ascii(
            "file outside source/ for "@,
        ));
        reveal_byteslit(b"file path traversal for ");
        reveal_strlit("file path traversal for ");
        assert(b"file path traversal for "@ == ckc_spec::v1text::ascii(
            "file path traversal for "@,
        ));
        reveal_byteslit(b"empty page for: ");
        reveal_strlit("empty page for: ");
        assert(b"empty page for: "@ == ckc_spec::v1text::ascii("empty page for: "@));
        reveal_byteslit(b"empty section for: ");
        reveal_strlit("empty section for: ");
        assert(b"empty section for: "@ == ckc_spec::v1text::ascii("empty section for: "@));
        reveal_byteslit(b"duplicate region id: ");
        reveal_strlit("duplicate region id: ");
        assert(b"duplicate region id: "@ == ckc_spec::v1text::ascii("duplicate region id: "@));
        reveal_byteslit(b"source/");
        reveal_strlit("source/");
        assert(b"source/"@ == ckc_spec::v1text::ascii("source/"@));
        reveal_byteslit(b"..");
        reveal_strlit("..");
        assert(b".."@ == ckc_spec::v1text::ascii(".."@));
        reveal_byteslit(b": ");
        reveal_strlit(": ");
        assert(b": "@ == ckc_spec::v1text::ascii(": "@));
    }
    let fs = split(line, 0x09);
    if fs.len() != 5 {
        return Err(concat(b"row without 5 columns: ", line));
    }
    let id = &fs[0];
    let file = &fs[1];
    let page = &fs[2];
    let section = &fs[3];
    if id.len() == 0 {
        return Err(concat(b"empty region id: ", line));
    }
    if has(id, 0x20) {
        return Err(concat(b"region id holds a space: ", id));
    }
    if !starts_with(file, b"source/") {
        return Err(named(b"file outside source/ for ", id, b": ", file));
    }
    if sub(file, b"..", 0) < file.len() {
        return Err(named(b"file path traversal for ", id, b": ", file));
    }
    if page.len() == 0 {
        return Err(concat(b"empty page for: ", id));
    }
    if section.len() == 0 {
        return Err(concat(b"empty section for: ", id));
    }
    if has_id(seen, id) {
        return Err(concat(b"duplicate region id: ", id));
    }
    match status(id, &fs[4]) {
        Err(e) => Err(e),
        Ok(status) => {
            let mut bytes = copy(line);
            bytes.push(0x0a);
            proof {
                assert_seqs_equal!(bytes@ == line@ + seq![0x0au8]);
            }
            let ghost st = status@;
            let id_bytes = copy(id);
            let file_bytes = copy(file);
            let row = ECoverageRow { id: id_bytes, file: file_bytes, status, line: bytes };
            proof {
                assert(row@ == Row {
                    id: id@,
                    file: file@,
                    status: st,
                    line: line@ + seq![0x0au8],
                });
                assert(parse_row(line@, row_views(seen@)) == Result::Ok(row@));
            }
            Ok(row)
        },
    }
}

pub proof fn push_row(rs: Seq<ECoverageRow>, r: ECoverageRow)
    ensures
        row_views(rs.push(r)) == row_views(rs).push(r@),
{
    assert_seqs_equal!(row_views(rs.push(r)) == row_views(rs).push(r@));
}

pub fn parse(lines: &Vec<Vec<u8>>, docids: &Vec<Vec<u8>>) -> (r: Result<Vec<ECoverageRow>, Vec<u8>>)
    ensures
        rows_result(r) == parse_rows(byte_rows(lines@), 0, byte_rows(docids@), Seq::empty()),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        assert(row_views(out@) == Seq::<Row>::empty());
    }
    while i < lines.len()
        invariant
            i <= lines@.len(),
            parse_rows(byte_rows(lines@), 0, byte_rows(docids@), Seq::empty()) == parse_rows(
                byte_rows(lines@),
                i as nat,
                byte_rows(docids@),
                row_views(out@),
            ),
        decreases lines.len() - i,
    {
        let line = &lines[i];
        proof {
            reveal(ckc_spec::v1text::ascii);
            reveal_byteslit(b"comment line after rows: ");
            reveal_strlit("comment line after rows: ");
            assert(b"comment line after rows: "@ == ckc_spec::v1text::ascii(
                "comment line after rows: "@,
            ));
            reveal_byteslit(b"docid claimed by two rows: ");
            reveal_strlit("docid claimed by two rows: ");
            assert(b"docid claimed by two rows: "@ == ckc_spec::v1text::ascii(
                "docid claimed by two rows: "@,
            ));
            reveal_byteslit(b"ace names unknown docid for ");
            reveal_strlit("ace names unknown docid for ");
            assert(b"ace names unknown docid for "@ == ckc_spec::v1text::ascii(
                "ace names unknown docid for "@,
            ));
            reveal_byteslit(b": ");
            reveal_strlit(": ");
            assert(b": "@ == ckc_spec::v1text::ascii(": "@));
            reveal_with_fuel(parse_rows, 2);
        }
        if line.len() > 0 && line[0] == 0x23 {
            if out.len() > 0 {
                return Err(concat(b"comment line after rows: ", line));
            }
        } else {
            match row(line, &out) {
                Err(e) => return Err(e),
                Ok(r) => {
                    match &r.status {
                        EStatus::Ace(d) => {
                            if claimed(&out, d) {
                                return Err(concat(b"docid claimed by two rows: ", d));
                            }
                            if !contains(docids, d) {
                                return Err(named(b"ace names unknown docid for ", &r.id, b": ", d));
                            }
                        },
                        _ => {},
                    }
                    proof {
                        push_row(out@, r);
                    }
                    out.push(r);
                },
            }
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(parse_rows, 2);
    }
    Ok(out)
}

} // verus!
