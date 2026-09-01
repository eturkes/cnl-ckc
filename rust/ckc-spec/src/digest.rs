use vstd::prelude::*;
// Spec-only use: rustc's non-verus pass erases spec fns and would warn.
#[allow(unused_imports)]
use crate::v1text::*;

verus! {

// Trusted spec: digest custody byte laws (contract m5u2 R3/R17). The
// kernel never computes sha256 and the verified crates carry no axiom for
// it — `--no-cheating` rejects uninterp/assume surfaces. The unverified
// `ckc` shell hashes with the vendored RustCrypto sha2 (its trust story =
// trust-audit + fixtures) and hands digest bytes across the boundary; the
// verified surface owns exactly WHICH bytes are hashed: `clause_line`
// (one canonical clause re-render including its LF — the K3 join law) and
// `semantic_lines` (the per-document retained-line stream), plus the
// lowercase-hex spelling of digest bytes.

pub open spec fn lhex_digit(d: int) -> u8 {
    if d < 10 {
        (0x30 + d) as u8
    } else {
        (0x61 + (d - 10)) as u8
    }
}

pub open spec fn hex_lower(bytes: Seq<u8>) -> Seq<u8>
    decreases bytes.len(),
{
    if bytes.len() == 0 {
        Seq::empty()
    } else {
        let b = bytes[0] as int;
        seq![lhex_digit(b / 16), lhex_digit(b % 16)] + hex_lower(bytes.drop_first())
    }
}

// Per-document semantic-digest input (review-manifest law): the
// schema-version line plus every bundle clause line, each with its LF;
// comments, directives and the document record drop.
pub open spec fn semantic_lines(d: DocFile) -> Seq<u8> {
    term_line(schema_version_term()) + all_clause_lines(d.bundles)
}

pub open spec fn all_clause_lines(bs: Seq<Bundle>) -> Seq<u8>
    decreases bs,
{
    if bs.len() == 0 {
        Seq::empty()
    } else {
        clauses_bytes(bs[0].clauses) + all_clause_lines(bs.drop_first())
    }
}

} // verus!
