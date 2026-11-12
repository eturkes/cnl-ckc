#[cfg(verus_keep_ghost)]
use ckc_spec::engine::{
    args_lt, bytes_lt, dedup, insert_sorted, msort, rank, sort_unique, term_lt,
};
use ckc_spec::term::Term;
use vstd::prelude::*;

verus! {

// Contract mc MC-T3: k2_sort's `sort_unique` yields the canonical answer list — strictly
// increasing in the standard order (total: k2_sort::term_order_total; irreflexive:
// k2_term::term_lt_irreflexive; transitive: below) and holding exactly its input's terms.
proof fn bytes_lt_trans(a: Seq<u8>, b: Seq<u8>, c: Seq<u8>)
    requires
        bytes_lt(a, b),
        bytes_lt(b, c),
    ensures
        bytes_lt(a, c),
    decreases a.len(),
{
    reveal_with_fuel(bytes_lt, 2);
    if a.len() > 0 && b.len() > 0 && c.len() > 0 && a[0] == b[0] && b[0] == c[0] {
        bytes_lt_trans(a.drop_first(), b.drop_first(), c.drop_first());
    }
}

pub proof fn term_lt_trans(a: Term, b: Term, c: Term)
    requires
        term_lt(a, b),
        term_lt(b, c),
    ensures
        term_lt(a, c),
    decreases a,
{
    reveal(rank);
    reveal_with_fuel(term_lt, 2);
    if rank(a) == rank(b) && rank(b) == rank(c) {
        match (a, b, c) {
            (Term::Atom(x), Term::Atom(y), Term::Atom(z)) => {
                bytes_lt_trans(x, y, z);
            },
            (Term::Comp(n, xs), Term::Comp(m, ys), Term::Comp(o, zs)) => {
                if xs.len() == ys.len() && ys.len() == zs.len() {
                    if n == m && m == o {
                        args_lt_trans(xs, ys, zs);
                    } else if n != m && m != o {
                        bytes_lt_trans(n, m, o);
                        if n == o {
                            crate::k2_term::term_lt_irreflexive(Term::Atom(n));
                        }
                    }
                }
            },
            _ => {},
        }
    }
}

proof fn args_lt_trans(xs: Seq<Term>, ys: Seq<Term>, zs: Seq<Term>)
    requires
        xs.len() == ys.len(),
        ys.len() == zs.len(),
        args_lt(xs, ys),
        args_lt(ys, zs),
    ensures
        args_lt(xs, zs),
    decreases xs,
{
    reveal_with_fuel(args_lt, 2);
    if xs[0] != ys[0] {
        if ys[0] != zs[0] {
            term_lt_trans(xs[0], ys[0], zs[0]);
            if xs[0] == zs[0] {
                crate::k2_term::term_lt_irreflexive(xs[0]);
            }
        }
    } else if ys[0] == zs[0] {
        args_lt_trans(xs.drop_first(), ys.drop_first(), zs.drop_first());
    }
}

pub open spec fn le(a: Term, b: Term) -> bool {
    a == b || term_lt(a, b)
}

pub open spec fn sorted(s: Seq<Term>) -> bool {
    forall|i: int, j: int| 0 <= i < j < s.len() ==> le(#[trigger] s[i], #[trigger] s[j])
}

pub open spec fn strict(s: Seq<Term>) -> bool {
    forall|i: int, j: int| 0 <= i < j < s.len() ==> term_lt(#[trigger] s[i], #[trigger] s[j])
}

proof fn le_trans(a: Term, b: Term, c: Term)
    requires
        le(a, b),
        le(b, c),
    ensures
        le(a, c),
{
    if a != b && b != c {
        term_lt_trans(a, b, c);
    }
}

proof fn lt_le_trans(a: Term, b: Term, c: Term)
    requires
        term_lt(a, b),
        le(b, c),
    ensures
        term_lt(a, c),
{
    if b != c {
        term_lt_trans(a, b, c);
    }
}

proof fn sorted_rest(s: Seq<Term>)
    requires
        sorted(s),
        s.len() > 0,
    ensures
        sorted(s.drop_first()),
{
    let rest = s.drop_first();
    assert forall|i: int, j: int| 0 <= i < j < rest.len() implies le(
        #[trigger] rest[i],
        #[trigger] rest[j],
    ) by {
        assert(rest[i] == s[i + 1] && rest[j] == s[j + 1]);
    }
}

proof fn insert_sorted_sorted(x: Term, s: Seq<Term>)
    requires
        sorted(s),
    ensures
        sorted(insert_sorted(x, s)),
    decreases s.len(),
{
    reveal_with_fuel(insert_sorted, 1);
    let r = insert_sorted(x, s);
    if s.len() > 0 && term_lt(s[0], x) {
        let rest = s.drop_first();
        let s0 = s[0];
        sorted_rest(s);
        insert_sorted_sorted(x, rest);
        assert forall|k: int| 0 <= k < rest.len() implies le(s0, #[trigger] rest[k]) by {
            assert(rest[k] == s[k + 1]);
            assert(le(s[0], s[k + 1]));
        }
        let p = |t: Term| le(s0, t);
        assert(p(x));
        assert forall|k: int| 0 <= k < rest.len() implies p(#[trigger] rest[k]) by {
            assert(le(s0, rest[k]));
        }
        crate::k2_sort::insert_property(x, rest, p);
        let tail = insert_sorted(x, rest);
        assert(r == seq![s0] + tail);
        assert forall|i: int, j: int| 0 <= i < j < r.len() implies le(
            #[trigger] r[i],
            #[trigger] r[j],
        ) by {
            assert(r[j] == tail[j - 1]);
            if i == 0 {
                assert(p(tail[j - 1]));
            } else {
                assert(r[i] == tail[i - 1]);
            }
        }
    } else {
        assert(r == seq![x] + s);
        assert forall|i: int, j: int| 0 <= i < j < r.len() implies le(
            #[trigger] r[i],
            #[trigger] r[j],
        ) by {
            assert(r[j] == s[j - 1]);
            if i == 0 {
                crate::k2_sort::term_order_total(x, s[0]);
                assert(le(x, s[0]));
                if j - 1 > 0 {
                    assert(le(s[0], s[j - 1]));
                    le_trans(x, s[0], s[j - 1]);
                }
            } else {
                assert(r[i] == s[i - 1]);
            }
        }
    }
}

proof fn msort_sorted(s: Seq<Term>)
    ensures
        sorted(msort(s)),
    decreases s.len(),
{
    reveal_with_fuel(msort, 1);
    if s.len() > 0 {
        msort_sorted(s.drop_first());
        insert_sorted_sorted(s[0], msort(s.drop_first()));
    }
}

proof fn dedup_strict(s: Seq<Term>)
    requires
        sorted(s),
    ensures
        strict(dedup(s)),
    decreases s.len(),
{
    reveal_with_fuel(dedup, 1);
    let d = dedup(s);
    if s.len() >= 2 {
        let rest = s.drop_first();
        sorted_rest(s);
        dedup_strict(rest);
        if s[0] != s[1] {
            let s0 = s[0];
            assert(le(s[0], s[1]));
            assert(term_lt(s0, s[1]));
            let p = |t: Term| term_lt(s0, t);
            assert forall|k: int| 0 <= k < rest.len() implies p(#[trigger] rest[k]) by {
                assert(rest[k] == s[k + 1]);
                if k > 0 {
                    assert(le(s[1], s[k + 1]));
                    lt_le_trans(s0, s[1], s[k + 1]);
                }
            }
            crate::k2_sort::dedup_property(rest, p);
            let tail = dedup(rest);
            assert(d == seq![s0] + tail);
            assert forall|i: int, j: int| 0 <= i < j < d.len() implies term_lt(
                #[trigger] d[i],
                #[trigger] d[j],
            ) by {
                assert(d[j] == tail[j - 1]);
                if i == 0 {
                    assert(p(tail[j - 1]));
                } else {
                    assert(d[i] == tail[i - 1]);
                }
            }
        }
    }
}

proof fn cons_contains(a: Term, s: Seq<Term>, t: Term)
    ensures
        (seq![a] + s).contains(t) == (t == a || s.contains(t)),
{
    let r = seq![a] + s;
    if r.contains(t) {
        let i = choose|i: int| 0 <= i < r.len() && r[i] == t;
        if i > 0 {
            assert(s[i - 1] == t);
        }
    }
    if t == a {
        assert(r[0] == t);
    }
    if s.contains(t) {
        let i = choose|i: int| 0 <= i < s.len() && s[i] == t;
        assert(r[i + 1] == t);
    }
}

proof fn insert_contains(x: Term, s: Seq<Term>, t: Term)
    ensures
        insert_sorted(x, s).contains(t) == (t == x || s.contains(t)),
    decreases s.len(),
{
    reveal_with_fuel(insert_sorted, 1);
    if s.len() > 0 && term_lt(s[0], x) {
        let rest = s.drop_first();
        insert_contains(x, rest, t);
        cons_contains(s[0], insert_sorted(x, rest), t);
        cons_contains(s[0], rest, t);
        assert(s =~= seq![s[0]] + rest);
    } else {
        cons_contains(x, s, t);
    }
}

proof fn msort_contains(s: Seq<Term>, t: Term)
    ensures
        msort(s).contains(t) == s.contains(t),
    decreases s.len(),
{
    reveal_with_fuel(msort, 1);
    if s.len() > 0 {
        let rest = s.drop_first();
        msort_contains(rest, t);
        insert_contains(s[0], msort(rest), t);
        cons_contains(s[0], rest, t);
        assert(s =~= seq![s[0]] + rest);
    }
}

proof fn dedup_contains(s: Seq<Term>, t: Term)
    ensures
        dedup(s).contains(t) == s.contains(t),
    decreases s.len(),
{
    reveal_with_fuel(dedup, 1);
    if s.len() >= 2 {
        let rest = s.drop_first();
        dedup_contains(rest, t);
        cons_contains(s[0], rest, t);
        assert(s =~= seq![s[0]] + rest);
        if s[0] == s[1] {
            assert(rest[0] == s[1]);
        } else {
            cons_contains(s[0], dedup(rest), t);
        }
    }
}

pub proof fn sort_unique_canonical(s: Seq<Term>)
    ensures
        forall|i: int, j: int|
            0 <= i < j < sort_unique(s).len() ==> term_lt(
                #[trigger] sort_unique(s)[i],
                #[trigger] sort_unique(s)[j],
            ),
        forall|t: Term| #[trigger] sort_unique(s).contains(t) == s.contains(t),
{
    reveal(sort_unique);
    msort_sorted(s);
    dedup_strict(msort(s));
    assert forall|t: Term| #[trigger] sort_unique(s).contains(t) == s.contains(t) by {
        msort_contains(s, t);
        dedup_contains(msort(s), t);
    }
}

} // verus!
