use vstd::prelude::*;

verus! {

// M5.4 seed: red stubs, one verification error each (contract.rs binds them).
pub fn align_resolve_impl(input: &[char], src: &[char], ace: &[char]) -> (r:
    ckc_spec::align::EResolve)
    ensures
        r@ == ckc_spec::align::resolve_outcome(input@, src@, ace@),
{
    assert(false);
    ckc_spec::align::EResolve::Err(Vec::new())
}

pub fn release_manifest_impl(
    head: &[u8],
    compiler: &[u8],
    lexicon: &[u8],
    staged: &Vec<ckc_spec::release::EMember>,
    profiles: &Vec<(Vec<u8>, Vec<u8>)>,
    urls: &Vec<(Vec<u8>, Vec<u8>)>,
    labels: &Vec<(Vec<u8>, Vec<u8>)>,
    tags: &Vec<ckc_spec::release::EMember>,
) -> (r: Vec<u8>)
    ensures
        r@ == ckc_spec::release::release_manifest(
            head@,
            compiler@,
            lexicon@,
            ckc_spec::release::members(staged@),
            ckc_spec::check::byte_pairs(profiles@),
            ckc_spec::check::byte_pairs(urls@),
            ckc_spec::check::byte_pairs(labels@),
            ckc_spec::release::members(tags@),
        ),
{
    assert(false);
    Vec::new()
}

} // verus!
