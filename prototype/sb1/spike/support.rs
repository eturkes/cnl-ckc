use crate::*;

verus! {

pub proof fn least(p: Program, m: Set<Atom>, x: Set<Atom>)
    requires
        closed(p, m, x),
    ensures
        lfp(p, m).subset_of(x),
{
}

pub proof fn lfp_closed(p: Program, m: Set<Atom>)
    requires
        wf(p),
    ensures
        closed(p, m, lfp(p, m)),
{
    assert forall|i: int|
        0 <= i < p.rules.len() && retained(p.rules[i], m) && p.rules[i].positive.subset_of(
            lfp(p, m),
        ) implies lfp(p, m).contains(p.rules[i].head) by {
        let r = p.rules[i];
        assert forall|x: Set<Atom>| closed(p, m, x) implies x.contains(r.head) by {
            least(p, m, x);
            assert(r.positive.subset_of(x));
        }
    }
}

pub proof fn lfp_fixed(p: Program, m: Set<Atom>)
    requires
        wf(p),
    ensures
        lfp(p, m) == immediate(p, m, lfp(p, m)),
{
    lfp_closed(p, m);
    let l = lfp(p, m);
    let t = immediate(p, m, l);
    assert(t.subset_of(l));
    assert forall|i: int|
        0 <= i < p.rules.len() && retained(p.rules[i], m) && p.rules[i].positive.subset_of(
            t,
        ) implies t.contains(p.rules[i].head) by {
        assert(p.rules[i].positive.subset_of(l));
        assert(p.atoms.contains(p.rules[i].head));
        assert(exists|j: int|
            0 <= j < p.rules.len() && p.rules[j].head == p.rules[i].head && retained(p.rules[j], m)
                && p.rules[j].positive.subset_of(l));
    }
    least(p, m, t);
    assert(l =~= t);
}

pub proof fn founded_in_closed(
    p: Program,
    m: Set<Atom>,
    level: spec_fn(Atom) -> nat,
    x: Set<Atom>,
    a: Atom,
)
    requires
        founded(p, m, level),
        closed(p, m, x),
        m.contains(a),
    ensures
        x.contains(a),
    decreases level(a),
{
    let i = choose|i: int|
        0 <= i < p.rules.len() && p.rules[i].head == a && retained(p.rules[i], m)
            && p.rules[i].positive.subset_of(m) && forall|b: Atom|
            p.rules[i].positive.contains(b) ==> level(b) < level(a);
    assert forall|b: Atom| p.rules[i].positive.contains(b) implies x.contains(b) by {
        founded_in_closed(p, m, level, x, b);
    }
}

// (a) Finite ground program + domain-respecting M + strictly level-founded
// support + closure under every retained rule => M is the reduct's least fixed
// point. Correlated existential NAF is retained()'s whole-witness test; ground
// built-in truth is immutable input, not an additional theorem assumption.
pub proof fn support_closure_lfp(p: Program, m: Set<Atom>, level: spec_fn(Atom) -> nat)
    requires
        wf(p),
        m.subset_of(p.atoms),
        founded(p, m, level),
        closed(p, m, m),
    ensures
        m == lfp(p, m),
        stable(p, m),
        m == immediate(p, m, m),
{
    least(p, m, m);
    assert forall|a: Atom| m.contains(a) implies lfp(p, m).contains(a) by {
        assert forall|x: Set<Atom>| closed(p, m, x) implies x.contains(a) by {
            founded_in_closed(p, m, level, x, a);
        }
    }
    assert(m =~= lfp(p, m));
    lfp_fixed(p, m);
}

} // verus!
