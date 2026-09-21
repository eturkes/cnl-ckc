use crate::k5_sound_reflect as r;
use ckc_spec::ui as u;
use vstd::prelude::*;
verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub open spec fn codes(s: Seq<char>) -> Seq<nat> {
    s.map_values(|c: char| c as nat)
}

pub open spec fn patterns(ps: Seq<u::Bytes>) -> Seq<Seq<nat>> {
    ps.map_values(|p: u::Bytes| codes(u::chars(p)))
}

pub open spec fn word(c: nat) -> bool {
    (65 <= c <= 90) || (97 <= c <= 122) || (48 <= c <= 57) || c == 95
}

pub open spec fn fold(c: nat) -> nat {
    if 65 <= c <= 90 {
        c + 32
    } else {
        c
    }
}

pub open spec fn admitted(c: nat) -> bool {
    c < 128 || c == 167 || c == 183 || c == 8212 || c == 8805
}

pub open spec fn emoji(c: nat) -> bool {
    126975 < c < 129792 || 9727 < c < 10176 || 11007 < c < 11264 || c == 65039 || c == 8205
}

pub open spec fn same(s: Seq<nat>, p: Seq<nat>, i: int, j: nat) -> bool
    decreases p.len() - j,
{
    j >= p.len() || (fold(s[i + j as int]) == p[j as int] && same(s, p, i, j + 1))
}

pub open spec fn matches(s: Seq<nat>, p: Seq<nat>, i: int, w: bool) -> bool {
    0 <= i && i + p.len() <= s.len() && same(s, p, i, 0) && (!w || ((i == 0 || !word(s[i - 1])) && (
    i + p.len() == s.len() || !word(s[i + p.len() as int]))))
}

pub open spec fn scan(s: Seq<nat>, p: Seq<nat>, w: bool, a: nat, z: nat) -> bool
    decreases z - a,
{
    if a >= z {
        true
    } else if a + 1 == z {
        !matches(s, p, a as int, w)
    } else {
        let m = (a + z) / 2;
        scan(s, p, w, a, m) && scan(s, p, w, m, z)
    }
}

pub open spec fn no_patterns(s: Seq<nat>, ps: Seq<Seq<nat>>, w: bool) -> bool
    decreases ps.len(),
{
    ps.len() == 0 || (scan(s, ps[0], w, 0, s.len()) && no_patterns(s, ps.drop_first(), w))
}

pub open spec fn chars_clean(s: Seq<nat>, a: nat, z: nat) -> bool
    decreases z - a,
{
    if a >= z {
        true
    } else if a + 1 == z {
        admitted(s[a as int]) && !emoji(s[a as int])
    } else {
        let m = (a + z) / 2;
        chars_clean(s, a, m) && chars_clean(s, m, z)
    }
}

pub open spec fn calm(s: Seq<nat>, a: nat, z: nat) -> bool
    decreases z - a,
{
    if a >= z {
        true
    } else if a + 1 == z {
        a + 1 >= s.len() || !word(s[a as int]) || s[a as int] == 95 || s[a as int + 1] != 33
    } else {
        let m = (a + z) / 2;
        calm(s, a, m) && calm(s, m, z)
    }
}

pub open spec fn clean(
    s: Seq<nat>,
    css: Seq<Seq<nat>>,
    marketing: Seq<Seq<nat>>,
    relative: Seq<Seq<nat>>,
) -> bool {
    chars_clean(s, 0, s.len()) && no_patterns(s, css, false) && no_patterns(s, marketing, true)
        && no_patterns(s, relative, true) && calm(s, 0, s.len())
}

pub proof fn char_facts(c: char)
    ensures
        word(c as nat) == u::ascii_word(c),
        fold(c as nat) == u::copy_fold(c) as nat,
        admitted(c as nat) == u::copy_admitted(c),
        emoji(c as nat) == u::emoji(c),
{
}

pub proof fn same_bridge(s: Seq<char>, p: Seq<char>, i: int, j: nat)
    requires
        0 <= i,
        i + p.len() <= s.len(),
        j <= p.len(),
    ensures
        same(codes(s), codes(p), i, j) == r::same_at(s, p, i, j),
    decreases p.len() - j,
{
    if j < p.len() {
        char_facts(s[i + j as int]);
        let x = u::copy_fold(s[i + j as int]);
        let y = p[j as int];
        vstd::utf8::char_u32_cast(x, x as u32);
        vstd::utf8::char_u32_cast(y, y as u32);
        assert((x as nat == y as nat) == (x == y));
        same_bridge(s, p, i, j + 1);
    }
}

