#![allow(unexpected_cfgs)]

use vstd::prelude::*;
#[cfg(verus_keep_ghost)]
use vstd::arithmetic::div_mod::{lemma_mod_multiples_vanish, lemma_small_mod};
#[cfg(verus_keep_ghost)]
use vstd::arithmetic::mul::lemma_mul_equality_converse;
#[cfg(verus_keep_ghost)]
use vstd::{assert_seqs_equal, assert_sets_equal};
use vstd::slice::{slice_subrange, slice_to_vec};

verus! {

fn ascii_digit(c: char) -> (r: bool)
    ensures r == ckc_spec::align::is_ascii_digit(c),
{
    c >= '0' && c <= '9'
}

fn all_ascii_digits(s: &[char]) -> (r: bool)
    ensures r == ckc_spec::align::all_ascii_digits(s@),
{
    let mut i = 0usize;
    while i < s.len()
        invariant
            i <= s@.len(),
            forall|j: int| 0 <= j < i ==> ckc_spec::align::is_ascii_digit(s@[j]),
        decreases s.len() - i,
    {
        if !ascii_digit(s[i]) {
            return false;
        }
        i += 1;
    }
    true
}

fn canonical_decimal(s: &[char]) -> (r: bool)
    ensures r == ckc_spec::align::is_canonical_decimal(s@),
{
    if s.len() == 0 {
        false
    } else if s.len() > 1 && s[0] == '0' {
        false
    } else {
        all_ascii_digits(s)
    }
}

fn digit_value(c: char) -> (d: usize)
    requires ckc_spec::align::is_ascii_digit(c),
    ensures d as int == ckc_spec::align::digit_value(c), d <= 9,
{
    if c == '0' { 0 }
    else if c == '1' { 1 }
    else if c == '2' { 2 }
    else if c == '3' { 3 }
    else if c == '4' { 4 }
    else if c == '5' { 5 }
    else if c == '6' { 6 }
    else if c == '7' { 7 }
    else if c == '8' { 8 }
    else { 9 }
}

proof fn prefix_push<A>(s: Seq<A>, i: int)
    requires 0 <= i < s.len(),
    ensures s.subrange(0, i + 1) == s.subrange(0, i).push(s[i]),
{
    assert_seqs_equal!(s.subrange(0, i + 1) == s.subrange(0, i).push(s[i]));
}

proof fn dec_push(s: Seq<char>, c: char)
    ensures ckc_spec::align::dec_value(s.push(c))
        == ckc_spec::align::dec_value(s) * 10 + ckc_spec::align::digit_value(c),
{
    assert_seqs_equal!(s.push(c).drop_last() == s);
    assert(s.push(c).last() == c);
    reveal_with_fuel(ckc_spec::align::dec_value, 2);
}

proof fn digit_bounds(c: char)
    requires ckc_spec::align::is_ascii_digit(c),
    ensures
        0 <= ckc_spec::align::digit_value(c),
        ckc_spec::align::digit_value(c) <= 9,
{
}

proof fn all_digits_drop_last(s: Seq<char>)
    requires
        s.len() > 0,
        ckc_spec::align::all_ascii_digits(s),
    ensures ckc_spec::align::all_ascii_digits(s.drop_last()),
{
    assert forall|i: int| #![auto] 0 <= i < s.drop_last().len()
        ==> ckc_spec::align::is_ascii_digit(s.drop_last()[i]) by {
        if 0 <= i < s.drop_last().len() {
            assert(i < s.len());
            assert(s.drop_last()[i] == s[i]);
        }
    }
}

proof fn canonical_drop_last(s: Seq<char>)
    requires
        s.len() > 1,
        ckc_spec::align::is_canonical_decimal(s),
    ensures ckc_spec::align::is_canonical_decimal(s.drop_last()),
{
    all_digits_drop_last(s);
    assert(s.drop_last().len() == s.len() - 1);
    if s.drop_last().len() > 1 {
        assert(s.drop_last()[0] == s[0]);
    }
}

proof fn dec_nonnegative(s: Seq<char>)
    requires ckc_spec::align::all_ascii_digits(s),
    ensures ckc_spec::align::dec_value(s) >= 0,
    decreases s.len(),
{
    reveal_with_fuel(ckc_spec::align::dec_value, 2);
    if s.len() > 0 {
        all_digits_drop_last(s);
        dec_nonnegative(s.drop_last());
        digit_bounds(s.last());
    }
}

proof fn dec_positive(s: Seq<char>)
    requires
        ckc_spec::align::is_canonical_decimal(s),
        s[0] != '0',
    ensures ckc_spec::align::dec_value(s) > 0,
    decreases s.len(),
{
    reveal_with_fuel(ckc_spec::align::dec_value, 2);
    digit_bounds(s.last());
    if s.len() == 1 {
        assert(s.last() == s[0]);
        assert(ckc_spec::align::digit_value(s[0]) > 0);
    } else {
        canonical_drop_last(s);
        assert(s.drop_last()[0] == s[0]);
        dec_positive(s.drop_last());
    }
}

proof fn canonical_injective(a: Seq<char>, b: Seq<char>)
    requires
        ckc_spec::align::is_canonical_decimal(a),
        ckc_spec::align::is_canonical_decimal(b),
        ckc_spec::align::dec_value(a) == ckc_spec::align::dec_value(b),
    ensures a == b,
    decreases a.len() + b.len(),
{
    assert(a.len() > 0 && b.len() > 0);
    let ap = a.drop_last();
    let bp = b.drop_last();
    let da = ckc_spec::align::digit_value(a.last());
    let db = ckc_spec::align::digit_value(b.last());
    all_digits_drop_last(a);
    all_digits_drop_last(b);
    dec_nonnegative(ap);
    dec_nonnegative(bp);
    digit_bounds(a.last());
    digit_bounds(b.last());
    reveal_with_fuel(ckc_spec::align::dec_value, 2);
    let va = ckc_spec::align::dec_value(ap);
    let vb = ckc_spec::align::dec_value(bp);
    assert(ckc_spec::align::dec_value(a) == va * 10 + da);
    assert(ckc_spec::align::dec_value(b) == vb * 10 + db);
    lemma_mod_multiples_vanish(va, da, 10);
    lemma_mod_multiples_vanish(vb, db, 10);
    lemma_small_mod(da as nat, 10);
    lemma_small_mod(db as nat, 10);
    assert(da % 10 == da);
    assert(db % 10 == db);
    assert(va * 10 == 10 * va) by (nonlinear_arith);
    assert(vb * 10 == 10 * vb) by (nonlinear_arith);
    assert((va * 10 + da) % 10 == da);
    assert((vb * 10 + db) % 10 == db);
    assert(va * 10 + da == vb * 10 + db);
    assert(da == db);
    assert(va * 10 + da == vb * 10 + da);
    assert(va * 10 == vb * 10);
    assert(10 * va == 10 * vb);
    lemma_mul_equality_converse(10, va, vb);
    if a.len() == 1 {
        if b.len() > 1 {
            canonical_drop_last(b);
            assert(bp[0] == b[0]);
            dec_positive(bp);
            assert(false);
        }
        assert(a == seq![a.last()]);
        assert(b == seq![b.last()]);
        assert(a.last() == b.last());
    } else if b.len() == 1 {
        canonical_drop_last(a);
        assert(ap[0] == a[0]);
        dec_positive(ap);
        assert(false);
    } else {
        canonical_drop_last(a);
        canonical_drop_last(b);
        canonical_injective(ap, bp);
        assert(a == ap.push(a.last()));
        assert(b == bp.push(b.last()));
        assert(a.last() == b.last());
    }
}

pub enum EDecimalCmp {
    Less,
    Equal,
    Greater,
}

pub open spec fn decimal_cmp_contract(c: EDecimalCmp, a: Seq<char>, b: Seq<char>) -> bool {
    match c {
        EDecimalCmp::Less => ckc_spec::align::dec_value(a) < ckc_spec::align::dec_value(b),
        EDecimalCmp::Equal => ckc_spec::align::dec_value(a) == ckc_spec::align::dec_value(b),
        EDecimalCmp::Greater => ckc_spec::align::dec_value(a) > ckc_spec::align::dec_value(b),
    }
}

fn decimal_cmp(a: &[char], b: &[char]) -> (c: EDecimalCmp)
    requires
        ckc_spec::align::all_ascii_digits(a@),
        ckc_spec::align::all_ascii_digits(b@),
    ensures decimal_cmp_contract(c, a@, b@),
    decreases a.len() + b.len(),
{
    if a.len() == 0 && b.len() == 0 {
        return EDecimalCmp::Equal;
    }
    let ap = if a.len() == 0 { a } else { slice_subrange(a, 0, a.len() - 1) };
    let bp = if b.len() == 0 { b } else { slice_subrange(b, 0, b.len() - 1) };
    let da = if a.len() == 0 { 0 } else {
        assert(ckc_spec::align::is_ascii_digit(a@[a@.len() - 1]));
        digit_value(a[a.len() - 1])
    };
    let db = if b.len() == 0 { 0 } else {
        assert(ckc_spec::align::is_ascii_digit(b@[b@.len() - 1]));
        digit_value(b[b.len() - 1])
    };
    proof {
        if a@.len() == 0 {
            reveal_with_fuel(ckc_spec::align::dec_value, 2);
        } else {
            assert_seqs_equal!(ap@ == a@.drop_last());
            all_digits_drop_last(a@);
            reveal_with_fuel(ckc_spec::align::dec_value, 2);
        }
        if b@.len() == 0 {
            reveal_with_fuel(ckc_spec::align::dec_value, 2);
        } else {
            assert_seqs_equal!(bp@ == b@.drop_last());
            all_digits_drop_last(b@);
            reveal_with_fuel(ckc_spec::align::dec_value, 2);
        }
        assert(ckc_spec::align::dec_value(a@)
            == ckc_spec::align::dec_value(ap@) * 10 + da as int);
        assert(ckc_spec::align::dec_value(b@)
            == ckc_spec::align::dec_value(bp@) * 10 + db as int);
    }
    match decimal_cmp(ap, bp) {
        EDecimalCmp::Less => {
            assert(ckc_spec::align::dec_value(ap@) + 1 <= ckc_spec::align::dec_value(bp@));
            assert(da <= 9 && db <= 9);
            EDecimalCmp::Less
        },
        EDecimalCmp::Greater => {
            assert(ckc_spec::align::dec_value(bp@) + 1 <= ckc_spec::align::dec_value(ap@));
            assert(da <= 9 && db <= 9);
            EDecimalCmp::Greater
        },
        EDecimalCmp::Equal => {
            if da < db { EDecimalCmp::Less }
            else if da > db { EDecimalCmp::Greater }
            else { EDecimalCmp::Equal }
        },
    }
}

fn decimal_le(a: &[char], b: &[char]) -> (r: bool)
    requires
        ckc_spec::align::all_ascii_digits(a@),
        ckc_spec::align::all_ascii_digits(b@),
    ensures r == (ckc_spec::align::dec_value(a@) <= ckc_spec::align::dec_value(b@)),
{
    match decimal_cmp(a, b) {
        EDecimalCmp::Greater => false,
        _ => true,
    }
}

fn bounded_decimal(s: &[char], bound: usize) -> (r: Option<usize>)
    requires ckc_spec::align::is_canonical_decimal(s@),
    ensures
        match r {
            Some(v) => v <= bound && v as int == ckc_spec::align::dec_value(s@),
            None => ckc_spec::align::dec_value(s@) > bound as int,
        },
{
    assert(ckc_spec::align::all_ascii_digits(s@));
    let mut i = 0usize;
    let mut value = 0usize;
    let mut over = false;
    while i < s.len()
        invariant
            i <= s@.len(),
            ckc_spec::align::all_ascii_digits(s@),
            value <= bound,
            !over ==> value as int == ckc_spec::align::dec_value(s@.subrange(0, i as int)),
            over ==> ckc_spec::align::dec_value(s@.subrange(0, i as int)) > bound as int,
        decreases s.len() - i,
    {
        assert(ckc_spec::align::is_ascii_digit(s@[i as int]));
        let d = digit_value(s[i]);
        proof {
            prefix_push(s@, i as int);
            dec_push(s@.subrange(0, i as int), s@[i as int]);
        }
        if over {
            assert(ckc_spec::align::digit_value(s@[i as int]) >= 0);
        } else {
            match value.checked_mul(10) {
                None => {
                    over = true;
                },
                Some(m) => {
                    match m.checked_add(d) {
                        None => {
                            over = true;
                        },
                        Some(next) => {
                            if next > bound {
                                over = true;
                            } else {
                                value = next;
                            }
                        },
                    }
                },
            }
        }
        i += 1;
    }
    proof { assert_seqs_equal!(s@.subrange(0, s@.len() as int) == s@); }
    if over { None } else { Some(value) }
}

fn seq_equal(a: &[char], b: &[char]) -> (r: bool)
    ensures r == (a@ == b@),
{
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0usize;
    while i < a.len()
        invariant
            a@.len() == b@.len(),
            i <= a@.len(),
            forall|j: int| 0 <= j < i ==> a@[j] == b@[j],
        decreases a.len() - i,
    {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    proof { assert_seqs_equal!(a@ == b@); }
    true
}

proof fn side_names_distinct()
    ensures "src"@ != "ace"@,
{
    reveal_strlit("src");
    reveal_strlit("ace");
    assert("src"@.len() == 3);
    assert("ace"@.len() == 3);
    assert("src"@[0] == 's');
    assert("ace"@[0] == 'a');
}

fn is_src(s: &[char]) -> (r: bool)
    ensures r == (s@ == "src"@),
{
    let mut name = Vec::new();
    name.push('s');
    name.push('r');
    name.push('c');
    proof {
        reveal_strlit("src");
        assert_seqs_equal!(name@ == "src"@);
    }
    seq_equal(s, name.as_slice())
}

fn is_ace(s: &[char]) -> (r: bool)
    ensures r == (s@ == "ace"@),
{
    let mut name = Vec::new();
    name.push('a');
    name.push('c');
    name.push('e');
    proof {
        reveal_strlit("ace");
        assert_seqs_equal!(name@ == "ace"@);
    }
    seq_equal(s, name.as_slice())
}

pub enum EViolation {
    MissingTrailingNewline,
    EmptyFile,
    FieldCount { row: usize },
    GroupCanonical { row: usize },
    StartCanonical { row: usize },
    EmptySpan { row: usize },
    SideVocab { row: usize },
    OutOfRange { row: usize },
    SpanMismatch { row: usize },
    NotBothSided,
    OverlapSrc,
    OverlapAce,
}

impl View for EViolation {
    type V = ckc_spec::align::Violation;

    open spec fn view(&self) -> ckc_spec::align::Violation {
        match self {
            EViolation::MissingTrailingNewline => ckc_spec::align::Violation::MissingTrailingNewline,
            EViolation::EmptyFile => ckc_spec::align::Violation::EmptyFile,
            EViolation::FieldCount { row } => ckc_spec::align::Violation::FieldCount { row: *row as int },
            EViolation::GroupCanonical { row } => ckc_spec::align::Violation::GroupCanonical { row: *row as int },
            EViolation::StartCanonical { row } => ckc_spec::align::Violation::StartCanonical { row: *row as int },
            EViolation::EmptySpan { row } => ckc_spec::align::Violation::EmptySpan { row: *row as int },
            EViolation::SideVocab { row } => ckc_spec::align::Violation::SideVocab { row: *row as int },
            EViolation::OutOfRange { row } => ckc_spec::align::Violation::OutOfRange { row: *row as int },
            EViolation::SpanMismatch { row } => ckc_spec::align::Violation::SpanMismatch { row: *row as int },
            EViolation::NotBothSided => ckc_spec::align::Violation::NotBothSided,
            EViolation::OverlapSrc => ckc_spec::align::Violation::OverlapSrc,
            EViolation::OverlapAce => ckc_spec::align::Violation::OverlapAce,
        }
    }
}

pub open spec fn option_violation_view(v: Option<EViolation>) -> Option<ckc_spec::align::Violation> {
    match v {
        Some(x) => Some(x@),
        None => None,
    }
}

pub struct ERawSpan {
    pub start: usize,
    pub end: usize,
    pub group: Vec<char>,
}

impl View for ERawSpan {
    type V = ckc_spec::align::RawSpan;

    open spec fn view(&self) -> ckc_spec::align::RawSpan {
        ckc_spec::align::RawSpan {
            start: self.start as int,
            end: self.end as int,
            group: ckc_spec::align::dec_value(self.group@),
        }
    }
}

pub open spec fn raw_view(s: Seq<ERawSpan>) -> Seq<ckc_spec::align::RawSpan>
    decreases s.len(),
{
    if s.len() == 0 {
        Seq::empty()
    } else {
        seq![s[0]@] + raw_view(s.drop_first())
    }
}

pub open spec fn raw_groups_canonical(s: Seq<ERawSpan>) -> bool {
    forall|i: int| #![auto] 0 <= i < s.len()
        ==> ckc_spec::align::is_canonical_decimal(s[i].group@)
}

pub enum ERows {
    Err(EViolation),
    Ok { src: Vec<ERawSpan>, ace: Vec<ERawSpan> },
}

pub open spec fn rows_contract(
    r: ERows,
    rows: Seq<Seq<char>>,
    n: int,
    src_text: Seq<char>,
    ace_text: Seq<char>,
) -> bool {
    match r {
        ERows::Err(v) => ckc_spec::align::rows_violation(rows, n, src_text, ace_text) == Some(v@),
        ERows::Ok { src, ace } => {
            &&& ckc_spec::align::rows_violation(rows, n, src_text, ace_text) is None
            &&& raw_view(src@) == ckc_spec::align::side_spans(rows, "src"@)
            &&& raw_view(ace@) == ckc_spec::align::side_spans(rows, "ace"@)
            &&& raw_groups_canonical(src@)
            &&& raw_groups_canonical(ace@)
        },
    }
}

pub open spec fn seqs_view(s: Seq<Vec<char>>) -> Seq<Seq<char>>
    decreases s.len(),
{
    if s.len() == 0 {
        Seq::empty()
    } else {
        seq![s[0]@] + seqs_view(s.drop_first())
    }
}

proof fn remove_zero<A>(s: Seq<A>)
    requires s.len() > 0,
    ensures s.remove(0) == s.drop_first(),
{
    assert_seqs_equal!(s.remove(0) == s.drop_first());
}

proof fn insert_zero<A>(s: Seq<A>, x: A)
    ensures s.insert(0, x) == seq![x] + s,
{
    assert_seqs_equal!(s.insert(0, x) == seq![x] + s);
}

proof fn seqs_view_concat(a: Seq<Vec<char>>, b: Seq<Vec<char>>)
    ensures seqs_view(a + b) == seqs_view(a) + seqs_view(b),
    decreases a.len(),
{
    reveal_with_fuel(seqs_view, 2);
    if a.len() > 0 {
        assert_seqs_equal!((a + b).drop_first() == a.drop_first() + b);
        seqs_view_concat(a.drop_first(), b);
    } else {
        assert(a == Seq::<Vec<char>>::empty());
    }
}

proof fn seqs_view_drop_first(s: Seq<Vec<char>>)
    requires s.len() > 0,
    ensures seqs_view(s.drop_first()) == seqs_view(s).drop_first(),
{
}

proof fn seqs_view_len(s: Seq<Vec<char>>)
    ensures seqs_view(s).len() == s.len(),
    decreases s.len(),
{
    reveal_with_fuel(seqs_view, 2);
    if s.len() > 0 {
        seqs_view_len(s.drop_first());
    }
}

proof fn seqs_view_index(s: Seq<Vec<char>>, i: int)
    requires
        0 <= i < s.len(),
        i < seqs_view(s).len(),
    ensures seqs_view(s)[i] == s[i]@,
    decreases i,
{
    reveal_with_fuel(seqs_view, 2);
    if i > 0 {
        seqs_view_len(s.drop_first());
        seqs_view_index(s.drop_first(), i - 1);
        assert(s.drop_first()[i - 1] == s[i]);
    }
}

proof fn raw_view_concat(a: Seq<ERawSpan>, b: Seq<ERawSpan>)
    ensures raw_view(a + b) == raw_view(a) + raw_view(b),
    decreases a.len(),
{
    reveal_with_fuel(raw_view, 2);
    if a.len() > 0 {
        assert_seqs_equal!((a + b).drop_first() == a.drop_first() + b);
        raw_view_concat(a.drop_first(), b);
    } else {
        assert(a == Seq::<ERawSpan>::empty());
    }
}

proof fn raw_view_prepend(s: Seq<ERawSpan>, x: ERawSpan)
    ensures raw_view(s.insert(0, x)) == seq![x@] + raw_view(s),
{
    insert_zero(s, x);
    raw_view_concat(seq![x], s);
    reveal_with_fuel(raw_view, 2);
}

proof fn raw_canonical_prepend(s: Seq<ERawSpan>, x: ERawSpan)
    requires
        raw_groups_canonical(s),
        ckc_spec::align::is_canonical_decimal(x.group@),
    ensures raw_groups_canonical(s.insert(0, x)),
{
    s.insert_ensures(0, x);
    assert(forall|i: int| #![auto] 0 <= i < s.insert(0, x).len()
        ==> ckc_spec::align::is_canonical_decimal(s.insert(0, x)[i].group@));
}

proof fn raw_view_len(s: Seq<ERawSpan>)
    ensures raw_view(s).len() == s.len(),
    decreases s.len(),
{
    reveal_with_fuel(raw_view, 2);
    if s.len() > 0 {
        raw_view_len(s.drop_first());
    }
}

proof fn raw_view_index(s: Seq<ERawSpan>, i: int)
    requires
        0 <= i < s.len(),
        i < raw_view(s).len(),
    ensures raw_view(s)[i] == s[i]@,
    decreases i,
{
    reveal_with_fuel(raw_view, 2);
    if i > 0 {
        raw_view_len(s.drop_first());
        raw_view_index(s.drop_first(), i - 1);
        assert(s.drop_first()[i - 1] == s[i]);
    }
}

proof fn raw_view_drop_first(s: Seq<ERawSpan>)
    requires s.len() > 0,
    ensures raw_view(s.drop_first()) == raw_view(s).drop_first(),
{
    reveal_with_fuel(raw_view, 2);
}

proof fn raw_canonical_drop_first(s: Seq<ERawSpan>)
    requires
        s.len() > 0,
        raw_groups_canonical(s),
    ensures
        raw_groups_canonical(s.drop_first()),
        ckc_spec::align::is_canonical_decimal(s[0].group@),
{
    assert forall|i: int| #![auto] 0 <= i < s.drop_first().len()
        ==> ckc_spec::align::is_canonical_decimal(s.drop_first()[i].group@) by {
        if 0 <= i < s.drop_first().len() {
            assert(s.drop_first()[i] == s[i + 1]);
        }
    }
}

proof fn split_nonempty(s: Seq<char>, sep: char)
    ensures ckc_spec::align::split_at_seps(s, sep).len() > 0,
    decreases s.len(),
{
    if s.len() > 0 {
        split_nonempty(s.drop_first(), sep);
    }
}

fn split_at_seps(s: &[char], sep: char) -> (out: Vec<Vec<char>>)
    ensures seqs_view(out@) == ckc_spec::align::split_at_seps(s@, sep),
    decreases s.len(),
{
    if s.len() == 0 {
        let mut out = Vec::new();
        out.push(Vec::new());
        proof { reveal_with_fuel(seqs_view, 2); }
        out
    } else {
        let tail = slice_subrange(s, 1, s.len());
        let mut rest = split_at_seps(tail, sep);
        proof { split_nonempty(tail@, sep); }
        let ghost before = rest@;
        let mut out = Vec::new();
        if s[0] == sep {
            out.push(Vec::new());
            proof { reveal_with_fuel(seqs_view, 2); }
            assert(seqs_view(out@) == seq![Seq::<char>::empty()]);
            assert(rest@ == before);
        } else {
            let mut first = rest.remove(0);
            proof { remove_zero(before); }
            assert(rest@ == before.drop_first());
            assert(first@ == before[0]@);
            let ghost first_before = first@;
            first.insert(0, s[0]);
            proof { insert_zero(first_before, s@[0]); }
            assert(first@ == seq![s@[0]] + before[0]@);
            out.push(first);
            proof { reveal_with_fuel(seqs_view, 2); }
            assert(seqs_view(out@) == seq![seq![s@[0]] + before[0]@]);
        }
        let ghost head = out@;
        let ghost remaining = rest@;
        out.append(&mut rest);
        proof { seqs_view_concat(head, remaining); }
        assert(seqs_view(out@) == seqs_view(head) + seqs_view(remaining));
        proof { seqs_view_drop_first(before); }
        assert(seqs_view(out@) == if s@[0] == sep {
            seq![Seq::<char>::empty()] + seqs_view(before)
        } else {
            seq![seq![s@[0]] + seqs_view(before)[0]] + seqs_view(before).drop_first()
        });
        out
    }
}

fn row_violation(
    row: &[char],
    n: usize,
    src: &[char],
    ace: &[char],
) -> (r: Option<EViolation>)
    ensures option_violation_view(r) == ckc_spec::align::row_violation(row@, n as int, src@, ace@),
{
    let fields = split_at_seps(row, '\t');
    proof { seqs_view_len(fields@); }
    if fields.len() != 4 {
        return Some(EViolation::FieldCount { row: n });
    }
    proof {
        seqs_view_index(fields@, 0);
        seqs_view_index(fields@, 1);
        seqs_view_index(fields@, 2);
        seqs_view_index(fields@, 3);
    }
    if !canonical_decimal(fields[0].as_slice()) {
        return Some(EViolation::GroupCanonical { row: n });
    }
    if !canonical_decimal(fields[2].as_slice()) {
        return Some(EViolation::StartCanonical { row: n });
    }
    if fields[3].len() == 0 {
        return Some(EViolation::EmptySpan { row: n });
    }
    let src_side = is_src(fields[1].as_slice());
    let ace_side = is_ace(fields[1].as_slice());
    if !src_side && !ace_side {
        return Some(EViolation::SideVocab { row: n });
    }
    let text = if src_side { src } else { ace };
    let start = match bounded_decimal(fields[2].as_slice(), text.len()) {
        None => return Some(EViolation::OutOfRange { row: n }),
        Some(v) => v,
    };
    if fields[3].len() > text.len() - start {
        return Some(EViolation::OutOfRange { row: n });
    }
    let end = start + fields[3].len();
    let got = slice_subrange(text, start, end);
    if !seq_equal(got, fields[3].as_slice()) {
        return Some(EViolation::SpanMismatch { row: n });
    }
    None
}

fn rows_scan(
    rows: &[Vec<char>],
    n: usize,
    src_text: &[char],
    ace_text: &[char],
) -> (r: ERows)
    requires
        n >= 1,
        n as int - 1 + rows@.len() <= usize::MAX as int,
    ensures rows_contract(r, seqs_view(rows@), n as int, src_text@, ace_text@),
    decreases rows.len(),
{
    proof {
        reveal_strlit("src");
        reveal_strlit("ace");
        side_names_distinct();
        seqs_view_len(rows@);
    }
    if rows.len() == 0 {
        let result = ERows::Ok { src: Vec::new(), ace: Vec::new() };
        assert(rows_contract(result, seqs_view(rows@), n as int, src_text@, ace_text@));
        return result;
    }
    proof { seqs_view_index(rows@, 0); }
    match row_violation(rows[0].as_slice(), n, src_text, ace_text) {
        Some(v) => {
            let result = ERows::Err(v);
            assert(rows_contract(result, seqs_view(rows@), n as int, src_text@, ace_text@));
            result
        },
        None => {
            let tail = slice_subrange(rows, 1, rows.len());
            proof {
                assert_seqs_equal!(tail@ == rows@.drop_first());
                seqs_view_drop_first(rows@);
            }
            let rest = if tail.len() == 0 {
                ERows::Ok { src: Vec::new(), ace: Vec::new() }
            } else {
                assert(n < usize::MAX);
                rows_scan(tail, n + 1, src_text, ace_text)
            };
            assert(rows_contract(rest, seqs_view(tail@), n as int + 1, src_text@, ace_text@));
            match rest {
                ERows::Err(v) => {
                    let result = ERows::Err(v);
                    assert(rows_contract(result, seqs_view(rows@), n as int, src_text@, ace_text@));
                    result
                },
                ERows::Ok { mut src, mut ace } => {
                    assert(raw_view(src@) == ckc_spec::align::side_spans(seqs_view(tail@), "src"@));
                    assert(raw_view(ace@) == ckc_spec::align::side_spans(seqs_view(tail@), "ace"@));
                    let fields = split_at_seps(rows[0].as_slice(), '\t');
                    proof {
                        seqs_view_len(fields@);
                        seqs_view_index(fields@, 0);
                        seqs_view_index(fields@, 1);
                        seqs_view_index(fields@, 2);
                        seqs_view_index(fields@, 3);
                    }
                    let src_side = is_src(fields[1].as_slice());
                    if src_side {
                        assert(fields[1]@ == "src"@);
                        assert(fields[1]@ != "ace"@);
                    } else {
                        assert(fields[1]@ == "ace"@);
                    }
                    let text = if src_side { src_text } else { ace_text };
                    let start = match bounded_decimal(fields[2].as_slice(), text.len()) {
                        Some(v) => v,
                        None => { assert(false); 0 },
                    };
                    let end = start + fields[3].len();
                    let group = slice_to_vec(fields[0].as_slice());
                    let span = ERawSpan { start, end, group };
                    let ghost span_view = span@;
                    if src_side {
                        let ghost before = src@;
                        proof {
                            raw_view_prepend(before, span);
                            raw_canonical_prepend(before, span);
                        }
                        src.insert(0, span);
                        assert(raw_view(src@) == seq![span_view] + raw_view(before));
                        assert(raw_view(ace@) == ckc_spec::align::side_spans(seqs_view(tail@), "ace"@));
                        assert(ckc_spec::align::side_spans(seqs_view(rows@), "ace"@)
                            == ckc_spec::align::side_spans(seqs_view(tail@), "ace"@));
                    } else {
                        let ghost before = ace@;
                        proof {
                            raw_view_prepend(before, span);
                            raw_canonical_prepend(before, span);
                        }
                        ace.insert(0, span);
                        assert(raw_view(ace@) == seq![span_view] + raw_view(before));
                        assert(raw_view(ace@) == seq![span_view]
                            + ckc_spec::align::side_spans(seqs_view(tail@), "ace"@));
                        proof { reveal_with_fuel(ckc_spec::align::side_spans, 2); }
                        assert(ckc_spec::align::side_spans(seqs_view(rows@), "ace"@)
                            == seq![span_view]
                                + ckc_spec::align::side_spans(seqs_view(tail@), "ace"@));
                    }
                    proof { reveal_with_fuel(ckc_spec::align::side_spans, 2); }
                    assert(ckc_spec::align::rows_violation(
                        seqs_view(rows@), n as int, src_text@, ace_text@
                    ) is None);
                    assert(raw_view(src@) == ckc_spec::align::side_spans(seqs_view(rows@), "src"@));
                    assert(raw_view(ace@) == ckc_spec::align::side_spans(seqs_view(rows@), "ace"@));
                    assert(raw_groups_canonical(src@));
                    assert(raw_groups_canonical(ace@));
                    let result = ERows::Ok { src, ace };
                    assert(rows_contract(result, seqs_view(rows@), n as int, src_text@, ace_text@));
                    result
                },
            }
        },
    }
}

fn group_equal(a: &[char], b: &[char]) -> (r: bool)
    requires
        ckc_spec::align::is_canonical_decimal(a@),
        ckc_spec::align::is_canonical_decimal(b@),
    ensures r == (ckc_spec::align::dec_value(a@) == ckc_spec::align::dec_value(b@)),
{
    let same = seq_equal(a, b);
    proof {
        if ckc_spec::align::dec_value(a@) == ckc_spec::align::dec_value(b@) {
            canonical_injective(a@, b@);
        }
    }
    same
}

fn contains_group(needle: &[char], spans: &[ERawSpan]) -> (r: bool)
    requires
        ckc_spec::align::is_canonical_decimal(needle@),
        raw_groups_canonical(spans@),
    ensures r == ckc_spec::align::groups_of(raw_view(spans@)).contains(
        ckc_spec::align::dec_value(needle@),
    ),
    decreases spans.len(),
{
    if spans.len() == 0 {
        false
    } else {
        proof { raw_canonical_drop_first(spans@); }
        if group_equal(needle, spans[0].group.as_slice()) {
            true
        } else {
            let tail = slice_subrange(spans, 1, spans.len());
            proof {
                assert_seqs_equal!(tail@ == spans@.drop_first());
                raw_view_drop_first(spans@);
            }
            contains_group(needle, tail)
        }
    }
}

fn all_groups_in(left: &[ERawSpan], right: &[ERawSpan]) -> (r: bool)
    requires
        raw_groups_canonical(left@),
        raw_groups_canonical(right@),
    ensures r == ckc_spec::align::groups_of(raw_view(left@)).subset_of(
        ckc_spec::align::groups_of(raw_view(right@)),
    ),
    decreases left.len(),
{
    if left.len() == 0 {
        true
    } else {
        proof { raw_canonical_drop_first(left@); }
        if !contains_group(left[0].group.as_slice(), right) {
            false
        } else {
            let tail = slice_subrange(left, 1, left.len());
            proof {
                assert_seqs_equal!(tail@ == left@.drop_first());
                raw_view_drop_first(left@);
            }
            all_groups_in(tail, right)
        }
    }
}

fn groups_equal(src: &[ERawSpan], ace: &[ERawSpan]) -> (r: bool)
    requires
        raw_groups_canonical(src@),
        raw_groups_canonical(ace@),
    ensures r == (ckc_spec::align::groups_of(raw_view(src@))
        == ckc_spec::align::groups_of(raw_view(ace@))),
{
    if !all_groups_in(src, ace) {
        false
    } else if !all_groups_in(ace, src) {
        false
    } else {
        proof {
            assert_sets_equal!(
                ckc_spec::align::groups_of(raw_view(src@))
                    == ckc_spec::align::groups_of(raw_view(ace@))
            );
        }
        true
    }
}

fn raw_le(a: &ERawSpan, b: &ERawSpan) -> (r: bool)
    requires
        ckc_spec::align::is_canonical_decimal(a.group@),
        ckc_spec::align::is_canonical_decimal(b.group@),
    ensures r == ckc_spec::align::raw_le(a@, b@),
{
    if a.start < b.start {
        true
    } else if a.start > b.start {
        false
    } else if a.end < b.end {
        true
    } else if a.end > b.end {
        false
    } else {
        decimal_le(a.group.as_slice(), b.group.as_slice())
    }
}

fn insert_raw(x: ERawSpan, mut s: Vec<ERawSpan>) -> (out: Vec<ERawSpan>)
    requires
        ckc_spec::align::is_canonical_decimal(x.group@),
        raw_groups_canonical(s@),
    ensures
        raw_view(out@) == ckc_spec::align::insert_raw(x@, raw_view(s@)),
        raw_groups_canonical(out@),
    decreases s.len(),
{
    let ghost before = s@;
    let ghost x_view = x@;
    let ghost x_group = x.group@;
    if s.len() == 0 {
        s.push(x);
        proof {
            reveal_with_fuel(raw_view, 2);
            reveal_with_fuel(ckc_spec::align::insert_raw, 2);
        }
        assert(raw_view(s@) == seq![x_view]);
        assert(ckc_spec::align::insert_raw(x_view, raw_view(before)) == seq![x_view]);
        s
    } else {
        proof { raw_canonical_drop_first(before); }
        if raw_le(&x, &s[0]) {
            assert(ckc_spec::align::raw_le(x_view, before[0]@));
            proof {
                raw_view_prepend(before, x);
                raw_canonical_prepend(before, x);
            }
            s.insert(0, x);
            proof { reveal_with_fuel(ckc_spec::align::insert_raw, 2); }
            assert(raw_view(s@) == seq![x_view] + raw_view(before));
            assert(ckc_spec::align::insert_raw(x_view, raw_view(before))
                == seq![x_view] + raw_view(before));
            s
        } else {
            assert(!ckc_spec::align::raw_le(x_view, before[0]@));
            proof { raw_view_drop_first(before); }
            let first = s.remove(0);
            proof { remove_zero(before); }
            let mut out = insert_raw(x, s);
            let ghost out_before = out@;
            proof {
                raw_view_prepend(out_before, first);
                raw_canonical_prepend(out_before, first);
            }
            out.insert(0, first);
            proof { reveal_with_fuel(ckc_spec::align::insert_raw, 2); }
            assert(raw_view(out@) == seq![before[0]@]
                + ckc_spec::align::insert_raw(x_view, raw_view(before.drop_first())));
            proof { reveal_with_fuel(raw_view, 2); }
            assert(raw_view(before) == seq![before[0]@] + raw_view(before.drop_first()));
            assert(ckc_spec::align::insert_raw(x_view, raw_view(before))
                == seq![before[0]@]
                    + ckc_spec::align::insert_raw(x_view, raw_view(before.drop_first())));
            assert(ckc_spec::align::is_canonical_decimal(x_group));
            out
        }
    }
}

fn sort_raw(mut s: Vec<ERawSpan>) -> (out: Vec<ERawSpan>)
    requires raw_groups_canonical(s@),
    ensures
        raw_view(out@) == ckc_spec::align::sort_raw(raw_view(s@)),
        raw_groups_canonical(out@),
    decreases s.len(),
{
    if s.len() == 0 {
        proof {
            reveal_with_fuel(raw_view, 2);
            reveal_with_fuel(ckc_spec::align::sort_raw, 2);
        }
        s
    } else {
        let ghost before = s@;
        proof {
            raw_canonical_drop_first(before);
            raw_view_drop_first(before);
        }
        let first = s.remove(0);
        proof { remove_zero(before); }
        let rest = sort_raw(s);
        assert(raw_view(rest@)
            == ckc_spec::align::sort_raw(raw_view(before.drop_first())));
        let out = insert_raw(first, rest);
        assert(raw_view(out@) == ckc_spec::align::insert_raw(
            before[0]@,
            ckc_spec::align::sort_raw(raw_view(before.drop_first())),
        ));
        proof {
            reveal_with_fuel(raw_view, 2);
            reveal_with_fuel(ckc_spec::align::sort_raw, 2);
        }
        assert(raw_view(before) == seq![before[0]@] + raw_view(before.drop_first()));
        assert(ckc_spec::align::sort_raw(raw_view(before))
            == ckc_spec::align::insert_raw(
                before[0]@,
                ckc_spec::align::sort_raw(raw_view(before.drop_first())),
            ));
        out
    }
}

pub open spec fn raw_overlap_at(s: Seq<ERawSpan>, i: int, j: int) -> bool
    recommends 0 <= i < s.len(), 0 <= j < s.len(),
{
    s[i].start < s[j].end && s[j].start < s[i].end
}

fn spans_intersect(a: &ERawSpan, b: &ERawSpan) -> (r: bool)
    ensures r == (a@.start < b@.end && b@.start < a@.end),
{
    a.start < b.end && b.start < a.end
}

fn has_overlap(spans: &[ERawSpan]) -> (r: bool)
    ensures r == ckc_spec::align::has_overlap(raw_view(spans@)),
{
    proof { raw_view_len(spans@); }
    let mut i = 0usize;
    while i < spans.len()
        invariant
            i <= spans@.len(),
            raw_view(spans@).len() == spans@.len(),
            forall|x: int, y: int| #![auto]
                0 <= x < y < spans@.len() && x < i
                    ==> !raw_overlap_at(spans@, x, y),
        decreases spans.len() - i,
    {
        let mut j = i + 1;
        while j < spans.len()
            invariant
                i < spans@.len(),
                i < j <= spans@.len(),
                raw_view(spans@).len() == spans@.len(),
                forall|x: int, y: int| #![auto]
                    0 <= x < y < spans@.len() && x < i
                        ==> !raw_overlap_at(spans@, x, y),
                forall|y: int| #![auto]
                    i < y < j ==> !raw_overlap_at(spans@, i as int, y),
            decreases spans.len() - j,
        {
            if spans_intersect(&spans[i], &spans[j]) {
                proof {
                    raw_view_index(spans@, i as int);
                    raw_view_index(spans@, j as int);
                    assert(ckc_spec::align::has_overlap(raw_view(spans@)));
                }
                return true;
            }
            j += 1;
        }
        i += 1;
    }
    proof {
        if ckc_spec::align::has_overlap(raw_view(spans@)) {
            let (x, y): (int, int) = choose|x: int, y: int|
                0 <= x < raw_view(spans@).len()
                    && 0 <= y < raw_view(spans@).len()
                    && x != y
                    && raw_view(spans@)[x].start < raw_view(spans@)[y].end
                    && raw_view(spans@)[y].start < raw_view(spans@)[x].end;
            raw_view_index(spans@, x);
            raw_view_index(spans@, y);
            if x < y {
                assert(raw_overlap_at(spans@, x, y));
                assert(false);
            } else {
                assert(raw_overlap_at(spans@, y, x));
                assert(false);
            }
        }
    }
    false
}

fn digit_char(d: usize) -> (c: char)
    requires d <= 9,
    ensures c == ckc_spec::align::digit_char(d as int),
{
    if d == 0 { '0' }
    else if d == 1 { '1' }
    else if d == 2 { '2' }
    else if d == 3 { '3' }
    else if d == 4 { '4' }
    else if d == 5 { '5' }
    else if d == 6 { '6' }
    else if d == 7 { '7' }
    else if d == 8 { '8' }
    else { '9' }
}

fn decimal_chars(n: usize) -> (out: Vec<char>)
    ensures out@ == ckc_spec::align::dec_str(n as int),
    decreases n,
{
    if n < 10 {
        let mut out = Vec::new();
        out.push(digit_char(n));
        proof { reveal_with_fuel(ckc_spec::align::dec_str, 2); }
        out
    } else {
        let q = n / 10;
        let r = n % 10;
        assert(q < n);
        assert(r <= 9);
        let mut out = decimal_chars(q);
        out.push(digit_char(r));
        proof { reveal_with_fuel(ckc_spec::align::dec_str, 2); }
        out
    }
}

fn lit_missing_trailing_newline() -> (out: Vec<char>)
    ensures out@ == "missing trailing newline"@,
{
    let out = vec![
        'm', 'i', 's', 's', 'i', 'n', 'g', ' ', 't', 'r', 'a', 'i', 'l', 'i', 'n', 'g',
        ' ', 'n', 'e', 'w', 'l', 'i', 'n', 'e',
    ];
    proof {
        reveal_strlit("missing trailing newline");
        assert_seqs_equal!(out@ == "missing trailing newline"@);
    }
    out
}

fn lit_empty_file() -> (out: Vec<char>)
    ensures out@ == "empty file"@,
{
    let out = vec!['e', 'm', 'p', 't', 'y', ' ', 'f', 'i', 'l', 'e'];
    proof {
        reveal_strlit("empty file");
        assert_seqs_equal!(out@ == "empty file"@);
    }
    out
}

fn lit_row() -> (out: Vec<char>)
    ensures out@ == "row "@,
{
    let out = vec!['r', 'o', 'w', ' '];
    proof {
        reveal_strlit("row ");
        assert_seqs_equal!(out@ == "row "@);
    }
    out
}

fn lit_colon_space() -> (out: Vec<char>)
    ensures out@ == ": "@,
{
    let out = vec![':', ' '];
    proof {
        reveal_strlit(": ");
        assert_seqs_equal!(out@ == ": "@);
    }
    out
}

fn lit_field_count() -> (out: Vec<char>)
    ensures out@ == "expected 4 tab-separated fields"@,
{
    let out = vec!['e', 'x', 'p', 'e', 'c', 't', 'e', 'd', ' ', '4', ' ', 't', 'a', 'b', '-', 's', 'e', 'p', 'a', 'r', 'a', 't', 'e', 'd', ' ', 'f', 'i', 'e', 'l', 'd', 's'];
    proof {
        reveal_strlit("expected 4 tab-separated fields");
        assert_seqs_equal!(out@ == "expected 4 tab-separated fields"@);
    }
    out
}

fn lit_group_canonical() -> (out: Vec<char>)
    ensures out@ == "group must be a canonical decimal"@,
{
    let out = vec!['g', 'r', 'o', 'u', 'p', ' ', 'm', 'u', 's', 't', ' ', 'b', 'e', ' ', 'a', ' ', 'c', 'a', 'n', 'o', 'n', 'i', 'c', 'a', 'l', ' ', 'd', 'e', 'c', 'i', 'm', 'a', 'l'];
    proof {
        reveal_strlit("group must be a canonical decimal");
        assert_seqs_equal!(out@ == "group must be a canonical decimal"@);
    }
    out
}

fn lit_start_canonical() -> (out: Vec<char>)
    ensures out@ == "start must be a canonical decimal"@,
{
    let out = vec!['s', 't', 'a', 'r', 't', ' ', 'm', 'u', 's', 't', ' ', 'b', 'e', ' ', 'a', ' ', 'c', 'a', 'n', 'o', 'n', 'i', 'c', 'a', 'l', ' ', 'd', 'e', 'c', 'i', 'm', 'a', 'l'];
    proof {
        reveal_strlit("start must be a canonical decimal");
        assert_seqs_equal!(out@ == "start must be a canonical decimal"@);
    }
    out
}

fn lit_empty_span() -> (out: Vec<char>)
    ensures out@ == "empty span"@,
{
    let out = vec!['e', 'm', 'p', 't', 'y', ' ', 's', 'p', 'a', 'n'];
    proof {
        reveal_strlit("empty span");
        assert_seqs_equal!(out@ == "empty span"@);
    }
    out
}

fn lit_side_vocab() -> (out: Vec<char>)
    ensures out@ == "side must be src or ace"@,
{
    let out = vec!['s', 'i', 'd', 'e', ' ', 'm', 'u', 's', 't', ' ', 'b', 'e', ' ', 's', 'r', 'c', ' ', 'o', 'r', ' ', 'a', 'c', 'e'];
    proof {
        reveal_strlit("side must be src or ace");
        assert_seqs_equal!(out@ == "side must be src or ace"@);
    }
    out
}

fn lit_out_of_range() -> (out: Vec<char>)
    ensures out@ == "span out of range"@,
{
    let out = vec!['s', 'p', 'a', 'n', ' ', 'o', 'u', 't', ' ', 'o', 'f', ' ', 'r', 'a', 'n', 'g', 'e'];
    proof {
        reveal_strlit("span out of range");
        assert_seqs_equal!(out@ == "span out of range"@);
    }
    out
}

fn lit_span_mismatch() -> (out: Vec<char>)
    ensures out@ == "span does not match the text at start"@,
{
    let out = vec!['s', 'p', 'a', 'n', ' ', 'd', 'o', 'e', 's', ' ', 'n', 'o', 't', ' ', 'm', 'a', 't', 'c', 'h', ' ', 't', 'h', 'e', ' ', 't', 'e', 'x', 't', ' ', 'a', 't', ' ', 's', 't', 'a', 'r', 't'];
    proof {
        reveal_strlit("span does not match the text at start");
        assert_seqs_equal!(out@ == "span does not match the text at start"@);
    }
    out
}

fn lit_not_both_sided() -> (out: Vec<char>)
    ensures out@ == "every group needs both a src span and an ace span"@,
{
    let out = vec!['e', 'v', 'e', 'r', 'y', ' ', 'g', 'r', 'o', 'u', 'p', ' ', 'n', 'e', 'e', 'd', 's', ' ', 'b', 'o', 't', 'h', ' ', 'a', ' ', 's', 'r', 'c', ' ', 's', 'p', 'a', 'n', ' ', 'a', 'n', 'd', ' ', 'a', 'n', ' ', 'a', 'c', 'e', ' ', 's', 'p', 'a', 'n'];
    proof {
        reveal_strlit("every group needs both a src span and an ace span");
        assert_seqs_equal!(out@ == "every group needs both a src span and an ace span"@);
    }
    out
}

fn lit_overlap_src() -> (out: Vec<char>)
    ensures out@ == "overlapping src spans"@,
{
    let out = vec!['o', 'v', 'e', 'r', 'l', 'a', 'p', 'p', 'i', 'n', 'g', ' ', 's', 'r', 'c', ' ', 's', 'p', 'a', 'n', 's'];
    proof {
        reveal_strlit("overlapping src spans");
        assert_seqs_equal!(out@ == "overlapping src spans"@);
    }
    out
}

fn lit_overlap_ace() -> (out: Vec<char>)
    ensures out@ == "overlapping ace spans"@,
{
    let out = vec!['o', 'v', 'e', 'r', 'l', 'a', 'p', 'p', 'i', 'n', 'g', ' ', 'a', 'c', 'e', ' ', 's', 'p', 'a', 'n', 's'];
    proof {
        reveal_strlit("overlapping ace spans");
        assert_seqs_equal!(out@ == "overlapping ace spans"@);
    }
    out
}

fn concat_chars(mut left: Vec<char>, mut right: Vec<char>) -> (out: Vec<char>)
    ensures out@ == left@ + right@,
{
    left.append(&mut right);
    left
}

fn row_prefix(n: usize) -> (out: Vec<char>)
    ensures out@ == ckc_spec::align::row_prefix(n as int),
{
    let out = concat_chars(lit_row(), decimal_chars(n));
    let out = concat_chars(out, lit_colon_space());
    proof { reveal(ckc_spec::align::row_prefix); }
    out
}

fn render_violation(v: EViolation) -> (out: Vec<char>)
    ensures out@ == ckc_spec::align::render(v@),
{
    match v {
        EViolation::MissingTrailingNewline => lit_missing_trailing_newline(),
        EViolation::EmptyFile => lit_empty_file(),
        EViolation::FieldCount { row } => concat_chars(row_prefix(row), lit_field_count()),
        EViolation::GroupCanonical { row } => concat_chars(row_prefix(row), lit_group_canonical()),
        EViolation::StartCanonical { row } => concat_chars(row_prefix(row), lit_start_canonical()),
        EViolation::EmptySpan { row } => concat_chars(row_prefix(row), lit_empty_span()),
        EViolation::SideVocab { row } => concat_chars(row_prefix(row), lit_side_vocab()),
        EViolation::OutOfRange { row } => concat_chars(row_prefix(row), lit_out_of_range()),
        EViolation::SpanMismatch { row } => concat_chars(row_prefix(row), lit_span_mismatch()),
        EViolation::NotBothSided => lit_not_both_sided(),
        EViolation::OverlapSrc => lit_overlap_src(),
        EViolation::OverlapAce => lit_overlap_ace(),
    }
}

pub open spec fn first_contract(
    r: ERows,
    align: Seq<char>,
    src_text: Seq<char>,
    ace_text: Seq<char>,
) -> bool {
    match r {
        ERows::Err(v) => ckc_spec::align::first_violation(align, src_text, ace_text) == Some(v@),
        ERows::Ok { src, ace } => {
            let rows = ckc_spec::align::split_at_seps(align.drop_last(), '\n');
            &&& ckc_spec::align::first_violation(align, src_text, ace_text) is None
            &&& raw_view(src@) == ckc_spec::align::side_spans(rows, "src"@)
            &&& raw_view(ace@) == ckc_spec::align::side_spans(rows, "ace"@)
            &&& raw_groups_canonical(src@)
            &&& raw_groups_canonical(ace@)
        },
    }
}

fn first_scan(align: &[char], src_text: &[char], ace_text: &[char]) -> (r: ERows)
    ensures first_contract(r, align@, src_text@, ace_text@),
{
    proof {
        reveal_strlit("src");
        reveal_strlit("ace");
        side_names_distinct();
    }
    if align.len() == 0 || align[align.len() - 1] != '\n' {
        return ERows::Err(EViolation::MissingTrailingNewline);
    }
    let body = slice_subrange(align, 0, align.len() - 1);
    proof { assert_seqs_equal!(body@ == align@.drop_last()); }
    if body.len() == 0 {
        return ERows::Err(EViolation::EmptyFile);
    }
    let rows = split_at_seps(body, '\n');
    let _row_count = rows.len();
    assert(rows@.len() == _row_count as int);
    assert(rows@.len() <= usize::MAX as int);
    match rows_scan(rows.as_slice(), 1, src_text, ace_text) {
        ERows::Err(v) => ERows::Err(v),
        ERows::Ok { src, ace } => {
            if !groups_equal(src.as_slice(), ace.as_slice()) {
                ERows::Err(EViolation::NotBothSided)
            } else if has_overlap(src.as_slice()) {
                ERows::Err(EViolation::OverlapSrc)
            } else if has_overlap(ace.as_slice()) {
                ERows::Err(EViolation::OverlapAce)
            } else {
                ERows::Ok { src, ace }
            }
        },
    }
}

pub open spec fn raw_group_values(s: Seq<ERawSpan>) -> Seq<int>
    decreases s.len(),
{
    if s.len() == 0 {
        Seq::empty()
    } else {
        seq![s[0]@.group] + raw_group_values(s.drop_first())
    }
}

proof fn raw_group_values_len(s: Seq<ERawSpan>)
    ensures raw_group_values(s).len() == s.len(),
    decreases s.len(),
{
    reveal_with_fuel(raw_group_values, 2);
    if s.len() > 0 {
        raw_group_values_len(s.drop_first());
    }
}

proof fn raw_group_values_drop_first(s: Seq<ERawSpan>)
    requires s.len() > 0,
    ensures raw_group_values(s.drop_first()) == raw_group_values(s).drop_first(),
{
    reveal_with_fuel(raw_group_values, 2);
}

proof fn raw_group_values_prepend(s: Seq<ERawSpan>, x: ERawSpan)
    ensures raw_group_values(s.insert(0, x)) == seq![x@.group] + raw_group_values(s),
{
    insert_zero(s, x);
    assert(s.insert(0, x) == seq![x] + s);
    assert(s.insert(0, x).len() > 0);
    assert(s.insert(0, x)[0] == x);
    assert_seqs_equal!(s.insert(0, x).drop_first() == s);
    reveal_with_fuel(raw_group_values, 2);
}

fn copy_raw_span(x: &ERawSpan) -> (out: ERawSpan)
    ensures
        out@ == x@,
        out.group@ == x.group@,
{
    let group = slice_to_vec(x.group.as_slice());
    ERawSpan { start: x.start, end: x.end, group }
}

fn copy_raw_spans(spans: &[ERawSpan]) -> (out: Vec<ERawSpan>)
    requires raw_groups_canonical(spans@),
    ensures
        raw_view(out@) == raw_view(spans@),
        raw_groups_canonical(out@),
    decreases spans.len(),
{
    if spans.len() == 0 {
        let out = Vec::new();
        proof { reveal_with_fuel(raw_view, 2); }
        out
    } else {
        proof {
            raw_canonical_drop_first(spans@);
            raw_view_drop_first(spans@);
        }
        let first = copy_raw_span(&spans[0]);
        let tail = slice_subrange(spans, 1, spans.len());
        proof { assert_seqs_equal!(tail@ == spans@.drop_first()); }
        let mut out = copy_raw_spans(tail);
        let ghost before = out@;
        proof {
            assert(first@ == spans@[0]@);
            assert(ckc_spec::align::is_canonical_decimal(first.group@));
            raw_view_prepend(before, first);
            raw_canonical_prepend(before, first);
        }
        out.insert(0, first);
        proof { reveal_with_fuel(raw_view, 2); }
        assert(raw_view(out@) == seq![spans@[0]@] + raw_view(spans@.drop_first()));
        out
    }
}

fn group_order(mut spans: Vec<ERawSpan>, mut seen: Vec<ERawSpan>) -> (out: Vec<ERawSpan>)
    requires
        raw_groups_canonical(spans@),
        raw_groups_canonical(seen@),
    ensures
        raw_group_values(out@) == ckc_spec::align::groups_in_order(
            raw_view(spans@),
            ckc_spec::align::groups_of(raw_view(seen@)),
        ),
        raw_groups_canonical(out@),
    decreases spans.len(),
{
    if spans.len() == 0 {
        let out = Vec::new();
        proof {
            reveal_with_fuel(raw_view, 2);
            reveal_with_fuel(raw_group_values, 2);
            reveal_with_fuel(ckc_spec::align::groups_in_order, 2);
        }
        out
    } else {
        let ghost before = spans@;
        let ghost seen_before = seen@;
        proof {
            raw_canonical_drop_first(before);
            raw_view_drop_first(before);
        }
        let present = contains_group(spans[0].group.as_slice(), seen.as_slice());
        let first = spans.remove(0);
        proof { remove_zero(before); }
        if present {
            let out = group_order(spans, seen);
            proof {
                reveal_with_fuel(raw_view, 2);
                reveal_with_fuel(ckc_spec::align::groups_in_order, 2);
            }
            assert(raw_view(before)[0] == first@);
            assert(raw_view(before).drop_first() == raw_view(before.drop_first()));
            assert(ckc_spec::align::groups_of(raw_view(seen_before)).contains(first@.group));
            out
        } else {
            let seen_copy = copy_raw_span(&first);
            let ghost seen_old = seen@;
            proof {
                assert(seen_copy@ == first@);
                assert(ckc_spec::align::is_canonical_decimal(seen_copy.group@));
                raw_view_prepend(seen_old, seen_copy);
                raw_canonical_prepend(seen_old, seen_copy);
            }
            seen.insert(0, seen_copy);
            proof {
                assert(raw_view(seen@) == seq![first@] + raw_view(seen_old));
                assert(raw_view(seen@).len() > 0);
                assert(raw_view(seen@)[0] == first@);
                assert_seqs_equal!(raw_view(seen@).drop_first() == raw_view(seen_old));
                reveal_with_fuel(ckc_spec::align::groups_of, 2);
                assert(ckc_spec::align::groups_of(raw_view(seen@))
                    == ckc_spec::align::groups_of(raw_view(seen_old)).insert(first@.group));
            }
            let mut out = group_order(spans, seen);
            let ghost out_before = out@;
            proof {
                raw_group_values_prepend(out_before, first);
                raw_canonical_prepend(out_before, first);
            }
            out.insert(0, first);
            proof {
                reveal_with_fuel(raw_view, 2);
                reveal_with_fuel(ckc_spec::align::groups_in_order, 2);
            }
            assert(raw_view(before).drop_first() == raw_view(before.drop_first()));
            out
        }
    }
}

fn group_index(order: &[ERawSpan], group: &[char]) -> (index: usize)
    requires
        raw_groups_canonical(order@),
        ckc_spec::align::is_canonical_decimal(group@),
    ensures
        index as int == ckc_spec::align::index_in(
            raw_group_values(order@),
            ckc_spec::align::dec_value(group@),
        ),
        index <= order.len(),
    decreases order.len(),
{
    if order.len() == 0 {
        proof {
            reveal_with_fuel(raw_group_values, 2);
            reveal_with_fuel(ckc_spec::align::index_in, 2);
        }
        0
    } else {
        proof {
            raw_canonical_drop_first(order@);
            raw_group_values_drop_first(order@);
        }
        if group_equal(group, order[0].group.as_slice()) {
            proof {
                reveal_with_fuel(raw_group_values, 2);
                reveal_with_fuel(ckc_spec::align::index_in, 2);
            }
            0
        } else {
            let tail = slice_subrange(order, 1, order.len());
            proof { assert_seqs_equal!(tail@ == order@.drop_first()); }
            let rest = group_index(tail, group);
            assert(rest < usize::MAX);
            proof {
                reveal_with_fuel(raw_group_values, 2);
                reveal_with_fuel(ckc_spec::align::index_in, 2);
            }
            rest + 1
        }
    }
}

pub open spec fn out_view(
    s: Seq<ckc_spec::align::ESpan>,
) -> Seq<ckc_spec::align::OutSpan>
    decreases s.len(),
{
    if s.len() == 0 {
        Seq::empty()
    } else {
        seq![s[0]@] + out_view(s.drop_first())
    }
}

proof fn out_view_drop_first(s: Seq<ckc_spec::align::ESpan>)
    requires s.len() > 0,
    ensures out_view(s.drop_first()) == out_view(s).drop_first(),
{
    reveal_with_fuel(out_view, 2);
}

proof fn out_view_prepend(
    s: Seq<ckc_spec::align::ESpan>,
    x: ckc_spec::align::ESpan,
)
    ensures out_view(s.insert(0, x)) == seq![x@] + out_view(s),
{
    insert_zero(s, x);
    assert(s.insert(0, x) == seq![x] + s);
    assert(s.insert(0, x).len() > 0);
    assert(s.insert(0, x)[0] == x);
    assert_seqs_equal!(s.insert(0, x).drop_first() == s);
    reveal_with_fuel(out_view, 2);
}

proof fn out_view_len(s: Seq<ckc_spec::align::ESpan>)
    ensures out_view(s).len() == s.len(),
    decreases s.len(),
{
    reveal_with_fuel(out_view, 2);
    if s.len() > 0 {
        out_view_len(s.drop_first());
    }
}

fn to_out_spans(
    mut spans: Vec<ERawSpan>,
    order: &[ERawSpan],
) -> (out: Vec<ckc_spec::align::ESpan>)
    requires
        raw_groups_canonical(spans@),
        raw_groups_canonical(order@),
    ensures out_view(out@) == ckc_spec::align::to_out(
        raw_view(spans@),
        raw_group_values(order@),
    ),
    decreases spans.len(),
{
    if spans.len() == 0 {
        let out = Vec::new();
        proof {
            reveal_with_fuel(raw_view, 2);
            reveal_with_fuel(out_view, 2);
            reveal_with_fuel(ckc_spec::align::to_out, 2);
        }
        out
    } else {
        let ghost before = spans@;
        proof {
            raw_canonical_drop_first(before);
            raw_view_drop_first(before);
        }
        let first = spans.remove(0);
        proof { remove_zero(before); }
        assert(ckc_spec::align::is_canonical_decimal(first.group@));
        let index = group_index(order, first.group.as_slice());
        let span = ckc_spec::align::ESpan {
            start: first.start as u64,
            end: first.end as u64,
            index: index as u64,
        };
        let ghost span_view = span@;
        let mut out = to_out_spans(spans, order);
        let ghost out_before = out@;
        proof { out_view_prepend(out_before, span); }
        out.insert(0, span);
        proof {
            reveal_with_fuel(raw_view, 2);
            reveal_with_fuel(ckc_spec::align::to_out, 2);
        }
        assert(span_view == ckc_spec::align::OutSpan {
            start: first.start as int,
            end: first.end as int,
            index: ckc_spec::align::index_in(
                raw_group_values(order@),
                ckc_spec::align::dec_value(first.group@),
            ),
        });
        assert(raw_view(before).drop_first() == raw_view(before.drop_first()));
        out
    }
}

fn out_le(
    a: &ckc_spec::align::ESpan,
    b: &ckc_spec::align::ESpan,
) -> (r: bool)
    ensures r == ckc_spec::align::out_le(a@, b@),
{
    if a.start < b.start {
        true
    } else if a.start > b.start {
        false
    } else if a.end < b.end {
        true
    } else if a.end > b.end {
        false
    } else {
        a.index <= b.index
    }
}

fn insert_out(
    x: ckc_spec::align::ESpan,
    mut s: Vec<ckc_spec::align::ESpan>,
) -> (out: Vec<ckc_spec::align::ESpan>)
    ensures out_view(out@) == ckc_spec::align::insert_out(x@, out_view(s@)),
    decreases s.len(),
{
    let ghost before = s@;
    let ghost x_view = x@;
    if s.len() == 0 {
        s.push(x);
        proof {
            reveal_with_fuel(out_view, 2);
            reveal_with_fuel(ckc_spec::align::insert_out, 2);
        }
        s
    } else if out_le(&x, &s[0]) {
        proof { out_view_prepend(before, x); }
        s.insert(0, x);
        proof { reveal_with_fuel(ckc_spec::align::insert_out, 2); }
        assert(out_view(s@) == seq![x_view] + out_view(before));
        s
    } else {
        proof { out_view_drop_first(before); }
        let first = s.remove(0);
        proof { remove_zero(before); }
        let mut out = insert_out(x, s);
        let ghost out_before = out@;
        proof { out_view_prepend(out_before, first); }
        out.insert(0, first);
        proof {
            reveal_with_fuel(out_view, 2);
            reveal_with_fuel(ckc_spec::align::insert_out, 2);
        }
        assert(out_view(before) == seq![before[0]@] + out_view(before.drop_first()));
        assert(ckc_spec::align::insert_out(x_view, out_view(before))
            == seq![before[0]@]
                + ckc_spec::align::insert_out(x_view, out_view(before.drop_first())));
        out
    }
}

fn sort_out(
    mut s: Vec<ckc_spec::align::ESpan>,
) -> (out: Vec<ckc_spec::align::ESpan>)
    ensures out_view(out@) == ckc_spec::align::sort_out(out_view(s@)),
    decreases s.len(),
{
    if s.len() == 0 {
        proof {
            reveal_with_fuel(out_view, 2);
            reveal_with_fuel(ckc_spec::align::sort_out, 2);
        }
        s
    } else {
        let ghost before = s@;
        proof { out_view_drop_first(before); }
        let first = s.remove(0);
        proof { remove_zero(before); }
        let rest = sort_out(s);
        let out = insert_out(first, rest);
        proof {
            reveal_with_fuel(out_view, 2);
            reveal_with_fuel(ckc_spec::align::sort_out, 2);
        }
        assert(out_view(before) == seq![before[0]@] + out_view(before.drop_first()));
        assert(ckc_spec::align::sort_out(out_view(before))
            == ckc_spec::align::insert_out(
                before[0]@,
                ckc_spec::align::sort_out(out_view(before.drop_first())),
            ));
        out
    }
}

proof fn out_map_drop_first(s: Seq<ckc_spec::align::ESpan>)
    requires s.len() > 0,
    ensures
        s.drop_first().map_values(|e: ckc_spec::align::ESpan| e@)
            == s.map_values(|e: ckc_spec::align::ESpan| e@).drop_first(),
{
    assert_seqs_equal!(
        s.drop_first().map_values(|e: ckc_spec::align::ESpan| e@)
            == s.map_values(|e: ckc_spec::align::ESpan| e@).drop_first()
    );
}

proof fn out_view_map(s: Seq<ckc_spec::align::ESpan>)
    ensures out_view(s) == s.map_values(|e: ckc_spec::align::ESpan| e@),
    decreases s.len(),
{
    reveal_with_fuel(out_view, 2);
    if s.len() > 0 {
        out_map_drop_first(s);
        out_view_map(s.drop_first());
        assert_seqs_equal!(
            out_view(s) == s.map_values(|e: ckc_spec::align::ESpan| e@)
        );
    }
}

pub open spec fn model_from_raw(
    src: Seq<ckc_spec::align::RawSpan>,
    ace: Seq<ckc_spec::align::RawSpan>,
) -> ckc_spec::align::AlignModel {
    let order = ckc_spec::align::groups_in_order(
        ckc_spec::align::sort_raw(ace),
        Set::empty(),
    );
    ckc_spec::align::AlignModel {
        src: ckc_spec::align::sort_out(ckc_spec::align::to_out(src, order)),
        ace: ckc_spec::align::sort_out(ckc_spec::align::to_out(ace, order)),
        count: order.len() as int,
    }
}

fn model_from_spans(
    src: Vec<ERawSpan>,
    ace: Vec<ERawSpan>,
) -> (model: ckc_spec::align::EModel)
    requires
        raw_groups_canonical(src@),
        raw_groups_canonical(ace@),
    ensures model@ == model_from_raw(raw_view(src@), raw_view(ace@)),
{
    let ghost src_before = src@;
    let ghost ace_before = ace@;
    let ace_copy = copy_raw_spans(ace.as_slice());
    let sorted_ace = sort_raw(ace_copy);
    let seen = Vec::new();
    proof {
        assert(raw_groups_canonical(seen@));
        reveal_with_fuel(raw_view, 2);
    }
    let order = group_order(sorted_ace, seen);
    proof {
        reveal_with_fuel(ckc_spec::align::groups_of, 2);
        raw_group_values_len(order@);
    }
    assert(raw_group_values(order@) == ckc_spec::align::groups_in_order(
        ckc_spec::align::sort_raw(raw_view(ace_before)),
        Set::empty(),
    ));
    let count = order.len() as u64;
    let src_out = to_out_spans(src, order.as_slice());
    let ace_out = to_out_spans(ace, order.as_slice());
    let src_sorted = sort_out(src_out);
    let ace_sorted = sort_out(ace_out);
    proof {
        out_view_map(src_sorted@);
        out_view_map(ace_sorted@);
        reveal(model_from_raw);
    }
    ckc_spec::align::EModel { src: src_sorted, ace: ace_sorted, count }
}

pub(crate) fn align_check_impl(align: &[char], src: &[char], ace: &[char]) -> (r:
    ckc_spec::align::ECheck)
    ensures
        r@ == ckc_spec::align::align_outcome(align@, src@, ace@),
{
    let checked = first_scan(align, src, ace);
    match checked {
        ERows::Err(v) => {
            let detail = render_violation(v);
            proof {
                reveal(first_contract);
                reveal(ckc_spec::align::align_outcome);
            }
            ckc_spec::align::ECheck::Err(detail)
        },
        ERows::Ok { src: src_spans, ace: ace_spans } => {
            let model = model_from_spans(src_spans, ace_spans);
            proof {
                reveal(first_contract);
                reveal(model_from_raw);
                reveal(ckc_spec::align::model_of);
                reveal(ckc_spec::align::align_outcome);
            }
            ckc_spec::align::ECheck::Ok(model)
        },
    }
}

} // verus!
