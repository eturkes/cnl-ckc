use vstd::assert_seqs_equal;
use vstd::prelude::*;
use vstd::slice::{slice_subrange, slice_to_vec};
verus! {

pub enum EDecimalCmp {
    Less,
    Equal,
    Greater,
}

pub(crate) fn ascii_digit(c: char) -> (r: bool)
    ensures
        r == ckc_spec::align::is_ascii_digit(c),
{
    c >= '0' && c <= '9'
}

pub(crate) fn all_ascii_digits(s: &[char]) -> (r: bool)
    ensures
        r == ckc_spec::align::all_ascii_digits(s@),
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

pub(crate) fn canonical_decimal(s: &[char]) -> (r: bool)
    ensures
        r == ckc_spec::align::is_canonical_decimal(s@),
{
    if s.len() == 0 {
        false
    } else if s.len() > 1 && s[0] == '0' {
        false
    } else {
        all_ascii_digits(s)
    }
}

pub(crate) fn digit_value(c: char) -> (d: usize)
    requires
        ckc_spec::align::is_ascii_digit(c),
    ensures
        d as int == ckc_spec::align::digit_value(c),
        d <= 9,
{
    if c == '0' {
        0
    } else if c == '1' {
        1
    } else if c == '2' {
        2
    } else if c == '3' {
        3
    } else if c == '4' {
        4
    } else if c == '5' {
        5
    } else if c == '6' {
        6
    } else if c == '7' {
        7
    } else if c == '8' {
        8
    } else {
        9
    }
}

pub(crate) proof fn prefix_push<A>(s: Seq<A>, i: int)
    requires
        0 <= i < s.len(),
    ensures
        s.subrange(0, i + 1) == s.subrange(0, i).push(s[i]),
{
    assert_seqs_equal!(s.subrange(0, i + 1) == s.subrange(0, i).push(s[i]));
}

pub(crate) proof fn dec_push(s: Seq<char>, c: char)
    ensures
        ckc_spec::align::dec_value(s.push(c)) == ckc_spec::align::dec_value(s) * 10
            + ckc_spec::align::digit_value(c),
{
    assert_seqs_equal!(s.push(c).drop_last() == s);
    assert(s.push(c).last() == c);
    reveal_with_fuel(ckc_spec::align::dec_value, 2);
}

pub(crate) proof fn digit_bounds(c: char)
    requires
        ckc_spec::align::is_ascii_digit(c),
    ensures
        0 <= ckc_spec::align::digit_value(c),
        ckc_spec::align::digit_value(c) <= 9,
{
}

pub(crate) proof fn all_digits_drop_last(s: Seq<char>)
    requires
        s.len() > 0,
        ckc_spec::align::all_ascii_digits(s),
    ensures
        ckc_spec::align::all_ascii_digits(s.drop_last()),
{
    assert forall|i: int|
        #![auto]
        0 <= i < s.drop_last().len() ==> ckc_spec::align::is_ascii_digit(s.drop_last()[i]) by {
        if 0 <= i < s.drop_last().len() {
            assert(i < s.len());
            assert(s.drop_last()[i] == s[i]);
        }
    }
}

pub(crate) proof fn canonical_drop_last(s: Seq<char>)
    requires
        s.len() > 1,
        ckc_spec::align::is_canonical_decimal(s),
    ensures
        ckc_spec::align::is_canonical_decimal(s.drop_last()),
{
    all_digits_drop_last(s);
    assert(s.drop_last().len() == s.len() - 1);
    if s.drop_last().len() > 1 {
        assert(s.drop_last()[0] == s[0]);
    }
}

pub(crate) proof fn dec_nonnegative(s: Seq<char>)
    requires
        ckc_spec::align::all_ascii_digits(s),
    ensures
        ckc_spec::align::dec_value(s) >= 0,
    decreases s.len(),
{
    reveal_with_fuel(ckc_spec::align::dec_value, 2);
    if s.len() > 0 {
        all_digits_drop_last(s);
        dec_nonnegative(s.drop_last());
        digit_bounds(s.last());
    }
}

pub(crate) proof fn dec_positive(s: Seq<char>)
    requires
        ckc_spec::align::is_canonical_decimal(s),
        s[0] != '0',
    ensures
        ckc_spec::align::dec_value(s) > 0,
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

pub open spec fn decimal_cmp_contract(c: EDecimalCmp, a: Seq<char>, b: Seq<char>) -> bool {
    match c {
        EDecimalCmp::Less => ckc_spec::align::dec_value(a) < ckc_spec::align::dec_value(b),
        EDecimalCmp::Equal => ckc_spec::align::dec_value(a) == ckc_spec::align::dec_value(b),
        EDecimalCmp::Greater => ckc_spec::align::dec_value(a) > ckc_spec::align::dec_value(b),
    }
}

pub(crate) fn decimal_cmp(a: &[char], b: &[char]) -> (c: EDecimalCmp)
    requires
        ckc_spec::align::all_ascii_digits(a@),
        ckc_spec::align::all_ascii_digits(b@),
    ensures
        decimal_cmp_contract(c, a@, b@),
    decreases a.len() + b.len(),
{
    if a.len() == 0 && b.len() == 0 {
        return EDecimalCmp::Equal;
    }
    let ap = if a.len() == 0 {
        a
    } else {
        slice_subrange(a, 0, a.len() - 1)
    };
    let bp = if b.len() == 0 {
        b
    } else {
        slice_subrange(b, 0, b.len() - 1)
    };
    let da = if a.len() == 0 {
        0
    } else {
        assert(ckc_spec::align::is_ascii_digit(a@[a@.len() - 1]));
        digit_value(a[a.len() - 1])
    };
    let db = if b.len() == 0 {
        0
    } else {
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
        assert(ckc_spec::align::dec_value(a@) == ckc_spec::align::dec_value(ap@) * 10 + da as int);
        assert(ckc_spec::align::dec_value(b@) == ckc_spec::align::dec_value(bp@) * 10 + db as int);
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
            if da < db {
                EDecimalCmp::Less
            } else if da > db {
                EDecimalCmp::Greater
            } else {
                EDecimalCmp::Equal
            }
        },
    }
}

