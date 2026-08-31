use vstd::prelude::*;

verus! {

// UNINSPECTED impl+proofs: sole review = `cargo verus verify`.
// Seed stub: intentionally unverifiable failing body; the M5.1 prod
// teammate replaces it with the real validator + proofs.
pub(crate) fn align_check_impl(align: &[char], src: &[char], ace: &[char]) -> (r:
    ckc_spec::align::ECheck)
    ensures
        r@ == ckc_spec::align::align_outcome(align@, src@, ace@),
{
    ckc_spec::align::ECheck::Err(Vec::new())
}

} // verus!
