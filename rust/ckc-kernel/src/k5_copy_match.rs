use crate::{k5_bytes as b, k5_codes as c};
use ckc_spec::ui as u;
use vstd::prelude::*;
use vstd::utf8::*;
verus! {

pub open spec fn ov(x: Option<Vec<u8>>) -> Option<Seq<u8>> {
    match x {
        Some(b) => Some(b@),
        None => None,
    }
}

pub open spec fn on(x: Option<usize>) -> Option<nat> {
    match x {
        Some(n) => Some(n as nat),
        None => None,
    }
}

pub fn word(x: u32) -> (yes: bool)
    requires
        is_scalar(x),
    ensures
        yes == u::ascii_word(x as char),
{
    (65 <= x && x <= 90) || (97 <= x && x <= 122) || (48 <= x && x <= 57) || x == 95
}

pub fn fold(x: u32) -> (y: u32)
    requires
        is_scalar(x),
    ensures
        is_scalar(y),
        (y as char) == u::copy_fold(x as char),
{
    if 65 <= x && x <= 90 {
        x + 32
    } else {
        x
    }
}

pub fn matches(s: &Vec<u32>, p: &Vec<u32>, i: usize, words: bool) -> (yes: bool)
    requires
        c::scalar_list(s@),
        c::scalar_list(p@),
    ensures
        yes == u::copy_match(c::cvs(s@), c::cvs(p@), i as int, words),
{
    if i > s.len() {
        return false;
    }
    if p.len() > s.len() - i {
        return false;
    }
    if words {
        if i > 0 && word(s[i - 1]) {
            return false;
        }
        if i + p.len() < s.len() && word(s[i + p.len()]) {
            return false;
        }
    }
    let mut j = 0;
    while j < p.len()
        invariant
            c::scalar_list(s@),
            c::scalar_list(p@),
            i <= s.len(),
            p.len() <= s.len() - i,
            j <= p.len(),
            !words || ((i == 0 || !u::ascii_word(c::cvs(s@)[i as int - 1])) && (i + p.len()
                == s.len() || !u::ascii_word(c::cvs(s@)[i as int + p.len() as int]))),
            forall|k: int|
                0 <= k < j ==> u::copy_fold(#[trigger] c::cvs(s@)[i as int + k]) == c::cvs(p@)[k],
        decreases p.len() - j,
    {
        let f = fold(s[i + j]);
        if f != p[j] {
            proof {
                assert((f as char) as u32 == f);
                assert((p@[j as int] as char) as u32 == p@[j as int]);
                if u::copy_match(c::cvs(s@), c::cvs(p@), i as int, words) {
                    assert(u::copy_fold(c::cvs(s@)[i as int + j as int]) == c::cvs(p@)[j as int]);
                }
            }
            return false;
        }
        j += 1;
    }
    true
}

pub fn contains(s: &Vec<u32>, p: &[u8], words: bool) -> (yes: bool)
    requires
        c::scalar_list(s@),
    ensures
        yes == u::copy_contains(c::cvs(s@), seq![p@], words),
{
    let pat = c::decode(p);
    let mut i = 0;
    while i < s.len()
        invariant
            c::scalar_list(s@),
            c::scalar_list(pat@),
            c::cvs(pat@) == u::chars(p@),
            i <= s.len(),
            forall|j: int| 0 <= j < i ==> !u::copy_match(c::cvs(s@), u::chars(p@), j, words),
        decreases s.len() - i,
    {
        if matches(s, &pat, i, words) {
            proof {
                assert(u::copy_match(c::cvs(s@), u::chars(seq![p@][0]), i as int, words));
            }
            return true;
        }
        i += 1;
    }
    proof {
        assert forall|j: int, k: int|
            0 <= j < s.len() && 0 <= k < seq![p@].len() implies !#[trigger] u::copy_match(
            c::cvs(s@),
            u::chars(seq![p@][k]),
            j,
            words,
        ) by {
            assert(k == 0);
        }
    }
    false
}

pub fn first_css(s: &Vec<u32>, ps: &Vec<Vec<u8>>) -> (out: Option<Vec<u8>>)
    requires
        c::scalar_list(s@),
    ensures
        ov(out) == u::first_css(c::cvs(s@), b::views(ps@)),
{
    let mut i = 0;
    proof {
        assert(b::views(ps@).skip(0) =~= b::views(ps@));
    }
    while i < ps.len()
        invariant
            c::scalar_list(s@),
            i <= ps.len(),
            u::first_css(c::cvs(s@), b::views(ps@).skip(i as int)) == u::first_css(
                c::cvs(s@),
                b::views(ps@),
            ),
        decreases ps.len() - i,
    {
        if contains(s, &ps[i], false) {
            return Some(vstd::slice::slice_to_vec(&ps[i]));
        }
        proof {
            assert(b::views(ps@).skip(i as int).drop_first() =~= b::views(ps@).skip(i as int + 1));
        }
        i += 1;
    }
    None
}

pub fn pattern_at(s: &Vec<u32>, ps: &Vec<Vec<u8>>, i: usize) -> (out: Option<usize>)
    requires
        c::scalar_list(s@),
        i <= s.len(),
    ensures
        on(out) == u::word_pattern_at(c::cvs(s@), b::views(ps@), i as nat),
        out matches Some(n) ==> n <= s.len() - i,
{
    let mut j = 0;
    proof {
        assert(b::views(ps@).skip(0) =~= b::views(ps@));
    }
    while j < ps.len()
        invariant
            c::scalar_list(s@),
            i <= s.len(),
            j <= ps.len(),
            u::word_pattern_at(c::cvs(s@), b::views(ps@).skip(j as int), i as nat)
                == u::word_pattern_at(c::cvs(s@), b::views(ps@), i as nat),
        decreases ps.len() - j,
    {
        let p = c::decode(&ps[j]);
        if matches(s, &p, i, true) {
            return Some(p.len());
        }
        proof {
            assert(b::views(ps@).skip(j as int).drop_first() =~= b::views(ps@).skip(j as int + 1));
        }
        j += 1;
    }
    None
}

pub fn first_word(s: &Vec<u32>, ps: &Vec<Vec<u8>>) -> (out: Option<Vec<u8>>)
    requires
        c::scalar_list(s@),
    ensures
        ov(out) == u::first_copy_word(c::cvs(s@), b::views(ps@), 0),
{
    let mut i = 0;
    while i < s.len()
        invariant
            c::scalar_list(s@),
            i <= s.len(),
            u::first_copy_word(c::cvs(s@), b::views(ps@), i as nat) == u::first_copy_word(
                c::cvs(s@),
                b::views(ps@),
                0,
            ),
        decreases s.len() - i,
    {
        match pattern_at(s, ps, i) {
            Some(n) => {
                let tail = &s[i..i + n];
                proof {
                    assert(c::scalar_list(tail@));
                    assert(c::cvs(tail@) =~= c::cvs(s@).subrange(i as int, i as int + n as int));
                }
                return Some(c::encode(tail));
            },
            None => {},
        }
        i += 1;
    }
    None
}

pub fn exclamation(s: &Vec<u32>) -> (out: Option<Vec<u8>>)
    requires
        c::scalar_list(s@),
    ensures
        ov(out) == u::first_exclamation(c::cvs(s@), 0),
{
    let mut i = 0;
    while i < s.len()
        invariant
            c::scalar_list(s@),
            i <= s.len(),
            u::first_exclamation(c::cvs(s@), i as nat) == u::first_exclamation(c::cvs(s@), 0),
        decreases s.len() - i,
    {
        if s.len() - i <= 1 {
            return None;
        }
        if word(s[i]) && s[i] != 95 && s[i + 1] == 33 {
            let part = &s[i..i + 2];
            proof {
                assert(c::scalar_list(part@));
                assert(c::cvs(part@) =~= c::cvs(s@).subrange(i as int, i as int + 2));
            }
            return Some(c::encode(part));
        }
        i += 1;
    }
    None
}

} // verus!
