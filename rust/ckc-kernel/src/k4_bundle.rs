use crate::k4_bytes::{append, copy};
use ckc_spec::check::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::v1text::ascii;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub fn bundle(b: &EBundle) -> (r: Vec<u8>)
    ensures
        r@ == bundle_block(b@.docid, b@.ace, b@.cov, b@.pay, b@.cl),
{
    let mut r = copy(b"bundle v2 ");
    append(&mut r, &b.docid);
    append(&mut r, b"\nace ");
    append(&mut r, &b.ace);
    append(&mut r, b"\ncoverage ");
    append(&mut r, &b.cov);
    append(&mut r, b"\npayload ");
    append(&mut r, &b.pay);
    append(&mut r, b"\nclauses ");
    append(&mut r, &b.cl);
    r.push(0x0a);
    proof {
        reveal_byteslit(b"bundle v2 ");
        reveal_strlit("bundle v2 ");
        reveal_byteslit(b"\nace ");
        reveal_strlit("ace ");
        reveal_byteslit(b"\ncoverage ");
        reveal_strlit("coverage ");
        reveal_byteslit(b"\npayload ");
        reveal_strlit("payload ");
        reveal_byteslit(b"\nclauses ");
        reveal_strlit("clauses ");
        reveal(ascii);
    }
    r
}

pub fn row(b: &EBundle) -> (r: Vec<u8>)
    ensures
        r@ == manifest_row(b@),
{
    let mut r = copy(&b.docid);
    r.push(0x09);
    append(&mut r, &b.ace);
    r.push(0x09);
    append(&mut r, &b.cov);
    r.push(0x09);
    append(&mut r, &b.pay);
    r.push(0x09);
    append(&mut r, &b.cl);
    r.push(0x09);
    append(&mut r, &b.review);
    r.push(0x0a);
    r
}

pub fn header() -> (r: Vec<u8>)
    ensures
        r@ == manifest_header(),
{
    let h: &[u8] =
        b"# format: docid<TAB>ace_sha256<TAB>coverage_row_sha256<TAB>region_payload_sha256<TAB>semantic_clause_sha256<TAB>review_sha256\n# bundle v2; review_sha256 = sha256 of the labeled component-digest block; regenerate: python3 -P tools/goal.py review-manifest <id>; do not edit.\n";
    proof {
        reveal_byteslit(
            b"# format: docid<TAB>ace_sha256<TAB>coverage_row_sha256<TAB>region_payload_sha256<TAB>semantic_clause_sha256<TAB>review_sha256\n# bundle v2; review_sha256 = sha256 of the labeled component-digest block; regenerate: python3 -P tools/goal.py review-manifest <id>; do not edit.\n",
        );
        reveal_strlit(
            "# format: docid<TAB>ace_sha256<TAB>coverage_row_sha256<TAB>region_payload_sha256<TAB>semantic_clause_sha256<TAB>review_sha256\n# bundle v2; review_sha256 = sha256 of the labeled component-digest block; regenerate: python3 -P tools/goal.py review-manifest <id>; do not edit.\n",
        );
        reveal(ascii);
    }
    copy(h)
}

pub fn manifest(bs: &Vec<EBundle>) -> (r: Vec<u8>)
    ensures
        r@ == print_manifest(bundles(bs@)),
{
    let mut r = header();
    let mut i = 0usize;
    while i < bs.len()
        invariant
            i <= bs@.len(),
            r@ == manifest_header() + bs@.take(i as int).map_values(
                |b: EBundle| manifest_row(b@),
            ).flatten(),
        decreases bs.len() - i,
    {
        let next = row(&bs[i]);
        append(&mut r, &next);
        proof {
            bs@.lemma_map_take_succ(|b: EBundle| manifest_row(b@), i as int);
            bs@.take(i as int).map_values(|b: EBundle| manifest_row(b@)).lemma_flatten_push(
                manifest_row(bs@[i as int]@),
            );
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(bs@.take(i as int) == bs@);
        reveal(print_manifest);
        assert_seqs_equal!(bs@.map_values(|b: EBundle| manifest_row(b@)) == bundles(bs@).map_values(|b: Bundle| manifest_row(b)));
    }
    r
}

} // verus!
