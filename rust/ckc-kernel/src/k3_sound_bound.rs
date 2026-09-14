use super::freshness::*;
use super::unification::*;
use super::*;

verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub open spec fn pair_below(p: (Term, Term), b: nat) -> bool {
    nvars(p.0) <= b && nvars(p.1) <= b
}

pub open spec fn pairs_below(ps: Seq<(Term, Term)>, b: nat) -> bool {
    forall|i: int| 0 <= i < ps.len() ==> #[trigger] pair_below(ps[i], b)
}

pub open spec fn bounded_witness(u: UState, out: UOut, b: nat, s: Seq<(nat, Term)>) -> bool {
    unifier_witness(u, out, s) && binds_below(s, b)
}

pub proof fn pairs_tail(ps: Seq<(Term, Term)>, b: nat)
    requires
        ps.len() > 0,
        pairs_below(ps, b),
    ensures
        pairs_below(ps.drop_first(), b),
{
    assert forall|i: int| 0 <= i < ps.drop_first().len() implies #[trigger] pair_below(
        ps.drop_first()[i],
        b,
    ) by {
        assert(ps.drop_first()[i] == ps[i + 1]);
    }
}

pub proof fn pairs_bind(ps: Seq<(Term, Term)>, b: nat, x: nat, v: Term)
    requires
        pairs_below(ps, b),
        nvars(v) <= b,
    ensures
        pairs_below(ps.map_values(|p: (Term, Term)| (subst(p.0, x, v), subst(p.1, x, v))), b),
{
    let mapped = ps.map_values(|p: (Term, Term)| (subst(p.0, x, v), subst(p.1, x, v)));
    assert forall|i: int| 0 <= i < mapped.len() implies #[trigger] pair_below(mapped[i], b) by {
        assert(pair_below(ps[i], b));
        subst_below(ps[i].0, x, v, b);
        subst_below(ps[i].1, x, v, b);
    }
}

pub proof fn pairs_decomp(xs: Seq<Term>, ys: Seq<Term>, rest: Seq<(Term, Term)>, b: nat)
    requires
        xs.len() == ys.len(),
        nvars_all(xs) <= b,
        nvars_all(ys) <= b,
        pairs_below(rest, b),
    ensures
        pairs_below(zip(xs, ys) + rest, b),
{
    assert forall|i: int| 0 <= i < (zip(xs, ys) + rest).len() implies #[trigger] pair_below(
        (zip(xs, ys) + rest)[i],
        b,
    ) by {
        if i < xs.len() {
            nvars_all_index(xs, i);
            nvars_all_index(ys, i);
            assert((zip(xs, ys) + rest)[i] == (xs[i], ys[i]));
        } else {
            assert((zip(xs, ys) + rest)[i] == rest[i - xs.len()]);
        }
    }
}

pub proof fn lift_bounded_binding(
    u: UState,
    out: UOut,
    b: nat,
    x: nat,
    v: Term,
    s: Seq<(nat, Term)>,
)
    requires
        u.pairs.len() > 0,
        !occurs(x, v),
        x < b,
        nvars(v) <= b,
        u.pairs[0] == (Term::Var(x), v) || u.pairs[0] == (v, Term::Var(x)),
        bounded_witness(u_bind(u, u.pairs.drop_first(), x, v), out, b, s),
    ensures
        bounded_witness(u, out, b, seq![(x, v)] + s),
{
    lift_binding(u, out, x, v, s);
    assert(binds_below(seq![(x, v)], b));
    binds_append(seq![(x, v)], s, b);
}

pub proof fn unify_n_bounded(u: UState, fuel: nat, bnd: nat)
    requires
        unify_n(u, fuel) is Ok,
        pairs_below(u.pairs, bnd),
    ensures
        exists|s: Seq<(nat, Term)>| #[trigger] bounded_witness(u, unify_n(u, fuel), bnd, s),
    decreases fuel,
{
    if fuel > 0 {
        if u.pairs.len() == 0 {
            let s = Seq::<(nat, Term)>::empty();
            assert(apply_terms(u.sol, s) =~= u.sol);
            assert(bounded_witness(u, unify_n(u, fuel), bnd, s));
        } else {
            let a = u.pairs[0].0;
            let b = u.pairs[0].1;
            let rest = u.pairs.drop_first();
            let f = (fuel - 1) as nat;
            assert(pair_below(u.pairs[0], bnd));
            pairs_tail(u.pairs, bnd);
            match (a, b) {
                (Term::Var(x), _) => {
                    if a == b {
                        let nu = UState { pairs: rest, ..u };
                        unify_n_bounded(nu, f, bnd);
                        let s = choose|s: Seq<(nat, Term)>|
                            bounded_witness(nu, unify_n(nu, f), bnd, s);
                        solves_join(u.pairs, s);
                        assert(bounded_witness(u, unify_n(u, fuel), bnd, s));
                    } else if !occurs(x, b) {
                        let nu = u_bind(u, rest, x, b);
                        pairs_bind(rest, bnd, x, b);
                        unify_n_bounded(nu, f, bnd);
                        let s = choose|s: Seq<(nat, Term)>|
                            bounded_witness(nu, unify_n(nu, f), bnd, s);
                        lift_bounded_binding(u, unify_n(nu, f), bnd, x, b, s);
                        assert(bounded_witness(u, unify_n(u, fuel), bnd, seq![(x, b)] + s));
                    }
                },
                (_, Term::Var(y)) => {
                    if !occurs(y, a) {
                        let nu = u_bind(u, rest, y, a);
                        pairs_bind(rest, bnd, y, a);
                        unify_n_bounded(nu, f, bnd);
                        let s = choose|s: Seq<(nat, Term)>|
                            bounded_witness(nu, unify_n(nu, f), bnd, s);
                        lift_bounded_binding(u, unify_n(nu, f), bnd, y, a, s);
                        assert(bounded_witness(u, unify_n(u, fuel), bnd, seq![(y, a)] + s));
                    }
                },
                (Term::Comp(n, xs), Term::Comp(m, ys)) => {
                    if n == m && xs.len() == ys.len() {
                        let nu = UState { pairs: zip(xs, ys) + rest, ..u };
                        pairs_decomp(xs, ys, rest, bnd);
                        unify_n_bounded(nu, f, bnd);
                        let s = choose|s: Seq<(nat, Term)>|
                            bounded_witness(nu, unify_n(nu, f), bnd, s);
                        solves_comp(n, xs, ys, rest, s);
                        solves_tail(zip(xs, ys), rest, s);
                        solves_join(u.pairs, s);
                        assert(bounded_witness(u, unify_n(u, fuel), bnd, s));
                    }
                },
                _ => {
                    if a == b {
                        let nu = UState { pairs: rest, ..u };
                        unify_n_bounded(nu, f, bnd);
                        let s = choose|s: Seq<(nat, Term)>|
                            bounded_witness(nu, unify_n(nu, f), bnd, s);
                        solves_join(u.pairs, s);
                        assert(bounded_witness(u, unify_n(u, fuel), bnd, s));
                    }
                },
            }
        }
    }
}

pub proof fn unify_bounded(u: UState, b: nat)
    requires
        unify(u) is Ok,
        pairs_below(u.pairs, b),
    ensures
        exists|s: Seq<(nat, Term)>| #[trigger] bounded_witness(u, unify(u), b, s),
{
    let f = choose|f: nat| !(unify_n(u, f) is Out);
    unify_n_bounded(u, f, b);
}

} // verus!
