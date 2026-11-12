use ckc_spec::term::Term;
use ckc_spec::v1text::{BodyItem, DocClause};
use vstd::prelude::*;

verus! {

// Contract mc D3 consequences the checker leans on: the bound set at each body item, and what
// left-to-right safety guarantees there.
// The variables items 0..j bind = items_safe's bound set at item j.
pub open spec fn binds_upto(c: DocClause, j: int) -> Set<nat>
    decreases j,
{
    if j <= 0 {
        Set::empty()
    } else {
        binds_upto(c, j - 1).union(ckc_spec::model::item_binds(c.body[j - 1]))
    }
}

pub proof fn items_safe_at(c: DocClause, j: nat)
    requires
        ckc_spec::model::items_safe(c, 0, Set::empty()),
        j <= c.body.len(),
    ensures
        ckc_spec::model::items_safe(c, j, binds_upto(c, j as int)),
    decreases j,
{
    if j > 0 {
        let i = (j - 1) as nat;
        items_safe_at(c, i);
        let b = binds_upto(c, i as int);
        assert(binds_upto(c, j as int) == b.union(ckc_spec::model::item_binds(c.body[i as int])));
        match c.body[i as int] {
            BodyItem::Pos(l) => {},
            BodyItem::Naf(gs) => {
                assert(b.union(Set::<nat>::empty()) =~= b);
            },
        }
    } else {
        assert(binds_upto(c, 0) == Set::<nat>::empty());
    }
}

// At a negation, its conjunction is safe from the bound set before it.
pub proof fn naf_safe_at(c: DocClause, j: int)
    requires
        ckc_spec::model::items_safe(c, 0, Set::empty()),
        0 <= j < c.body.len(),
        c.body[j] is Naf,
    ensures
        ckc_spec::model::conj_safe(c.body[j]->Naf_0, binds_upto(c, j)),
        forall|x: nat| #[trigger]
            ckc_spec::model::vars_all(c.body[j]->Naf_0).contains(x) && ckc_spec::model::outside(
                c,
                j,
                x,
            ) ==> binds_upto(c, j).contains(x),
{
    items_safe_at(c, j as nat);
}

// A safe literal stays safe over a larger bound set; so does a conjunction.
pub proof fn conj_safe_mono(gs: Seq<Term>, b1: Set<nat>, b2: Set<nat>)
    requires
        ckc_spec::model::conj_safe(gs, b1),
        b1.subset_of(b2),
    ensures
        ckc_spec::model::conj_safe(gs, b2),
    decreases gs.len(),
{
    if gs.len() > 0 {
        let l = gs[0];
        assert(ckc_spec::model::lit_safe(l, b2));
        let n1 = b1.union(ckc_spec::model::binds(l));
        let n2 = b2.union(ckc_spec::model::binds(l));
        assert(n1.subset_of(n2));
        conj_safe_mono(gs.drop_first(), n1, n2);
    }
}

// What a positive literal binds are its variables.
pub proof fn binds_in_vars(l: Term, x: nat)
    requires
        ckc_spec::model::binds(l).contains(x),
    ensures
        ckc_spec::model::vars(l).contains(x),
{
    if ckc_spec::model::builtin(l) {
        let args = l->Comp_1;
        let outs = ckc_spec::model::builtin_outputs(l);
        if ckc_spec::arith::is_arith(l->Comp_0, args.len()) {
            assert(outs =~= args.subrange(4, args.len() as int));
            crate::mc_safe::vars_all_sub(args, 4, args.len() as int, x);
        } else {
            assert(outs =~= Seq::<Term>::empty());
            crate::mc_safe::vars_all_any(outs, x);
        }
        crate::mc_safe::vars_comp(l, x);
    }
}

// Every bound variable occurs in a positive item, so it is non-local.
pub proof fn binds_upto_nonlocal(c: DocClause, j: int, x: nat)
    requires
        0 <= j <= c.body.len(),
        binds_upto(c, j).contains(x),
    ensures
        ckc_spec::model::nonlocal(c, x),
    decreases j,
{
    if j > 0 {
        if binds_upto(c, j - 1).contains(x) {
            binds_upto_nonlocal(c, j - 1, x);
        } else {
            let it = c.body[j - 1];
            assert(ckc_spec::model::item_binds(it).contains(x));
            if let BodyItem::Pos(l) = it {
                binds_in_vars(l, x);
                assert(ckc_spec::model::item_vars(c.body[j - 1]).contains(x));
            }
        }
    }
}

} // verus!
