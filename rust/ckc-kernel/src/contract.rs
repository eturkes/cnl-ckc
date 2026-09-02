use vstd::prelude::*;

verus! {

// TRUSTED binding surface (spec-manifest-pinned): the kernel's public API
// with ensures binding every exec result to ckc-spec. Bodies delegate to
// uninspected impl modules; `cargo verus verify` enforces the binding.
pub fn align_check(align: &[char], src: &[char], ace: &[char]) -> (r: ckc_spec::align::ECheck)
    ensures
        r@ == ckc_spec::align::align_outcome(align@, src@, ace@),
{
    crate::align_impl::align_check_impl(align, src, ace)
}

// M5.2 K1: v1 clause-file acceptance (contract m5u2 R9/R10). The Reject
// offset = deterministic diagnostic (first divergence), pinned by fixtures,
// outside the theorem beyond its bound.
pub fn v1_check(bytes: &[u8]) -> (r: ckc_spec::v1text::EV1Verdict)
    ensures
        r.is_accept() <==> ckc_spec::v1text::accepts(bytes@),
        r matches ckc_spec::v1text::EV1Verdict::Reject { at } ==> at <= bytes@.len(),
{
    crate::v1_impl::v1_check_impl(bytes)
}

} // verus!
