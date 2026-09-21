use crate::align_impl as a;
use ckc_spec::align::*;
use vstd::assert_seqs_equal;
use vstd::prelude::*;
use vstd::slice::{slice_subrange, slice_to_vec};

verus! {

pub struct Span {
    pub start: usize,
    pub end: usize,
    pub group: Vec<char>,
    pub span: Vec<char>,
}

impl View for Span {
    type V = ResSpan;

    open spec fn view(&self) -> ResSpan {
        ResSpan {
            start: self.start as int,
            end: self.end as int,
            group: dec_value(self.group@),
            span: self.span@,
        }
    }
}

pub open spec fn spans(v: Seq<Span>) -> Seq<ResSpan> {
    v.map_values(|s: Span| s@)
}

pub open spec fn valid(v: Seq<Span>) -> bool {
    forall|i: int| 0 <= i < v.len() ==> is_canonical_decimal(v[i].group@)
}

pub open spec fn groups(v: Seq<Vec<char>>) -> Seq<int> {
    v.map_values(|s: Vec<char>| dec_value(s@))
}

pub open spec fn canon(v: Seq<Vec<char>>) -> bool {
    forall|i: int| 0 <= i < v.len() ==> is_canonical_decimal(v[i]@)
}

pub fn canonical_int(s: &[char]) -> (r: bool)
    ensures
        r == is_canonical_int(s@),
{
    if a::canonical_decimal(s) {
        return true;
    }
    if s.len() <= 1 || s[0] != '-' {
        return false;
    }
    let tail = slice_subrange(s, 1, s.len());
    proof {
        assert_seqs_equal!(tail@ == s@.drop_first());
        reveal_strlit("0");
        assert_seqs_equal!(seq!['0'] == "0"@);
    }
    a::canonical_decimal(tail) && !a::seq_equal(tail, &crate::resolve_text::digit_zero())
}

pub proof fn nonzero(s: Seq<char>)
    requires
        is_canonical_decimal(s),
        s != "0"@,
    ensures
        dec_value(s) > 0,
{
    reveal_strlit("0");
    if s[0] == '0' {
        assert_seqs_equal!(s == "0"@);
    }
    a::dec_positive(s);
}

pub proof fn canonical_print(s: Seq<char>)
    requires
        is_canonical_decimal(s),
    ensures
        dec_str(dec_value(s)) == s,
    decreases s.len(),
{
    a::all_digits_drop_last(s);
    a::dec_nonnegative(s.drop_last());
    a::digit_bounds(s.last());
    reveal_with_fuel(dec_value, 2);
    let p = dec_value(s.drop_last());
    let d = digit_value(s.last());
    assert(digit_char(d) == s.last());
    if s.len() == 1 {
        assert(s.drop_last().len() == 0);
        assert(p == 0);
        reveal_with_fuel(dec_str, 2);
        assert_seqs_equal!(s == seq![s.last()]);
    } else {
        a::canonical_drop_last(s);
        a::dec_positive(s.drop_last());
        canonical_print(s.drop_last());
        vstd::arithmetic::div_mod::lemma_div_multiples_vanish_fancy(p, d, 10);
        vstd::arithmetic::div_mod::lemma_mod_multiples_vanish(p, d, 10);
        vstd::arithmetic::div_mod::lemma_small_mod(d as nat, 10);
        assert(p * 10 == 10 * p) by (nonlinear_arith);
        reveal_with_fuel(dec_str, 2);
        assert_seqs_equal!(s == s.drop_last().push(s.last()));
    }
}

pub proof fn too_many(text: Seq<char>, span: Seq<char>, n: int, i: int)
    requires
        span.len() > 0,
        0 <= i <= text.len(),
        n > text.len() - i,
    ensures
        nth_start(text, span, n, i) == -1,
    decreases text.len() - i,
{
    reveal_with_fuel(nth_start, 2);
    if i + span.len() <= text.len() {
        if text.subrange(i, i + span.len()) == span {
            too_many(text, span, n - 1, i + span.len());
        } else {
            too_many(text, span, n, i + 1);
        }
    }
}

pub fn nth(text: &[char], span: &[char], n: usize, i: usize) -> (r: Option<usize>)
    requires
        span.len() > 0,
        n > 0,
        i <= text.len(),
    ensures
        match r {
            Some(k) => k as int == nth_start(text@, span@, n as int, i as int) && k + span.len()
                <= text.len(),
            None => nth_start(text@, span@, n as int, i as int) == -1,
        },
    decreases text.len() - i,
{
    proof {
        reveal_with_fuel(nth_start, 2);
    }
    if span.len() > text.len() - i {
        return None;
    }
    if a::seq_equal(slice_subrange(text, i, i + span.len()), span) {
        if n == 1 {
            Some(i)
        } else {
            nth(text, span, n - 1, i + span.len())
        }
    } else {
        nth(text, span, n, i + 1)
    }
}

pub fn chars_le_exec(x: &[char], y: &[char]) -> (r: bool)
    ensures
        r == chars_le(x@, y@),
    decreases x.len(),
{
    if x.len() == 0 {
        true
    } else if y.len() == 0 {
        false
    } else if x[0] < y[0] {
        true
    } else if x[0] > y[0] {
        false
    } else {
        chars_le_exec(slice_subrange(x, 1, x.len()), slice_subrange(y, 1, y.len()))
    }
}

pub fn less(x: &Span, y: &Span) -> (r: bool)
    requires
        is_canonical_decimal(x.group@),
        is_canonical_decimal(y.group@),
    ensures
        r == res_le(x@, y@),
{
    if x.start != y.start {
        return x.start < y.start;
    }
    if x.end != y.end {
        return x.end < y.end;
    }
    match a::decimal_cmp(&x.group, &y.group) {
        a::EDecimalCmp::Less => true,
        a::EDecimalCmp::Greater => false,
        a::EDecimalCmp::Equal => chars_le_exec(&x.span, &y.span),
    }
}

pub fn clone_span(x: &Span) -> (r: Span)
    ensures
        r@ == x@,
        r.group@ == x.group@,
{
    Span {
        start: x.start,
        end: x.end,
        group: slice_to_vec(x.group.as_slice()),
        span: slice_to_vec(x.span.as_slice()),
    }
}

proof fn insert_at(x: ResSpan, s: Seq<ResSpan>, i: nat)
    requires
        i <= s.len(),
        forall|j: int| 0 <= j < i ==> !res_le(x, s[j]),
        i == s.len() || res_le(x, s[i as int]),
    ensures
        insert_res(x, s) == s.take(i as int) + seq![x] + s.skip(i as int),
    decreases i,
{
    reveal_with_fuel(insert_res, 2);
    if i > 0 {
        insert_at(x, s.drop_first(), (i - 1) as nat);
        assert_seqs_equal!(s.take(i as int)==seq![s[0]]+s.drop_first().take(i as int-1));
        assert_seqs_equal!(s.skip(i as int)==s.drop_first().skip(i as int-1));
    }
}

pub fn insert(x: Span, mut s: Vec<Span>) -> (r: Vec<Span>)
    requires
        is_canonical_decimal(x.group@),
        valid(s@),
    ensures
        spans(r@) == insert_res(x@, spans(s@)),
        valid(r@),
        r.len() == s.len() + 1,
{
    let mut i = 0usize;
    while i < s.len() && !less(&x, &s[i])
        invariant
            i <= s.len(),
            valid(s@),
            is_canonical_decimal(x.group@),
            forall|j: int| 0 <= j < i ==> !res_le(x@, s@[j]@),
        decreases s.len() - i,
    {
        i += 1;
    }
    let ghost before = s@;
    s.insert(i, x);
    proof {
        insert_at(x@, spans(before), i as nat);
        assert_seqs_equal!(spans(s@)==spans(before).take(i as int)+seq![x@]+spans(before).skip(i as int));
        assert forall|j: int| 0 <= j < s.len() implies is_canonical_decimal(s@[j].group@) by {
            if j < i {
                assert(s@[j] === before[j]);
            } else if j == i {
                assert(s@[j] === x);
            } else {
                assert(s@[j] === before[j - 1]);
            }
        }
    }
    s
}

pub fn sorted(s: &Vec<Span>) -> (r: Vec<Span>)
    requires
        valid(s@),
    ensures
        spans(r@) == sort_res(spans(s@)),
        valid(r@),
        r.len() == s.len(),
{
    let mut r = Vec::new();
    let mut i = s.len();
    while i > 0
        invariant
            i <= s.len(),
            valid(s@),
            valid(r@),
            r.len() == s.len() - i,
            spans(r@) == sort_res(spans(s@).skip(i as int)),
        decreases i,
    {
        let x = clone_span(&s[i - 1]);
        r = insert(x, r);
        proof {
            assert_seqs_equal!(spans(s@).skip(i as int-1).drop_first()==spans(s@).skip(i as int));
            reveal_with_fuel(sort_res, 2);
        }
        i -= 1;
    }
    proof {
        assert_seqs_equal!(spans(s@).skip(0)==spans(s@));
    }
    r
}

pub fn contains_group(v: &Vec<Vec<char>>, key: &[char]) -> (r: bool)
    requires
        canon(v@),
        is_canonical_decimal(key@),
    ensures
        r == groups(v@).contains(dec_value(key@)),
{
    let mut i = 0usize;
    while i < v.len()
        invariant
            i <= v.len(),
            canon(v@),
            is_canonical_decimal(key@),
            forall|j: int| 0 <= j < i ==> dec_value(v@[j]@) != dec_value(key@),
        decreases v.len() - i,
    {
        match a::decimal_cmp(&v[i], key) {
            a::EDecimalCmp::Equal => {
                proof {
                    assert(groups(v@)[i as int] == dec_value(key@));
                }
                return true;
            },
            _ => {},
        }
        i += 1;
    }
    false
}

pub fn group_list_exec(s: &Vec<Span>, i: usize, mut acc: Vec<Vec<char>>) -> (r: Vec<Vec<char>>)
    requires
        i <= s.len(),
        valid(s@),
        canon(acc@),
    ensures
        groups(r@) == group_list(spans(s@).skip(i as int), groups(acc@)),
        canon(r@),
        r.len() <= acc.len() + s.len() - i,
    decreases s.len() - i,
{
    proof {
        reveal_with_fuel(group_list, 2);
    }
    if i == s.len() {
        return acc;
    }
    let ghost original = acc@;
    if !contains_group(&acc, &s[i].group) {
        let g = slice_to_vec(s[i].group.as_slice());
        acc.push(g);
        proof {
            assert_seqs_equal!(groups(acc@)==groups(original).push(dec_value(g@)));
        }
    }
    proof {
        assert_seqs_equal!(spans(s@).skip(i as int).drop_first()==spans(s@).skip(i as int+1));
    }
    group_list_exec(s, i + 1, acc)
}

pub fn groups_subset(x: &Vec<Vec<char>>, y: &Vec<Vec<char>>) -> (r: bool)
    requires
        canon(x@),
        canon(y@),
    ensures
        r == (forall|i: int|
            0 <= i < groups(x@).len() ==> groups(y@).contains(#[trigger] groups(x@)[i])),
{
    let mut i = 0usize;
    while i < x.len()
        invariant
            i <= x.len(),
            canon(x@),
            canon(y@),
            forall|j: int| 0 <= j < i ==> groups(y@).contains(#[trigger] groups(x@)[j]),
        decreases x.len() - i,
    {
        if !contains_group(y, &x[i]) {
            proof {
                assert(groups(x@)[i as int] == dec_value(x@[i as int]@));
                assert(!groups(y@).contains(groups(x@)[i as int]));
            }
            return false;
        }
        i += 1;
    }
    true
}

pub fn same(x: &Vec<Vec<char>>, y: &Vec<Vec<char>>) -> (r: bool)
    requires
        canon(x@),
        canon(y@),
    ensures
        r == same_groups(groups(x@), groups(y@)),
{
    groups_subset(x, y) && groups_subset(y, x)
}

pub fn side(name: &[char], s: &Vec<Span>, i: usize, prev: usize) -> (r: Result<
    Vec<char>,
    Vec<char>,
>)
    requires
        i <= s.len(),
        valid(s@),
    ensures
        match r {
            Ok(t) => side_out(name@, spans(s@).skip(i as int), prev as int) == Ok(t@),
            Err(e) => side_out(name@, spans(s@).skip(i as int), prev as int) == Err(e@),
        },
    decreases s.len() - i,
{
    proof {
        reveal_with_fuel(side_out, 2);
    }
    if i == s.len() {
        return Ok(Vec::new());
    }
    let x = &s[i];
    if x.start < prev {
        let mut e = a::concat_chars(crate::resolve_text::overlap(), slice_to_vec(name));
        e = a::concat_chars(e, crate::resolve_text::offset());
        e = a::concat_chars(e, a::decimal_chars(x.start));
        return Err(e);
    }
    proof {
        assert_seqs_equal!(spans(s@).skip(i as int).drop_first()==spans(s@).skip(i as int+1));
    }
    let tail = side(name, s, i + 1, x.end);
    proof {
        reveal_with_fuel(side_out, 2);
        assert(spans(s@).skip(i as int)[0] == x@);
    }
    match tail {
        Err(e) => {
            proof {
                assert(side_out(name@, spans(s@).skip(i as int + 1), x.end as int) == Err(e@));
                reveal_with_fuel(side_out, 2);
                assert(side_out(name@, spans(s@).skip(i as int), prev as int) == Err(e@));
            }
            Err(e)
        },
        Ok(t) => {
            let mut row = slice_to_vec(x.group.as_slice());
            proof {
                canonical_print(x.group@);
            }
            row.push('\t');
            row = a::concat_chars(row, slice_to_vec(name));
            row.push('\t');
            row = a::concat_chars(row, a::decimal_chars(x.start));
            row.push('\t');
            row = a::concat_chars(row, slice_to_vec(x.span.as_slice()));
            row.push('\n');
            proof {
                reveal_strlit("\t");
                reveal_strlit("\n");
                assert_seqs_equal!(row@ == dec_str(x@.group) + "\t"@ + name@ + "\t"@ + dec_str(x@.start) + "\t"@ + x@.span + "\n"@);
                reveal_with_fuel(side_out, 2);
                assert(side_out(name@, spans(s@).skip(i as int), prev as int) == Ok(row@ + t@));
            }
            Ok(a::concat_chars(row, t))
        },
    }
}

} // verus!
