use crate::k2_term::{term_lt, ETermArena};
#[cfg(verus_keep_ghost)]
use crate::k2_term::{arena_ok, root_ok, term_lt_irreflexive};
#[cfg(verus_keep_ghost)]
use ckc_spec::term::Term;
#[cfg(verus_keep_ghost)]
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub open spec fn root_terms(arena: &ETermArena, roots: Seq<usize>) -> Seq<Term> {
    roots.map_values(|root: usize| arena@[root as int])
}

pub open spec fn roots_ok(arena: &ETermArena, roots: Seq<usize>) -> bool {
    arena_ok(arena)
        && forall|i: int| 0 <= i < roots.len() ==> roots[i] < arena.nodes@.len()
}

pub open spec fn roots_ground(arena: &ETermArena, roots: Seq<usize>) -> bool {
    forall|i: int| 0 <= i < roots.len()
        ==> ckc_spec::term::ground(arena@[roots[i] as int])
}

proof fn roots_ok_index(arena: &ETermArena, roots: Seq<usize>, i: int)
    requires
        roots_ok(arena, roots),
        0 <= i < roots.len(),
    ensures root_ok(arena, roots[i]),
{
    reveal(roots_ok);
    reveal(root_ok);
}

proof fn roots_ground_index(arena: &ETermArena, roots: Seq<usize>, i: int)
    requires
        roots_ground(arena, roots),
        0 <= i < roots.len(),
    ensures ckc_spec::term::ground(arena@[roots[i] as int]),
{
    reveal(roots_ground);
}

proof fn root_terms_index(arena: &ETermArena, roots: Seq<usize>, i: int)
    requires 0 <= i < roots.len(),
    ensures root_terms(arena, roots)[i] == arena@[roots[i] as int],
{
    reveal(root_terms);
}

proof fn root_terms_subrange(
    arena: &ETermArena,
    roots: Seq<usize>,
    start: int,
    end: int,
)
    requires 0 <= start <= end <= roots.len(),
    ensures
        root_terms(arena, roots.subrange(start, end))
            == root_terms(arena, roots).subrange(start, end),
{
    reveal(root_terms);
    assert_seqs_equal!(
        root_terms(arena, roots.subrange(start, end))
            == root_terms(arena, roots).subrange(start, end)
    );
}

proof fn root_terms_insert(
    arena: &ETermArena,
    roots: Seq<usize>,
    i: int,
    root: usize,
)
    requires 0 <= i <= roots.len(),
    ensures
        root_terms(arena, roots.insert(i, root))
            == root_terms(arena, roots).insert(i, arena@[root as int]),
{
    reveal(root_terms);
    assert_seqs_equal!(
        root_terms(arena, roots.insert(i, root))
            == root_terms(arena, roots).insert(i, arena@[root as int])
    );
}

proof fn roots_ok_insert(
    arena: &ETermArena,
    roots: Seq<usize>,
    i: int,
    root: usize,
)
    requires
        roots_ok(arena, roots),
        root_ok(arena, root),
        0 <= i <= roots.len(),
    ensures roots_ok(arena, roots.insert(i, root)),
{
    reveal(roots_ok);
    reveal(root_ok);
    assert forall|j: int| 0 <= j < roots.insert(i, root).len()
        implies roots.insert(i, root)[j] < arena.nodes@.len() by {
        if j < i {
            assert(roots.insert(i, root)[j] == roots[j]);
        } else if j == i {
            assert(roots.insert(i, root)[j] == root);
        } else {
            assert(roots.insert(i, root)[j] == roots[j - 1]);
        }
    }
}

proof fn roots_ground_insert(
    arena: &ETermArena,
    roots: Seq<usize>,
    i: int,
    root: usize,
)
    requires
        roots_ground(arena, roots),
        ckc_spec::term::ground(arena@[root as int]),
        0 <= i <= roots.len(),
    ensures roots_ground(arena, roots.insert(i, root)),
{
    reveal(roots_ground);
    assert forall|j: int| 0 <= j < roots.insert(i, root).len()
        implies ckc_spec::term::ground(arena@[roots.insert(i, root)[j] as int]) by {
        if j < i {
            assert(roots.insert(i, root)[j] == roots[j]);
        } else if j == i {
            assert(roots.insert(i, root)[j] == root);
        } else {
            assert(roots.insert(i, root)[j] == roots[j - 1]);
        }
    }
}

