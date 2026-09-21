use super::*;

verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub open spec fn apply_terms(ts: Seq<Term>, s: Seq<(nat, Term)>) -> Seq<Term> {
    ts.map_values(|t: Term| apply(t, s))
}

pub proof fn subst_absent(t: Term, x: nat, v: Term)
    requires
        !occurs(x, t),
    ensures
        subst(t, x, v) == t,
    decreases t,
{
    if let Term::Comp(_, args) = t {
        subst_all_absent(args, x, v);
    }
}

pub proof fn subst_all_absent(ts: Seq<Term>, x: nat, v: Term)
    requires
        !occurs_all(x, ts),
    ensures
        subst_all(ts, x, v) == ts,
    decreases ts,
{
    if ts.len() > 0 {
        subst_absent(ts[0], x, v);
        subst_all_absent(ts.drop_first(), x, v);
        assert(seq![ts[0]] + ts.drop_first() =~= ts);
    }
}

pub proof fn subst_all_map(ts: Seq<Term>, x: nat, v: Term)
    ensures
        subst_all(ts, x, v) == ts.map_values(|t: Term| subst(t, x, v)),
    decreases ts.len(),
{
    if ts.len() > 0 {
        subst_all_map(ts.drop_first(), x, v);
    }
    assert(subst_all(ts, x, v) =~= ts.map_values(|t: Term| subst(t, x, v)));
}

pub proof fn apply_terms_cons(ts: Seq<Term>, x: nat, v: Term, s: Seq<(nat, Term)>)
    ensures
        apply_terms(ts, seq![(x, v)] + s) == apply_terms(subst_all(ts, x, v), s),
{
    subst_all_map(ts, x, v);
    assert forall|i: int| 0 <= i < ts.len() implies #[trigger] apply_terms(ts, seq![(x, v)] + s)[i]
        == apply_terms(subst_all(ts, x, v), s)[i] by {
        apply_cons(ts[i], x, v, s);
    }
    assert(apply_terms(ts, seq![(x, v)] + s) =~= apply_terms(subst_all(ts, x, v), s));
}

pub proof fn apply_comp(name: Seq<u8>, ts: Seq<Term>, s: Seq<(nat, Term)>)
    ensures
        apply(Term::Comp(name, ts), s) == Term::Comp(name, apply_terms(ts, s)),
    decreases s.len(),
{
    if s.len() > 0 {
        apply_comp(name, subst_all(ts, s[0].0, s[0].1), s.drop_first());
        assert(seq![s[0]] + s.drop_first() =~= s);
        apply_terms_cons(ts, s[0].0, s[0].1, s.drop_first());
    } else {
        assert(apply_terms(ts, s) =~= ts);
    }
}

pub open spec fn solves(p: (Term, Term), s: Seq<(nat, Term)>) -> bool {
    apply(p.0, s) == apply(p.1, s)
}

pub open spec fn solves_all(ps: Seq<(Term, Term)>, s: Seq<(nat, Term)>) -> bool {
    forall|i: int| 0 <= i < ps.len() ==> #[trigger] solves(ps[i], s)
}

pub open spec fn unifier_witness(u: UState, out: UOut, s: Seq<(nat, Term)>) -> bool {
    match out {
        UOut::Ok(_, sol) => solves_all(u.pairs, s) && sol == apply_terms(u.sol, s),
        _ => false,
    }
}

pub proof fn solves_join(ps: Seq<(Term, Term)>, s: Seq<(nat, Term)>)
    requires
        ps.len() > 0,
        solves(ps[0], s),
        solves_all(ps.drop_first(), s),
    ensures
        solves_all(ps, s),
{
    assert forall|i: int| 0 <= i < ps.len() implies #[trigger] solves(ps[i], s) by {
        if i > 0 {
            assert(ps.drop_first()[i - 1] == ps[i]);
        }
    }
}

pub proof fn solves_bind(rest: Seq<(Term, Term)>, x: nat, v: Term, s: Seq<(nat, Term)>)
    requires
        solves_all(rest.map_values(|p: (Term, Term)| (subst(p.0, x, v), subst(p.1, x, v))), s),
    ensures
        solves_all(rest, seq![(x, v)] + s),
{
    let mapped = rest.map_values(|p: (Term, Term)| (subst(p.0, x, v), subst(p.1, x, v)));
    assert forall|i: int| 0 <= i < rest.len() implies #[trigger] solves(
        rest[i],
        seq![(x, v)] + s,
    ) by {
        assert(solves(mapped[i], s));
        apply_cons(rest[i].0, x, v, s);
        apply_cons(rest[i].1, x, v, s);
    }
}

pub proof fn solves_tail(front: Seq<(Term, Term)>, rest: Seq<(Term, Term)>, s: Seq<(nat, Term)>)
    requires
        solves_all(front + rest, s),
    ensures
        solves_all(rest, s),
{
    assert forall|i: int| 0 <= i < rest.len() implies #[trigger] solves(rest[i], s) by {
        assert((front + rest)[front.len() + i] == rest[i]);
    }
}

