#[cfg(verus_keep_ghost)]
use crate::k4_bytes::byte_rows_push;
use crate::k4_bytes::{append, concat, contains, copy, has, less, range, split, starts_with};
use crate::k4_scalar::{docid, hex, n_plus, number};
use ckc_spec::check::*;
use ckc_spec::replay::ESrc;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub open spec fn result_view(r: (Vec<EBundle>, Option<Vec<u8>>)) -> (Seq<Bundle>, Option<Seq<u8>>) {
    (bundles(r.0@), optional_bytes(r.1))
}

pub fn error(rows: Vec<EBundle>, e: Vec<u8>) -> (r: (Vec<EBundle>, Option<Vec<u8>>))
    ensures
        result_view(r) == (bundles(rows@), Some(e@)),
{
    (rows, Some(e))
}

pub fn detail(i: usize, what: &[u8]) -> (r: Vec<u8>)
    ensures
        r@ == ckc_spec::v1text::ascii("manifest row "@) + nat_bytes(i as nat + 3) + seq![0x20u8]
            + what@,
{
    let mut r = copy(b"manifest row ");
    append(&mut r, &n_plus(i, 3));
    r.push(0x20);
    append(&mut r, what);
    proof {
        reveal_byteslit(b"manifest row ");
        reveal_strlit("manifest row ");
        reveal(ckc_spec::v1text::ascii);
    }
    r
}

pub proof fn bundles_push(v: Seq<EBundle>, b: EBundle)
    ensures
        bundles(v.push(b)) == bundles(v).push(b@),
{
    assert_seqs_equal!(bundles(v.push(b)) == bundles(v).push(b@));
}

pub fn rows(lines: &Vec<Vec<u8>>) -> (r: (Vec<EBundle>, Option<Vec<u8>>))
    ensures
        result_view(r) == parse_rows_of(byte_rows(lines@), 0, 3, Seq::empty(), Seq::empty()),
{
    let mut out = Vec::new();
    let mut ids = Vec::new();
    let mut prev = Vec::new();
    let mut i = 0usize;
    proof {
        assert(bundles(out@) == Seq::<Bundle>::empty());
        assert(byte_rows(ids@) == Seq::<Seq<u8>>::empty());
    }
    while i < lines.len()
        invariant
            i <= lines@.len(),
            byte_rows(ids@) == bundles(out@).map_values(|b: Bundle| b.docid),
            parse_rows_of(byte_rows(lines@), 0, 3, Seq::empty(), Seq::empty()) == parse_rows_of(
                byte_rows(lines@),
                i as nat,
                i as nat + 3,
                prev@,
                bundles(out@),
            ),
        decreases lines.len() - i,
    {
        proof {
            reveal(ckc_spec::v1text::ascii);
            reveal_byteslit(b"field-count ");
            reveal_strlit("field-count ");
            assert(b"field-count "@ == ckc_spec::v1text::ascii("field-count "@));
            reveal_byteslit(b"docid-grammar");
            reveal_strlit("docid-grammar");
            assert(b"docid-grammar"@ == ckc_spec::v1text::ascii("docid-grammar"@));
            reveal_byteslit(b"duplicate-docid ");
            reveal_strlit("duplicate-docid ");
            assert(b"duplicate-docid "@ == ckc_spec::v1text::ascii("duplicate-docid "@));
            reveal_byteslit(b"sort-order ");
            reveal_strlit("sort-order ");
            assert(b"sort-order "@ == ckc_spec::v1text::ascii("sort-order "@));
            reveal_byteslit(b" after ");
            reveal_strlit(" after ");
            assert(b" after "@ == ckc_spec::v1text::ascii(" after "@));
            reveal_byteslit(b"ace_sha256");
            reveal_strlit("ace_sha256");
            assert(b"ace_sha256"@ == ckc_spec::v1text::ascii("ace_sha256"@));
            reveal_byteslit(b"coverage_row_sha256");
            reveal_strlit("coverage_row_sha256");
            assert(b"coverage_row_sha256"@ == ckc_spec::v1text::ascii("coverage_row_sha256"@));
            reveal_byteslit(b"region_payload_sha256");
            reveal_strlit("region_payload_sha256");
            assert(b"region_payload_sha256"@ == ckc_spec::v1text::ascii("region_payload_sha256"@));
            reveal_byteslit(b"semantic_clause_sha256");
            reveal_strlit("semantic_clause_sha256");
            assert(b"semantic_clause_sha256"@ == ckc_spec::v1text::ascii(
                "semantic_clause_sha256"@,
            ));
            reveal_byteslit(b"review_sha256");
            reveal_strlit("review_sha256");
            assert(b"review_sha256"@ == ckc_spec::v1text::ascii("review_sha256"@));
            reveal_with_fuel(parse_rows_of, 2);
        }
        let fs = split(&lines[i], 0x09);
        if fs.len() != 6 {
            let mut e = detail(i, b"field-count ");
            append(&mut e, &number(fs.len()));
            return error(out, e);
        }
        if !docid(&fs[0]) {
            return error(out, detail(i, b"docid-grammar"));
        }
        if contains(&ids, &fs[0]) {
            let mut e = detail(i, b"duplicate-docid ");
            append(&mut e, &fs[0]);
            return error(out, e);
        }
        if less(&fs[0], &prev) {
            let mut e = detail(i, b"sort-order ");
            append(&mut e, &fs[0]);
            append(&mut e, b" after ");
            append(&mut e, &prev);
            return error(out, e);
        }
        if !hex(&fs[1], 64) {
            return error(out, detail(i, b"ace_sha256"));
        }
        if !hex(&fs[2], 64) {
            return error(out, detail(i, b"coverage_row_sha256"));
        }
        if !hex(&fs[3], 64) {
            return error(out, detail(i, b"region_payload_sha256"));
        }
        if !hex(&fs[4], 64) {
            return error(out, detail(i, b"semantic_clause_sha256"));
        }
        if !hex(&fs[5], 64) {
            return error(out, detail(i, b"review_sha256"));
        }
        let b = EBundle {
            docid: copy(&fs[0]),
            ace: copy(&fs[1]),
            cov: copy(&fs[2]),
            pay: copy(&fs[3]),
            cl: copy(&fs[4]),
            review: copy(&fs[5]),
        };
        let id = copy(&fs[0]);
        prev = copy(&fs[0]);
        proof {
            bundles_push(out@, b);
            byte_rows_push(ids@, id);
        }
        out.push(b);
        ids.push(id);
        i += 1;
    }
    proof {
        reveal_with_fuel(parse_rows_of, 2);
    }
    (out, None)
}