proof fn bytes_order_total(a: Seq<u8>, b: Seq<u8>)
    ensures
        a == b
            || ckc_spec::engine::bytes_lt(a, b)
            || ckc_spec::engine::bytes_lt(b, a),
    decreases a.len() + b.len(),
{
    reveal_with_fuel(ckc_spec::engine::bytes_lt, 2);
    if a.len() == 0 {
    } else if b.len() == 0 {
    } else if a[0] == b[0] {
        bytes_order_total(a.drop_first(), b.drop_first());
        if a.drop_first() == b.drop_first() {
            assert_seqs_equal!(a == seq![a[0]] + a.drop_first());
            assert_seqs_equal!(b == seq![b[0]] + b.drop_first());
            assert_seqs_equal!(a == b);
        }
    } else if a[0] < b[0] {
    } else {
        assert(a[0] > b[0]);
    }
}

proof fn term_order_total(a: Term, b: Term)
    ensures
        a == b
            || ckc_spec::engine::term_lt(a, b)
            || ckc_spec::engine::term_lt(b, a),
    decreases a,
{
    reveal(ckc_spec::engine::rank);
    reveal_with_fuel(ckc_spec::engine::term_lt, 2);
    if ckc_spec::engine::rank(a) != ckc_spec::engine::rank(b) {
    } else {
        match (a, b) {
            (Term::Atom(x), Term::Atom(y)) => {
                bytes_order_total(x, y);
            },
            (Term::Comp(n, xs), Term::Comp(m, ys)) => {
                if xs.len() == ys.len() && n == m {
                    args_order_total(xs, ys);
                } else if xs.len() == ys.len() {
                    bytes_order_total(n, m);
                }
            },
            _ => {},
        }
    }
}

proof fn args_order_total(xs: Seq<Term>, ys: Seq<Term>)
    requires xs.len() == ys.len(),
    ensures
        xs == ys
            || ckc_spec::engine::args_lt(xs, ys)
            || ckc_spec::engine::args_lt(ys, xs),
    decreases xs,
{
    reveal_with_fuel(ckc_spec::engine::args_lt, 2);
    if xs.len() == 0 {
        assert_seqs_equal!(xs == ys);
    } else if xs[0] == ys[0] {
        args_order_total(xs.drop_first(), ys.drop_first());
        if xs.drop_first() == ys.drop_first() {
            assert_seqs_equal!(xs == seq![xs[0]] + xs.drop_first());
            assert_seqs_equal!(ys == seq![ys[0]] + ys.drop_first());
            assert_seqs_equal!(xs == ys);
        }
    } else {
        term_order_total(xs[0], ys[0]);
    }
}

pub fn term_equal(arena: &ETermArena, left: usize, right: usize) -> (equal: bool)
    requires
        root_ok(arena, left),
        root_ok(arena, right),
        ckc_spec::term::ground(arena@[left as int]),
        ckc_spec::term::ground(arena@[right as int]),
    ensures equal == (arena@[left as int] == arena@[right as int]),
{
    let less = term_lt(arena, left, right);
    if less {
        proof {
            if arena@[left as int] == arena@[right as int] {
                term_lt_irreflexive(arena@[left as int]);
            }
        }
        false
    } else {
        let greater = term_lt(arena, right, left);
        if greater {
            proof {
                if arena@[left as int] == arena@[right as int] {
                    term_lt_irreflexive(arena@[left as int]);
                }
            }
            false
        } else {
            proof {
                term_order_total(arena@[left as int], arena@[right as int]);
            }
            true
        }
    }
}

proof fn insert_sorted_at(x: Term, s: Seq<Term>, i: nat)
    requires
        i <= s.len(),
        forall|j: int| 0 <= j < i ==> ckc_spec::engine::term_lt(s[j], x),
        i == s.len() || !ckc_spec::engine::term_lt(s[i as int], x),
    ensures
        ckc_spec::engine::insert_sorted(x, s) == s.insert(i as int, x),
    decreases i,
{
    reveal_with_fuel(ckc_spec::engine::insert_sorted, 2);
    if i == 0 {
        assert_seqs_equal!(s.insert(0, x) == seq![x] + s);
    } else {
        assert(s.len() > 0);
        assert(ckc_spec::engine::term_lt(s[0], x));
        assert forall|j: int| 0 <= j < i - 1
            implies ckc_spec::engine::term_lt(s.drop_first()[j], x) by {
            assert(s.drop_first()[j] == s[j + 1]);
        }
        if i < s.len() {
            assert(s.drop_first()[(i - 1) as int] == s[i as int]);
        }
        insert_sorted_at(x, s.drop_first(), (i - 1) as nat);
        assert_seqs_equal!(
            s.insert(i as int, x)
                == seq![s[0]] + s.drop_first().insert((i - 1) as int, x)
        );
    }
}

