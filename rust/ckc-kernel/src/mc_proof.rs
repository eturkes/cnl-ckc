use ckc_spec::model::*;
use ckc_spec::term::Term;
use ckc_spec::v1text::BodyItem;
use vstd::prelude::*;

verus! {

// Contract mc D7: the theorems behind `contract::mc_t1`, `mc_t1b`, `mc_wfm` + `mc_t3_wfm`.
pub proof fn lemma_apply_all_len(ts: Seq<Term>, th: Map<nat, Term>)
    ensures
        apply_all(ts, th).len() == ts.len(),
    decreases ts.len(),
{
    if ts.len() > 0 {
        lemma_apply_all_len(ts.drop_first(), th);
    }
}

// A valuation keeps a built-in's name + arity.
pub proof fn lemma_builtin_apply(l: Term, th: Map<nat, Term>)
    requires
        builtin(l),
    ensures
        builtin(apply(l, th)),
{
    if let Term::Comp(_, args) = l {
        lemma_apply_all_len(args, th);
    }
}

// Every claimed fact lies in every set closed under T_M (induction on the claim's level).
proof fn claim_in_closed(p: Program, m: ISet<Term>, c: Certificate, x: ISet<Term>, k: int)
    requires
        accepted(p, m, c),
        closed(p, m, x),
        0 <= k < c.claims.len(),
    ensures
        x.contains(c.claims[k].fact),
    decreases c.claims[k].level,
{
    let cl = c.claims[k];
    let ci = cl.clause as int;
    let th = cl.valuation;
    let cls = p.clauses[ci];
    assert(claim_valid(p, m, c, k));
    assert forall|j: int| 0 <= j < cls.body.len() implies #[trigger] item_holds(
        m,
        x,
        cls.body[j],
        th,
    ) by {
        assert(item_holds(m, m, cls.body[j], th));
        assert(supported(p, c, k, j));
        if let BodyItem::Pos(l) = cls.body[j] {
            let g = apply(l, th);
            if !builtin(g) {
                if builtin(l) {
                    lemma_builtin_apply(l, th);
                }
                assert(claimed_below(c, g, cl.level));
                let q = choose|q: int|
                    0 <= q < c.claims.len() && (#[trigger] c.claims[q]).fact == g
                        && c.claims[q].level < cl.level;
                claim_in_closed(p, m, c, x, q);
            }
        }
    }
    assert(instance_of(p, ci, th) && body_holds(m, x, p.clauses[ci], th) && cl.fact == apply(
        p.clauses[ci].head,
        th,
    ));
    assert(immediate(p, m, x).contains(cl.fact));
}

pub proof fn mc_t1_proof(p: Program, m: ISet<Term>, c: Certificate)
    requires
        accepted(p, m, c),
    ensures
        stable(p, m),
{
    assert forall|a: Term| #[trigger] lfp(p, m).contains(a) implies m.contains(a) by {
        assert(closed(p, m, m));
    }
    assert forall|a: Term| #[trigger] m.contains(a) implies lfp(p, m).contains(a) by {
        let k = choose|k: int| 0 <= k < c.claims.len() && (#[trigger] c.claims[k]).fact == a;
        assert forall|x: ISet<Term>| closed(p, m, x) implies #[trigger] x.contains(a) by {
            claim_in_closed(p, m, c, x, k);
        }
    }
    assert(m =~= lfp(p, m));
}

proof fn lemma_conj_mono(m1: ISet<Term>, m2: ISet<Term>, w: Seq<Term>)
    requires
        m1.subset_of(m2),
        conj_holds(m1, w),
    ensures
        conj_holds(m2, w),
{
    assert forall|k: int| 0 <= k < w.len() implies #[trigger] holds(m2, w[k]) by {
        assert(holds(m1, w[k]));
    }
}

// A larger M defeats more negations ⇒ fewer immediate consequences ⇒ more closed sets.
proof fn lemma_closed_antimono(p: Program, m1: ISet<Term>, m2: ISet<Term>, x: ISet<Term>)
    requires
        m1.subset_of(m2),
        closed(p, m1, x),
    ensures
        closed(p, m2, x),
{
    assert forall|h: Term| #[trigger] immediate(p, m2, x).contains(h) implies x.contains(h) by {
        let (ci, th) = choose|ci: int, th: Map<nat, Term>| #[trigger]
            instance_of(p, ci, th) && body_holds(m2, x, p.clauses[ci], th) && h == apply(
                p.clauses[ci].head,
                th,
            );
        let cls = p.clauses[ci];
        assert forall|j: int| 0 <= j < cls.body.len() implies #[trigger] item_holds(
            m1,
            x,
            cls.body[j],
            th,
        ) by {
            assert(item_holds(m2, x, cls.body[j], th));
            if let BodyItem::Naf(gs) = cls.body[j] {
                if !naf_holds(m1, gs, th) {
                    let s = choose|s: Map<nat, Term>| #[trigger]
                        conj_holds(m1, apply_all(apply_all(gs, th), s));
                    lemma_conj_mono(m1, m2, apply_all(apply_all(gs, th), s));
                }
            }
        }
        assert(instance_of(p, ci, th) && body_holds(m1, x, p.clauses[ci], th) && h == apply(
            p.clauses[ci].head,
            th,
        ));
        assert(immediate(p, m1, x).contains(h));
    }
}

// Γ(M) = lfp(T_M) is antimonotone in M.
proof fn lemma_lfp_antimono(p: Program, m1: ISet<Term>, m2: ISet<Term>)
    requires
        m1.subset_of(m2),
    ensures
        lfp(p, m2).subset_of(lfp(p, m1)),
{
    assert forall|a: Term| #[trigger] lfp(p, m2).contains(a) implies lfp(p, m1).contains(a) by {
        assert forall|x: ISet<Term>| closed(p, m1, x) implies #[trigger] x.contains(a) by {
            lemma_closed_antimono(p, m1, m2, x);
        }
    }
}

pub proof fn mc_t1b_proof(p: Program, m: ISet<Term>)
    requires
        stable(p, m),
    ensures
        lower(p).subset_of(m),
        m.subset_of(upper(p)),
{
    assert(lfp(p, lfp(p, m)) == m);
    assert forall|a: Term| #[trigger] lower(p).contains(a) implies m.contains(a) by {
        assert(lfp(p, lfp(p, m)).subset_of(m));
    }
    lemma_lfp_antimono(p, lower(p), m);
}

pub proof fn mc_wfm_proof(p: Program, m: ISet<Term>, c: Certificate)
    requires
        accepted(p, m, c),
        two_valued(p),
    ensures
        m == lower(p),
{
    mc_t1_proof(p, m, c);
    mc_t1b_proof(p, m);
    assert(m =~= lower(p));
}

pub proof fn mc_t3_wfm_proof(p: Program, s: ISet<nat>, m: ISet<Term>, c: Certificate, q: Question)
    requires
        accepted(select(p, s), m, c),
        two_valued(select(p, s)),
        answer_terms(lower(select(p, s)), q) == answer_terms(lower(p), q),
    ensures
        answer_terms(m, q) == answer_terms(lower(p), q),
{
    mc_wfm_proof(select(p, s), m, c);
}

} // verus!
