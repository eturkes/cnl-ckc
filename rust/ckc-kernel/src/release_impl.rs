use crate::k4_bytes::{append, copy, eq};
use ckc_spec::release::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::v1text::ascii;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub fn align_resolve_impl(input: &[char], src: &[char], ace: &[char]) -> (r:
    ckc_spec::align::EResolve)
    ensures
        r@ == ckc_spec::align::resolve_outcome(input@, src@, ace@),
{
    crate::resolve_impl::resolve_impl(input, src, ace)
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

pub fn dist_digest_lines_impl(ms: &Vec<EMember>) -> (r: Vec<u8>)
    ensures
        r@ == digest_lines(members(ms@)),
{
    let sorted = crate::release_rows::sorted(ms);
    let mut r = Vec::new();
    let mut i = 0usize;
    let ghost f = |m: Member| m.sha + ascii("  "@) + m.path + lf();
    while i < sorted.len()
        invariant
            i <= sorted.len(),
            members(sorted@) == sort_members(members(ms@)),
            r@ == members(sorted@).take(i as int).map_values(f).flatten(),
            f == (|m: Member| m.sha + ascii("  "@) + m.path + lf()),
        decreases sorted.len() - i,
    {
        let m = &sorted[i];
        let mut row = copy(&m.sha);
        append(&mut row, b"  ");
        append(&mut row, &m.path);
        row.push(b'\n');
        proof {
            reveal_byteslit(b"  ");
            reveal_strlit("  ");
            reveal(ascii);
            assert(row@ == f(m@));
        }
        append(&mut r, &row);
        proof {
            members(sorted@).lemma_map_take_succ(f, i as int);
            members(sorted@).take(i as int).map_values(f).lemma_flatten_push(f(m@));
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(members(sorted@).take(i as int) == members(sorted@));
    }
    r
}

pub fn dist_tagmanifest_lines_impl(tags: &Vec<EMember>) -> (r: Vec<u8>)
    ensures
        r@ == tagmanifest_lines(members(tags@)),
{
    let mut kept = Vec::new();
    let mut i = 0usize;
    let ghost keep = |m: Member| m.path != ascii("tagmanifest-sha256.txt"@);
    while i < tags.len()
        invariant
            i <= tags.len(),
            members(kept@) == members(tags@).take(i as int).filter(keep),
            keep == (|m: Member| m.path != ascii("tagmanifest-sha256.txt"@)),
        decreases tags.len() - i,
    {
        let m = &tags[i];
        let is_self = eq(&m.path, b"tagmanifest-sha256.txt");
        proof {
            reveal_byteslit(b"tagmanifest-sha256.txt");
            reveal_strlit("tagmanifest-sha256.txt");
            reveal(ascii);
            assert_seqs_equal!(b"tagmanifest-sha256.txt"@ == ascii("tagmanifest-sha256.txt"@));
            assert(keep(m@) == !is_self);
            let s = members(tags@).take(i as int);
            assert_seqs_equal!(members(tags@).take(i as int + 1) == s.push(m@));
            s.lemma_filter_push(m@, keep);
        }
        if !is_self {
            let x = crate::release_rows::clone_member(m);
            let ghost before = kept@;
            kept.push(x);
            proof {
                assert_seqs_equal!(members(kept@) == members(before).push(x@));
            }
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(members(tags@).take(i as int) == members(tags@));
    }
    dist_digest_lines_impl(&kept)
}

} // verus!