#[verifier::rlimit(5000)]
fn insert_root(arena: &ETermArena, root: usize, sorted: &mut Vec<usize>)
    requires
        root_ok(arena, root),
        ckc_spec::term::ground(arena@[root as int]),
        roots_ok(arena, old(sorted)@),
        roots_ground(arena, old(sorted)@),
    ensures
        roots_ok(arena, final(sorted)@),
        roots_ground(arena, final(sorted)@),
        root_terms(arena, final(sorted)@)
            == ckc_spec::engine::insert_sorted(
                arena@[root as int],
                root_terms(arena, old(sorted)@),
            ),
{
    let ghost before = sorted@;
    let mut i = 0usize;
    let mut stopped = false;
    while i < sorted.len() && !stopped
        invariant
            sorted@ == before,
            roots_ok(arena, sorted@),
            roots_ground(arena, sorted@),
            root_ok(arena, root),
            ckc_spec::term::ground(arena@[root as int]),
            i <= sorted@.len(),
            stopped ==> i < sorted@.len()
                && !ckc_spec::engine::term_lt(
                    root_terms(arena, sorted@)[i as int],
                    arena@[root as int],
                ),
            forall|j: int| 0 <= j < i
                ==> ckc_spec::engine::term_lt(
                    root_terms(arena, sorted@)[j],
                    arena@[root as int],
                ),
        decreases sorted.len() - i, if stopped { 0int } else { 1int },
    {
        proof {
            roots_ok_index(arena, sorted@, i as int);
            roots_ground_index(arena, sorted@, i as int);
            root_terms_index(arena, sorted@, i as int);
        }
        if term_lt(arena, sorted[i], root) {
            i += 1;
        } else {
            stopped = true;
        }
    }
    proof {
        if i < sorted@.len() {
            root_terms_index(arena, sorted@, i as int);
        }
        insert_sorted_at(
            arena@[root as int],
            root_terms(arena, before),
            i as nat,
        );
        root_terms_insert(arena, before, i as int, root);
        roots_ok_insert(arena, before, i as int, root);
        roots_ground_insert(arena, before, i as int, root);
    }
    sorted.insert(i, root);
}

#[verifier::rlimit(5000)]
pub fn msort(arena: &ETermArena, roots: &Vec<usize>) -> (out: Vec<usize>)
    requires
        roots_ok(arena, roots@),
        roots_ground(arena, roots@),
    ensures
        roots_ok(arena, out@),
        roots_ground(arena, out@),
        root_terms(arena, out@)
            == ckc_spec::engine::msort(root_terms(arena, roots@)),
{
    let mut out = Vec::new();
    let mut i = roots.len();
    proof {
        root_terms_subrange(arena, roots@, i as int, roots@.len() as int);
        reveal_with_fuel(ckc_spec::engine::msort, 2);
        reveal(roots_ok);
        reveal(roots_ground);
    }
    while i > 0
        invariant
            roots_ok(arena, roots@),
            roots_ground(arena, roots@),
            roots_ok(arena, out@),
            roots_ground(arena, out@),
            i <= roots@.len(),
            root_terms(arena, out@)
                == ckc_spec::engine::msort(
                    root_terms(arena, roots@).subrange(i as int, roots@.len() as int),
                ),
        decreases i,
    {
        i -= 1;
        proof {
            roots_ok_index(arena, roots@, i as int);
            roots_ground_index(arena, roots@, i as int);
            root_terms_index(arena, roots@, i as int);
        }
        insert_root(arena, roots[i], &mut out);
        proof {
            root_terms_subrange(arena, roots@, i as int, roots@.len() as int);
            root_terms_subrange(arena, roots@, i as int + 1, roots@.len() as int);
            assert(
                root_terms(arena, roots@).subrange(i as int, roots@.len() as int)[0]
                    == root_terms(arena, roots@)[i as int]
            );
            assert_seqs_equal!(
                root_terms(arena, roots@).subrange(i as int, roots@.len() as int).drop_first()
                    == root_terms(arena, roots@).subrange(
                        i as int + 1,
                        roots@.len() as int,
                    )
            );
            reveal_with_fuel(ckc_spec::engine::msort, 2);
        }
    }
    proof {
        assert_seqs_equal!(
            root_terms(arena, roots@).subrange(0, roots@.len() as int)
                == root_terms(arena, roots@)
        );
    }
    out
}