pub fn first_header() -> (h: Vec<u8>)
    ensures
        h@ == manifest_header_1(),
{
    let h = copy(
        b"# format: docid<TAB>ace_sha256<TAB>coverage_row_sha256<TAB>region_payload_sha256<TAB>semantic_clause_sha256<TAB>review_sha256\n",
    );
    proof {
        reveal_byteslit(
            b"# format: docid<TAB>ace_sha256<TAB>coverage_row_sha256<TAB>region_payload_sha256<TAB>semantic_clause_sha256<TAB>review_sha256\n",
        );
        reveal_strlit(
            "# format: docid<TAB>ace_sha256<TAB>coverage_row_sha256<TAB>region_payload_sha256<TAB>semantic_clause_sha256<TAB>review_sha256\n",
        );
        reveal(ckc_spec::v1text::ascii);
        assert(h@ == manifest_header_1());
    }
    h
}

pub fn second_header() -> (h: Vec<u8>)
    ensures
        h@ == manifest_header_2(),
{
    let h = copy(
        b"# bundle v2; review_sha256 = sha256 of the labeled component-digest block; regenerate: python3 -P tools/goal.py review-manifest <id>; do not edit.\n",
    );
    proof {
        reveal_byteslit(
            b"# bundle v2; review_sha256 = sha256 of the labeled component-digest block; regenerate: python3 -P tools/goal.py review-manifest <id>; do not edit.\n",
        );
        reveal_strlit(
            "# bundle v2; review_sha256 = sha256 of the labeled component-digest block; regenerate: python3 -P tools/goal.py review-manifest <id>; do not edit.\n",
        );
        reveal(ckc_spec::v1text::ascii);
        assert(h@ == manifest_header_2());
    }
    h
}

pub fn parse(src: &ESrc, path: &[u8]) -> (r: (Vec<EBundle>, Option<Vec<u8>>))
    ensures
        result_view(r) == parse_manifest(src@, path@),
{
    proof {
        reveal(ckc_spec::v1text::ascii);
        reveal_byteslit(b"manifest missing: ");
        reveal_strlit("manifest missing: ");
        assert(b"manifest missing: "@ == ckc_spec::v1text::ascii("manifest missing: "@));
        reveal_byteslit(b"manifest encoding");
        reveal_strlit("manifest encoding");
        assert(b"manifest encoding"@ == ckc_spec::v1text::ascii("manifest encoding"@));
        reveal_byteslit(b"manifest carriage-return");
        reveal_strlit("manifest carriage-return");
        assert(b"manifest carriage-return"@ == ckc_spec::v1text::ascii(
            "manifest carriage-return"@,
        ));
        reveal_byteslit(b"manifest final-newline");
        reveal_strlit("manifest final-newline");
        assert(b"manifest final-newline"@ == ckc_spec::v1text::ascii("manifest final-newline"@));
        reveal_byteslit(b"manifest header line 1");
        reveal_strlit("manifest header line 1");
        assert(b"manifest header line 1"@ == ckc_spec::v1text::ascii("manifest header line 1"@));
        reveal_byteslit(b"manifest header line 2");
        reveal_strlit("manifest header line 2");
        assert(b"manifest header line 2"@ == ckc_spec::v1text::ascii("manifest header line 2"@));
        reveal_byteslit(b"manifest holds no rows: ");
        reveal_strlit("manifest holds no rows: ");
        assert(b"manifest holds no rows: "@ == ckc_spec::v1text::ascii(
            "manifest holds no rows: "@,
        ));
    }
    match src {
        ESrc::Missing => error(Vec::new(), concat(b"manifest missing: ", path)),
        ESrc::Bad(_) => error(Vec::new(), copy(b"manifest encoding")),
        ESrc::Bytes(b) => {
            if has(b, 0x0d) {
                return error(Vec::new(), copy(b"manifest carriage-return"));
            }
            if b.len() == 0 || b[b.len() - 1] != 0x0a {
                return error(Vec::new(), copy(b"manifest final-newline"));
            }
            let h1 = first_header();
            let h2 = second_header();
            if !starts_with(b, &h1) {
                return error(Vec::new(), copy(b"manifest header line 1"));
            }
            let remainder = range(b, h1.len(), b.len());
            if !starts_with(&remainder, &h2) {
                return error(Vec::new(), copy(b"manifest header line 2"));
            }
            let body = range(&remainder, h2.len(), remainder.len());
            let mut lines = split(&body, 0x0a);
            let ghost before = lines@;
            let last = lines.pop();
            proof {
                assert_seqs_equal!(byte_rows(lines@) == byte_rows(before).drop_last());
                assert_seqs_equal!(body@ == b@.skip(manifest_header().len() as int));
            }
            if lines.len() == 0 {
                return error(Vec::new(), concat(b"manifest holds no rows: ", path));
            }
            rows(&lines)
        },
    }
}

} // verus!
