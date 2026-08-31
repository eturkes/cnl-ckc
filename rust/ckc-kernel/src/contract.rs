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

} // verus!