proof fn dedup_step(x: Term, tail: Seq<Term>)
    ensures
        ckc_spec::engine::dedup(seq![x] + tail)
            == if tail.len() == 0 {
                seq![x]
            } else if x == tail[0] {
                ckc_spec::engine::dedup(tail)
            } else {
                seq![x] + ckc_spec::engine::dedup(tail)
            },
{
    reveal_with_fuel(ckc_spec::engine::dedup, 2);
    if tail.len() > 0 {
        assert((seq![x] + tail).drop_first() == tail);
    }
}

#[verifier::rlimit(5000)]
fn dedup_roots(arena: &ETermArena, sorted: &Vec<usize>) -> (out: Vec<usize>)
    requires
        roots_ok(arena, sorted@),
        roots_ground(arena, sorted@),
    ensures
        roots_ok(arena, out@),
        roots_ground(arena, out@),
        root_terms(arena, out@)
            == ckc_spec::engine::dedup(root_terms(arena, sorted@)),
{
    let mut out = Vec::new();
    let mut i = sorted.len();
    proof {
        root_terms_subrange(arena, sorted@, i as int, sorted@.len() as int);
        reveal_with_fuel(ckc_spec::engine::dedup, 2);
        reveal(roots_ok);
        reveal(roots_ground);
    }
    while i > 0
        invariant
            roots_ok(arena, sorted@),
            roots_ground(arena, sorted@),
            roots_ok(arena, out@),
            roots_ground(arena, out@),
            i <= sorted@.len(),
            root_terms(arena, out@)
                == ckc_spec::engine::dedup(
                    root_terms(arena, sorted@).subrange(i as int, sorted@.len() as int),
                ),
        decreases i,
    {
        i -= 1;
        proof {
            roots_ok_index(arena, sorted@, i as int);
            roots_ground_index(arena, sorted@, i as int);
            root_terms_index(arena, sorted@, i as int);
            root_terms_subrange(arena, sorted@, i as int, sorted@.len() as int);
            root_terms_subrange(arena, sorted@, i as int + 1, sorted@.len() as int);
        }
        let keep = if i + 1 == sorted.len() {
            true
        } else {
            proof {
                roots_ok_index(arena, sorted@, i as int + 1);
                roots_ground_index(arena, sorted@, i as int + 1);
            }
            !term_equal(arena, sorted[i], sorted[i + 1])
        };
        if keep {
            let ghost before = out@;
            proof {
                root_terms_insert(arena, before, 0, sorted@[i as int]);
                roots_ok_insert(arena, before, 0, sorted@[i as int]);
                roots_ground_insert(arena, before, 0, sorted@[i as int]);
            }
            out.insert(0, sorted[i]);
        }
        proof {
            let suffix = root_terms(arena, sorted@).subrange(
                i as int + 1,
                sorted@.len() as int,
            );
            assert(
                root_terms(arena, sorted@).subrange(i as int, sorted@.len() as int)[0]
                    == root_terms(arena, sorted@)[i as int]
            );
            assert_seqs_equal!(
                root_terms(arena, sorted@).subrange(i as int, sorted@.len() as int)
                    == seq![root_terms(arena, sorted@)[i as int]] + suffix
            );
            dedup_step(root_terms(arena, sorted@)[i as int], suffix);
            if suffix.len() > 0 {
                assert(suffix[0] == root_terms(arena, sorted@)[i as int + 1]);
            }
        }
    }
    proof {
        assert_seqs_equal!(
            root_terms(arena, sorted@).subrange(0, sorted@.len() as int)
                == root_terms(arena, sorted@)
        );
    }
    out
}