pub(crate) fn bounded_decimal(s: &[char], bound: usize) -> (r: Option<usize>)
    requires
        ckc_spec::align::is_canonical_decimal(s@),
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
    proof {
        assert_seqs_equal!(s@.subrange(0, s@.len() as int) == s@);
    }
    if over {
        None
    } else {
        Some(value)
    }
}

pub(crate) fn seq_equal(a: &[char], b: &[char]) -> (r: bool)
    ensures
        r == (a@ == b@),
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
    proof {
        assert_seqs_equal!(a@ == b@);
    }
    true
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

pub(crate) proof fn remove_zero<A>(s: Seq<A>)
    requires
        s.len() > 0,
    ensures
        s.remove(0) == s.drop_first(),
{
    assert_seqs_equal!(s.remove(0) == s.drop_first());
}

pub(crate) proof fn insert_zero<A>(s: Seq<A>, x: A)
    ensures
        s.insert(0, x) == seq![x] + s,
{
    assert_seqs_equal!(s.insert(0, x) == seq![x] + s);
}

pub(crate) proof fn seqs_view_concat(a: Seq<Vec<char>>, b: Seq<Vec<char>>)
    ensures
        seqs_view(a + b) == seqs_view(a) + seqs_view(b),
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

pub(crate) proof fn seqs_view_drop_first(s: Seq<Vec<char>>)
    requires
        s.len() > 0,
    ensures
        seqs_view(s.drop_first()) == seqs_view(s).drop_first(),
{
}

pub(crate) proof fn seqs_view_len(s: Seq<Vec<char>>)
    ensures
        seqs_view(s).len() == s.len(),
    decreases s.len(),
{
    reveal_with_fuel(seqs_view, 2);
    if s.len() > 0 {
        seqs_view_len(s.drop_first());
    }
}

pub(crate) proof fn seqs_view_index(s: Seq<Vec<char>>, i: int)
    requires
        0 <= i < s.len(),
        i < seqs_view(s).len(),
    ensures
        seqs_view(s)[i] == s[i]@,
    decreases i,
{
    reveal_with_fuel(seqs_view, 2);
    if i > 0 {
        seqs_view_len(s.drop_first());
        seqs_view_index(s.drop_first(), i - 1);
        assert(s.drop_first()[i - 1] == s[i]);
    }
}

pub(crate) proof fn split_nonempty(s: Seq<char>, sep: char)
    ensures
        ckc_spec::align::split_at_seps(s, sep).len() > 0,
    decreases s.len(),
{
    if s.len() > 0 {
        split_nonempty(s.drop_first(), sep);
    }
}