pub proof fn solves_comp(
    name: Seq<u8>,
    xs: Seq<Term>,
    ys: Seq<Term>,
    rest: Seq<(Term, Term)>,
    s: Seq<(nat, Term)>,
)
    requires
        xs.len() == ys.len(),
        solves_all(zip(xs, ys) + rest, s),
    ensures
        solves((Term::Comp(name, xs), Term::Comp(name, ys)), s),
{
    assert forall|i: int| 0 <= i < xs.len() implies #[trigger] apply_terms(xs, s)[i] == apply_terms(
        ys,
        s,
    )[i] by {
        assert((zip(xs, ys) + rest)[i] == (xs[i], ys[i]));
        assert(solves((xs[i], ys[i]), s));
    }
    assert(apply_terms(xs, s) =~= apply_terms(ys, s));
    apply_comp(name, xs, s);
    apply_comp(name, ys, s);
}

pub proof fn lift_binding(u: UState, out: UOut, x: nat, v: Term, s: Seq<(nat, Term)>)
    requires
        u.pairs.len() > 0,
        !occurs(x, v),
        u.pairs[0] == (Term::Var(x), v) || u.pairs[0] == (v, Term::Var(x)),
        unifier_witness(u_bind(u, u.pairs.drop_first(), x, v), out, s),
    ensures
        unifier_witness(u, out, seq![(x, v)] + s),
{
    subst_absent(v, x, v);
    apply_cons(Term::Var(x), x, v, s);
    apply_cons(v, x, v, s);
    solves_bind(u.pairs.drop_first(), x, v, s);
    solves_join(u.pairs, seq![(x, v)] + s);
    apply_terms_cons(u.sol, x, v, s);
}

pub proof fn unify_n_witness(u: UState, fuel: nat)
    requires
        unify_n(u, fuel) is Ok,
    ensures
        exists|s: Seq<(nat, Term)>| #[trigger] unifier_witness(u, unify_n(u, fuel), s),
    decreases fuel,
{
    if fuel > 0 {
        if u.pairs.len() == 0 {
            let s = Seq::<(nat, Term)>::empty();
            assert(apply_terms(u.sol, s) =~= u.sol);
            assert(unifier_witness(u, unify_n(u, fuel), s));
        } else {
            let a = u.pairs[0].0;
            let b = u.pairs[0].1;
            let rest = u.pairs.drop_first();
            let f = (fuel - 1) as nat;
            match (a, b) {
                (Term::Var(x), _) => {
                    if a == b {
                        let nu = UState { pairs: rest, ..u };
                        unify_n_witness(nu, f);
                        let s = choose|s: Seq<(nat, Term)>| unifier_witness(nu, unify_n(nu, f), s);
                        solves_join(u.pairs, s);
                        assert(unifier_witness(u, unify_n(u, fuel), s));
                    } else if !occurs(x, b) {
                        let nu = u_bind(u, rest, x, b);
                        unify_n_witness(nu, f);
                        let s = choose|s: Seq<(nat, Term)>| unifier_witness(nu, unify_n(nu, f), s);
                        lift_binding(u, unify_n(nu, f), x, b, s);
                        assert(unifier_witness(u, unify_n(u, fuel), seq![(x, b)] + s));
                    }
                },
                (_, Term::Var(y)) => {
                    if !occurs(y, a) {
                        let nu = u_bind(u, rest, y, a);
                        unify_n_witness(nu, f);
                        let s = choose|s: Seq<(nat, Term)>| unifier_witness(nu, unify_n(nu, f), s);
                        lift_binding(u, unify_n(nu, f), y, a, s);
                        assert(unifier_witness(u, unify_n(u, fuel), seq![(y, a)] + s));
                    }
                },
                (Term::Comp(n, xs), Term::Comp(m, ys)) => {
                    if n == m && xs.len() == ys.len() {
                        let nu = UState { pairs: zip(xs, ys) + rest, ..u };
                        unify_n_witness(nu, f);
                        let s = choose|s: Seq<(nat, Term)>| unifier_witness(nu, unify_n(nu, f), s);
                        solves_comp(n, xs, ys, rest, s);
                        solves_tail(zip(xs, ys), rest, s);
                        solves_join(u.pairs, s);
                        assert(unifier_witness(u, unify_n(u, fuel), s));
                    }
                },
                _ => {
                    if a == b {
                        let nu = UState { pairs: rest, ..u };
                        unify_n_witness(nu, f);
                        let s = choose|s: Seq<(nat, Term)>| unifier_witness(nu, unify_n(nu, f), s);
                        solves_join(u.pairs, s);
                        assert(unifier_witness(u, unify_n(u, fuel), s));
                    }
                },
            }
        }
    }
}

pub proof fn unify_witness(u: UState)
    requires
        unify(u) is Ok,
    ensures
        exists|s: Seq<(nat, Term)>| #[trigger] unifier_witness(u, unify(u), s),
{
    let f = choose|f: nat| !(unify_n(u, f) is Out);
    unify_n_witness(u, f);
}

} // verus!