pub proof fn matches_bridge(s: Seq<char>, p: Seq<char>, i: int, w: bool)
    ensures
        matches(codes(s), codes(p), i, w) == r::matches(s, p, i, w),
{
    if 0 <= i && i + p.len() <= s.len() {
        same_bridge(s, p, i, 0);
        if i > 0 {
            char_facts(s[i - 1]);
        }
        if i + p.len() < s.len() {
            char_facts(s[i + p.len() as int]);
        }
    }
}

pub proof fn scan_at(s: Seq<nat>, p: Seq<nat>, w: bool, a: nat, z: nat, k: int)
    requires
        scan(s, p, w, a, z),
        a <= k < z,
    ensures
        !matches(s, p, k, w),
    decreases z - a,
{
    if a + 1 < z {
        let m = (a + z) / 2;
        if k < m {
            scan_at(s, p, w, a, m, k);
        } else {
            scan_at(s, p, w, m, z, k);
        }
    }
}

pub proof fn occurs_bridge(s: Seq<char>, p: Seq<char>, w: bool, a: nat)
    requires
        scan(codes(s), codes(p), w, 0, s.len()),
    ensures
        r::no_occurs(s, p, w, a),
    decreases s.len() - a,
{
    if a < s.len() {
        scan_at(codes(s), codes(p), w, 0, s.len(), a as int);
        matches_bridge(s, p, a as int, w);
        occurs_bridge(s, p, w, a + 1);
    }
}

pub proof fn patterns_bridge(s: Seq<char>, ps: Seq<u::Bytes>, w: bool)
    requires
        no_patterns(codes(s), patterns(ps), w),
    ensures
        r::no_patterns(s, ps, w),
    decreases ps.len(),
{
    if ps.len() > 0 {
        assert(patterns(ps).drop_first() =~= patterns(ps.drop_first()));
        occurs_bridge(s, u::chars(ps[0]), w, 0);
        patterns_bridge(s, ps.drop_first(), w);
    }
}

pub proof fn chars_at(s: Seq<nat>, a: nat, z: nat, k: int)
    requires
        chars_clean(s, a, z),
        a <= k < z,
    ensures
        admitted(s[k]),
        !emoji(s[k]),
    decreases z - a,
{
    if a + 1 < z {
        let m = (a + z) / 2;
        if k < m {
            chars_at(s, a, m, k);
        } else {
            chars_at(s, m, z, k);
        }
    }
}

pub proof fn chars_bridge(s: Seq<char>, a: nat)
    requires
        chars_clean(codes(s), 0, s.len()),
        a <= s.len(),
    ensures
        r::clean_chars(s.skip(a as int)),
    decreases s.len() - a,
{
    if a < s.len() {
        chars_at(codes(s), 0, s.len(), a as int);
        char_facts(s[a as int]);
        chars_bridge(s, a + 1);
        assert(s.skip(a as int).drop_first() =~= s.skip(a as int + 1));
    }
}

pub proof fn calm_at(s: Seq<nat>, a: nat, z: nat, k: int)
    requires
        calm(s, a, z),
        a <= k < z,
    ensures
        k + 1 >= s.len() || !word(s[k]) || s[k] == 95 || s[k + 1] != 33,
    decreases z - a,
{
    if a + 1 < z {
        let m = (a + z) / 2;
        if k < m {
            calm_at(s, a, m, k);
        } else {
            calm_at(s, m, z, k);
        }
    }
}

pub proof fn calm_bridge(s: Seq<char>, a: nat)
    requires
        calm(codes(s), 0, s.len()),
    ensures
        u::first_exclamation(s, a) is None,
    decreases s.len() - a,
{
    if a + 1 < s.len() {
        calm_at(codes(s), 0, s.len(), a as int);
        char_facts(s[a as int]);
        calm_bridge(s, a + 1);
    }
}

pub proof fn clean_bridge(s: Seq<char>)
    requires
        clean(
            codes(s),
            patterns(u::css_tokens()),
            patterns(u::marketing_tokens()),
            patterns(u::relative_tokens()),
        ),
    ensures
        u::copy_literal_ok(u::lit(s)),
{
    chars_bridge(s, 0);
    assert(s.skip(0) =~= s);
    patterns_bridge(s, u::css_tokens(), false);
    patterns_bridge(s, u::marketing_tokens(), true);
    patterns_bridge(s, u::relative_tokens(), true);
    calm_bridge(s, 0);
    r::clean_ok(s);
}

} // verus!