pub(crate) fn split_at_seps(s: &[char], sep: char) -> (out: Vec<Vec<char>>)
    ensures
        seqs_view(out@) == ckc_spec::align::split_at_seps(s@, sep),
    decreases s.len(),
{
    if s.len() == 0 {
        let mut out = Vec::new();
        out.push(Vec::new());
        proof {
            reveal_with_fuel(seqs_view, 2);
        }
        out
    } else {
        let tail = slice_subrange(s, 1, s.len());
        let mut rest = split_at_seps(tail, sep);
        proof {
            split_nonempty(tail@, sep);
        }
        let ghost before = rest@;
        let mut out = Vec::new();
        if s[0] == sep {
            out.push(Vec::new());
            proof {
                reveal_with_fuel(seqs_view, 2);
            }
            assert(seqs_view(out@) == seq![Seq::<char>::empty()]);
            assert(rest@ == before);
        } else {
            let mut first = rest.remove(0);
            proof {
                remove_zero(before);
            }
            assert(rest@ == before.drop_first());
            assert(first@ == before[0]@);
            let ghost first_before = first@;
            first.insert(0, s[0]);
            proof {
                insert_zero(first_before, s@[0]);
            }
            assert(first@ == seq![s@[0]] + before[0]@);
            out.push(first);
            proof {
                reveal_with_fuel(seqs_view, 2);
            }
            assert(seqs_view(out@) == seq![seq![s@[0]] + before[0]@]);
        }
        let ghost head = out@;
        let ghost remaining = rest@;
        out.append(&mut rest);
        proof {
            seqs_view_concat(head, remaining);
        }
        assert(seqs_view(out@) == seqs_view(head) + seqs_view(remaining));
        proof {
            seqs_view_drop_first(before);
        }
        assert(seqs_view(out@) == if s@[0] == sep {
            seq![Seq::<char>::empty()] + seqs_view(before)
        } else {
            seq![seq![s@[0]] + seqs_view(before)[0]] + seqs_view(before).drop_first()
        });
        out
    }
}

pub(crate) fn digit_char(d: usize) -> (c: char)
    requires
        d <= 9,
    ensures
        c == ckc_spec::align::digit_char(d as int),
{
    if d == 0 {
        '0'
    } else if d == 1 {
        '1'
    } else if d == 2 {
        '2'
    } else if d == 3 {
        '3'
    } else if d == 4 {
        '4'
    } else if d == 5 {
        '5'
    } else if d == 6 {
        '6'
    } else if d == 7 {
        '7'
    } else if d == 8 {
        '8'
    } else {
        '9'
    }
}

pub(crate) fn decimal_chars(n: usize) -> (out: Vec<char>)
    ensures
        out@ == ckc_spec::align::dec_str(n as int),
    decreases n,
{
    if n < 10 {
        let mut out = Vec::new();
        out.push(digit_char(n));
        proof {
            reveal_with_fuel(ckc_spec::align::dec_str, 2);
        }
        out
    } else {
        let q = n / 10;
        let r = n % 10;
        assert(q < n);
        assert(r <= 9);
        let mut out = decimal_chars(q);
        out.push(digit_char(r));
        proof {
            reveal_with_fuel(ckc_spec::align::dec_str, 2);
        }
        out
    }
}

pub(crate) fn lit_row() -> (out: Vec<char>)
    ensures
        out@ == "row "@,
{
    let out = vec!['r', 'o', 'w', ' '];
    proof {
        reveal_strlit("row ");
        assert_seqs_equal!(out@ == "row "@);
    }
    out
}

pub(crate) fn lit_colon_space() -> (out: Vec<char>)
    ensures
        out@ == ": "@,
{
    let out = vec![':', ' '];
    proof {
        reveal_strlit(": ");
        assert_seqs_equal!(out@ == ": "@);
    }
    out
}

pub(crate) fn concat_chars(mut left: Vec<char>, mut right: Vec<char>) -> (out: Vec<char>)
    ensures
        out@ == left@ + right@,
{
    left.append(&mut right);
    left
}

pub(crate) fn row_prefix(n: usize) -> (out: Vec<char>)
    ensures
        out@ == ckc_spec::align::row_prefix(n as int),
{
    let out = concat_chars(lit_row(), decimal_chars(n));
    let out = concat_chars(out, lit_colon_space());
    proof {
        reveal(ckc_spec::align::row_prefix);
    }
    out
}

} // verus!
