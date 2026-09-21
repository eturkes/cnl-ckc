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
    let (mut pay, src) = crate::release_rows::partition(staged, profiles, urls);
    let ghost original = ckc_spec::release::members(pay@);
    let mut i = 0usize;
    while i < tags.len()
        invariant
            i <= tags.len(),
            original == ckc_spec::release::payload(
                ckc_spec::release::members(staged@),
                ckc_spec::check::byte_pairs(profiles@),
            ),
            crate::release_rows::source_views(src@) == ckc_spec::release::sources(
                ckc_spec::release::members(staged@),
                ckc_spec::check::byte_pairs(profiles@),
                ckc_spec::check::byte_pairs(urls@),
            ),
            ckc_spec::release::members(pay@) == original + ckc_spec::release::members(tags@).take(
                i as int,
            ),
        decreases tags.len() - i,
    {
        let x = crate::release_rows::clone_member(&tags[i]);
        let ghost before = pay@;
        pay.push(x);
        proof {
            vstd::assert_seqs_equal!(ckc_spec::release::members(pay@) == ckc_spec::release::members(before).push(x@));
            vstd::assert_seqs_equal!(ckc_spec::release::members(tags@).take(i as int + 1) == ckc_spec::release::members(tags@).take(i as int).push(x@));
        }
        i += 1;
    }
    proof {
        vstd::assert_seqs_equal!(ckc_spec::release::members(tags@).take(i as int) == ckc_spec::release::members(tags@));
    }
    let sorted = crate::release_rows::sorted(&pay);
    let mut r = crate::release_rows::meta(head, compiler, lexicon);
    let ms = crate::release_rows::rows(&sorted);
    crate::k4_bytes::append(&mut r, &ms);
    let ss = crate::release_rows::source_text(&src);
    crate::k4_bytes::append(&mut r, &ss);
    let ls = crate::release_rows::label_text(labels);
    crate::k4_bytes::append(&mut r, &ls);
    r
}

} // verus!