#[verifier::rlimit(5000)]
pub fn sort_unique(arena: &ETermArena, roots: &Vec<usize>) -> (out: Vec<usize>)
    requires
        roots_ok(arena, roots@),
        roots_ground(arena, roots@),
    ensures
        roots_ok(arena, out@),
        roots_ground(arena, out@),
        root_terms(arena, out@)
            == ckc_spec::engine::sort_unique(root_terms(arena, roots@)),
{
    let sorted = msort(arena, roots);
    let out = dedup_roots(arena, &sorted);
    proof { reveal(ckc_spec::engine::sort_unique); }
    out
}

proof fn insert_property(x: Term, terms: Seq<Term>, p: spec_fn(Term) -> bool)
    requires p(x), forall|i: int| 0 <= i < terms.len() ==> p(#[trigger] terms[i]),
    ensures forall|i: int| 0 <= i < ckc_spec::engine::insert_sorted(x, terms).len()
        ==> p(#[trigger] ckc_spec::engine::insert_sorted(x, terms)[i]),
    decreases terms.len(),
{
    reveal_with_fuel(ckc_spec::engine::insert_sorted, 1);
    if terms.len() > 0 && ckc_spec::engine::term_lt(terms[0], x) {
        insert_property(x, terms.drop_first(), p);
    }
    assert forall|i: int| 0 <= i < ckc_spec::engine::insert_sorted(x, terms).len()
        implies p(#[trigger] ckc_spec::engine::insert_sorted(x, terms)[i]) by {
        if terms.len() > 0 && ckc_spec::engine::term_lt(terms[0], x) {
            if i == 0 { assert(ckc_spec::engine::insert_sorted(x, terms)[i] == terms[0]); }
            else { assert(ckc_spec::engine::insert_sorted(x, terms)[i]
                == ckc_spec::engine::insert_sorted(x, terms.drop_first())[i - 1]); }
        } else {
            if i == 0 { assert(ckc_spec::engine::insert_sorted(x, terms)[i] == x); }
            else { assert(ckc_spec::engine::insert_sorted(x, terms)[i] == terms[i - 1]); }
        }
    }
}

proof fn msort_property(terms: Seq<Term>, p: spec_fn(Term) -> bool)
    requires forall|i: int| 0 <= i < terms.len() ==> p(#[trigger] terms[i]),
    ensures forall|i: int| 0 <= i < ckc_spec::engine::msort(terms).len()
        ==> p(#[trigger] ckc_spec::engine::msort(terms)[i]),
    decreases terms.len(),
{
    reveal_with_fuel(ckc_spec::engine::msort, 1);
    if terms.len() > 0 {
        msort_property(terms.drop_first(), p);
        insert_property(terms[0], ckc_spec::engine::msort(terms.drop_first()), p);
    }
}

proof fn dedup_property(terms: Seq<Term>, p: spec_fn(Term) -> bool)
    requires forall|i: int| 0 <= i < terms.len() ==> p(#[trigger] terms[i]),
    ensures forall|i: int| 0 <= i < ckc_spec::engine::dedup(terms).len()
        ==> p(#[trigger] ckc_spec::engine::dedup(terms)[i]),
    decreases terms.len(),
{
    reveal_with_fuel(ckc_spec::engine::dedup, 1);
    if terms.len() >= 2 { dedup_property(terms.drop_first(), p); }
    assert forall|i: int| 0 <= i < ckc_spec::engine::dedup(terms).len()
        implies p(#[trigger] ckc_spec::engine::dedup(terms)[i]) by {
        if terms.len() < 2 { assert(ckc_spec::engine::dedup(terms)[i] == terms[i]); }
        else if terms[0] == terms[1] {
            assert(ckc_spec::engine::dedup(terms)[i] == ckc_spec::engine::dedup(terms.drop_first())[i]);
        } else if i == 0 { assert(ckc_spec::engine::dedup(terms)[i] == terms[0]); }
        else { assert(ckc_spec::engine::dedup(terms)[i] == ckc_spec::engine::dedup(terms.drop_first())[i - 1]); }
    }
}

pub proof fn sort_unique_property(terms: Seq<Term>, p: spec_fn(Term) -> bool)
    requires forall|i: int| 0 <= i < terms.len() ==> p(#[trigger] terms[i]),
    ensures forall|i: int| 0 <= i < ckc_spec::engine::sort_unique(terms).len()
        ==> p(#[trigger] ckc_spec::engine::sort_unique(terms)[i]),
{
    msort_property(terms, p);
    dedup_property(ckc_spec::engine::msort(terms), p);
}

} // verus!
