use ckc_spec::ui as u;
use vstd::prelude::*;
use vstd::utf8::*;
verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub open spec fn same_at(s: Seq<char>, p: Seq<char>, i: int, j: nat) -> bool
    decreases p.len() - j,
{
    j >= p.len() || (u::copy_fold(s[i + j as int]) == p[j as int] && same_at(s, p, i, j + 1))
}

pub open spec fn matches(s: Seq<char>, p: Seq<char>, i: int, words: bool) -> bool {
    0 <= i && i + p.len() <= s.len() && same_at(s, p, i, 0) && (!words || ((i == 0
        || !u::ascii_word(s[i - 1])) && (i + p.len() == s.len() || !u::ascii_word(
        s[i + p.len() as int],
    ))))
}

pub proof fn same_at_equiv(s: Seq<char>, p: Seq<char>, i: int, j: nat)
    requires
        0 <= i,
        i + p.len() <= s.len(),
        j <= p.len(),
    ensures
        same_at(s, p, i, j) == (forall|k: int|
            j <= k < p.len() ==> u::copy_fold(#[trigger] s[i + k]) == p[k]),
    decreases p.len() - j,
{
    if j < p.len() {
        same_at_equiv(s, p, i, j + 1);
    }
}

pub proof fn matches_equiv(s: Seq<char>, p: Seq<char>, i: int, words: bool)
    ensures
        matches(s, p, i, words) == u::copy_match(s, p, i, words),
{
    if 0 <= i && i + p.len() <= s.len() {
        same_at_equiv(s, p, i, 0);
    }
}

pub open spec fn no_occurs(s: Seq<char>, p: Seq<char>, words: bool, i: nat) -> bool
    decreases s.len() - i,
{
    i >= s.len() || (!matches(s, p, i as int, words) && no_occurs(s, p, words, i + 1))
}

pub proof fn no_occurs_at(s: Seq<char>, p: Seq<char>, words: bool, i: nat, k: int)
    requires
        no_occurs(s, p, words, i),
        i <= k < s.len(),
    ensures
        !u::copy_match(s, p, k, words),
    decreases s.len() - i,
{
    if k == i {
        matches_equiv(s, p, k, words);
    } else {
        no_occurs_at(s, p, words, i + 1, k);
    }
}

pub open spec fn no_patterns(s: Seq<char>, ps: Seq<u::Bytes>, words: bool) -> bool
    decreases ps.len(),
{
    ps.len() == 0 || (no_occurs(s, u::chars(ps[0]), words, 0) && no_patterns(
        s,
        ps.drop_first(),
        words,
    ))
}

pub proof fn no_patterns_at(s: Seq<char>, ps: Seq<u::Bytes>, words: bool, i: int, j: int)
    requires
        no_patterns(s, ps, words),
        0 <= i < s.len(),
        0 <= j < ps.len(),
    ensures
        !u::copy_match(s, u::chars(ps[j]), i, words),
    decreases ps.len(),
{
    if j == 0 {
        no_occurs_at(s, u::chars(ps[0]), words, 0, i);
    } else {
        no_patterns_at(s, ps.drop_first(), words, i, j - 1);
    }
}

pub proof fn no_contains(s: Seq<char>, ps: Seq<u::Bytes>, words: bool)
    requires
        no_patterns(s, ps, words),
    ensures
        !u::copy_contains(s, ps, words),
{
    assert forall|i: int, j: int|
        0 <= i < s.len() && 0 <= j < ps.len() implies !#[trigger] u::copy_match(
        s,
        u::chars(ps[j]),
        i,
        words,
    ) by {
        no_patterns_at(s, ps, words, i, j);
    }
}

pub proof fn no_css(s: Seq<char>, ps: Seq<u::Bytes>)
    requires
        no_patterns(s, ps, false),
    ensures
        u::first_css(s, ps) is None,
    decreases ps.len(),
{
    if ps.len() > 0 {
        assert(seq![ps[0]].drop_first() =~= Seq::<u::Bytes>::empty());
        reveal_with_fuel(no_patterns, 2);
        assert(no_patterns(s, seq![ps[0]], false));
        no_contains(s, seq![ps[0]], false);
        no_css(s, ps.drop_first());
    }
}

pub proof fn no_word_at(s: Seq<char>, ps: Seq<u::Bytes>, i: nat)
    requires
        no_patterns(s, ps, true),
        i < s.len(),
    ensures
        u::word_pattern_at(s, ps, i) is None,
    decreases ps.len(),
{
    if ps.len() > 0 {
        no_occurs_at(s, u::chars(ps[0]), true, 0, i as int);
        no_word_at(s, ps.drop_first(), i);
    }
}

pub proof fn no_words(s: Seq<char>, ps: Seq<u::Bytes>, i: nat)
    requires
        no_patterns(s, ps, true),
    ensures
        u::first_copy_word(s, ps, i) is None,
    decreases s.len() - i,
{
    if i < s.len() {
        no_word_at(s, ps, i);
        no_words(s, ps, i + 1);
    }
}

pub open spec fn clean_chars(s: Seq<char>) -> bool
    decreases s.len(),
{
    s.len() == 0 || (!u::emoji(s[0]) && u::copy_admitted(s[0]) && clean_chars(s.drop_first()))
}

pub proof fn clean_chars_ok(s: Seq<char>)
    requires
        clean_chars(s),
    ensures
        u::first_copy_char(s, |c: char| u::emoji(c)) is None,
        u::first_copy_char(s, |c: char| !u::copy_admitted(c)) is None,
    decreases s.len(),
{
    if s.len() > 0 {
        clean_chars_ok(s.drop_first());
    }
}

pub open spec fn clean(s: Seq<char>) -> bool {
    clean_chars(s) && no_patterns(s, u::css_tokens(), false) && no_patterns(
        s,
        u::marketing_tokens(),
        true,
    ) && no_patterns(s, u::relative_tokens(), true) && u::first_exclamation(s, 0) is None
}

pub proof fn clean_ok(s: Seq<char>)
    requires
        clean(s),
    ensures
        u::copy_literal_ok(u::lit(s)),
{
    encode_utf8_valid_utf8(s);
    encode_utf8_decode_utf8(s);
    clean_chars_ok(s);
    no_css(s, u::css_tokens());
    no_words(s, u::marketing_tokens(), 0);
    no_words(s, u::relative_tokens(), 0);
}

} // verus!
