#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::k2_term::{ENode, ETermArena};
use crate::m6_ante::*;
use crate::m6_drs::box_parts;
use crate::m6_flat::Env;
use crate::m6_group::{error, variant};
use crate::m6_model::*;
use crate::m6_refs::{first_vars, ref_slots};
use crate::m6_symbols::Sym;
use crate::m6_term::*;
use ckc_spec::emit as spec;
use ckc_spec::term::Term;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub open spec fn groups_result(r: Result<Vec<Group>, T>) -> Result<Seq<spec::Group>, Term> {
    match r {
        Ok(gs) => Ok(group_models(gs@)),
        Err(e) => Err(e@),
    }
}

pub open spec fn groups_result_valid(nodes: Seq<ENode>, r: &Result<Vec<Group>, T>) -> bool {
    match r {
        Ok(gs) => groups_valid(nodes, gs@),
        Err(e) => valid(nodes, e),
    }
}

pub fn one_group(arena: &ETermArena, g: Group) -> (out: Vec<Group>)
    requires
        arena_ok(arena),
        group_valid(arena.nodes@, &g),
    ensures
        groups_valid(arena.nodes@, out@),
        group_models(out@) == seq![g@],
{
    let mut out = Vec::new();
    out.push(g);
    proof {
        assert_seqs_equal!(group_models(out@) == seq![g@]);
    }
    out
}

pub fn two_groups(arena: &ETermArena, a: Group, b: Group) -> (out: Vec<Group>)
    requires
        arena_ok(arena),
        group_valid(arena.nodes@, &a),
        group_valid(arena.nodes@, &b),
    ensures
        groups_valid(arena.nodes@, out@),
        group_models(out@) == seq![a@, b@],
{
    let mut out = Vec::new();
    out.push(a);
    out.push(b);
    proof {
        assert_seqs_equal!(group_models(out@) == seq![a@, b@]);
    }
    out
}

pub struct Ants {
    pub shared: Flat,
    pub left: Flat,
    pub right: Flat,
}

impl View for Ants {
    type V = (spec::Flat, spec::Flat, spec::Flat);

    open spec fn view(&self) -> Self::V {
        (self.shared@, self.left@, self.right@)
    }
}

pub open spec fn ants_valid(nodes: Seq<ENode>, a: &Ants) -> bool {
    flat_valid(nodes, &a.shared) && flat_valid(nodes, &a.left) && flat_valid(nodes, &a.right)
}

pub open spec fn ants_result(r: Result<Ants, T>) -> Result<
    (spec::Flat, spec::Flat, spec::Flat),
    Term,
> {
    match r {
        Ok(a) => Ok(a@),
        Err(e) => Err(e@),
    }
}

pub open spec fn ants_result_valid(nodes: Seq<ENode>, r: &Result<Ants, T>) -> bool {
    match r {
        Ok(a) => ants_valid(nodes, a),
        Err(e) => valid(nodes, e),
    }
}

pub proof fn ants_prefix(before: Seq<ENode>, after: Seq<ENode>, a: &Ants)
    requires
        before.is_prefix_of(after),
        ants_valid(before, a),
    ensures
        ants_valid(after, a),
{
    items_prefix(before, after, a.shared.items@);
    items_prefix(before, after, a.left.items@);
    items_prefix(before, after, a.right.items@);
}

pub open spec fn ant_relation(shared: Seq<Term>, left: Term, right: Term, env: &Env) -> Result<
    (spec::Flat, spec::Flat, spec::Flat),
    Term,
> {
    match spec::flatten_seq(shared, env.s as nat, env.docid@, 1, env.base as nat) {
        Err(e) => Err(e),
        Ok(fs) => match spec::flatten_list(
            left,
            spec::Where::Antecedent,
            env.s as nat,
            env.docid@,
            Term::Nil,
            spec::actual(),
            spec::Encl::Top,
            fs.n,
            env.base as nat,
        ) {
            Err(e) => Err(e),
            Ok(f1) => match spec::flatten_list(
                right,
                spec::Where::Antecedent,
                env.s as nat,
                env.docid@,
                Term::Nil,
                spec::actual(),
                spec::Encl::Top,
                f1.n,
                env.base as nat,
            ) {
                Err(e) => Err(e),
                Ok(f2) => Ok((fs, f1, f2)),
            },
        },
    }
}

pub fn antecedents(arena: &mut ETermArena, shared: &Vec<T>, left: &T, right: &T, env: &Env) -> (out:
    Result<Ants, T>)
    requires
        arena_ok(old(arena)),
        valid_all(old(arena).nodes@, shared@),
        valid(old(arena).nodes@, left),
        valid(old(arena).nodes@, right),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        ants_result_valid(final(arena).nodes@, &out),
        ants_result(out) == ant_relation(models(shared@), left@, right@, env),
{
    hide(spec::flatten_list);
    hide(spec::flatten_seq);
    let ghost start = arena.nodes@;
    let fs = match flatten_seq(arena, shared, env, 1) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let ghost n1 = arena.nodes@;
    proof {
        prefix(start, n1, left);
    }
    let f1_result = flatten_ante(arena, left, env, fs.n);
    let ghost n2 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n1, n2);
        items_prefix(n1, n2, fs.items@);
    }
    let f1 = match f1_result {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    proof {
        prefix(start, n2, right);
    }
    let f2_result = flatten_ante(arena, right, env, f1.n);
    let ghost n3 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n2, n3);
        items_prefix(n2, n3, fs.items@);
        items_prefix(n2, n3, f1.items@);
    }
    match f2_result {
        Err(e) => Err(e),
        Ok(f2) => Ok(Ants { shared: fs, left: f1, right: f2 }),
    }
}

pub open spec fn cons_relation(conds: Term, d1: Term, d2: Term, n: nat, env: &Env) -> Result<
    (spec::Flat, spec::Flat),
    Term,
> {
    match spec::flatten_cons(conds, env.s as nat, env.docid@, d1, n, env.base as nat) {
        Err(e) => Err(e),
        Ok(c1) => match spec::flatten_cons(
            conds,
            env.s as nat,
            env.docid@,
            d2,
            n,
            env.base as nat,
        ) {
            Err(e) => Err(e),
            Ok(c2) => Ok((c1, c2)),
        },
    }
}

pub open spec fn cons_pair_result(r: Result<(Flat, Flat), T>) -> Result<
    (spec::Flat, spec::Flat),
    Term,
> {
    match r {
        Ok((a, b)) => Ok((a@, b@)),
        Err(e) => Err(e@),
    }
}

pub open spec fn cons_pair_valid(nodes: Seq<ENode>, r: &Result<(Flat, Flat), T>) -> bool {
    match r {
        Ok((a, b)) => flat_valid(nodes, a) && flat_valid(nodes, b),
        Err(e) => valid(nodes, e),
    }
}

pub fn consequents(arena: &mut ETermArena, conds: &T, d1: &T, d2: &T, n: usize, env: &Env) -> (out:
    Result<(Flat, Flat), T>)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, conds),
        valid(old(arena).nodes@, d1),
        valid(old(arena).nodes@, d2),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        cons_pair_valid(final(arena).nodes@, &out),
        cons_pair_result(out) == cons_relation(conds@, d1@, d2@, n as nat, env),
{
    hide(spec::flatten_cons);
    let ghost start = arena.nodes@;
    let c1 = match flatten_cons(arena, conds, env, d1, n) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let ghost middle = arena.nodes@;
    proof {
        prefix(start, middle, conds);
        prefix(start, middle, d2);
    }
    let c2 = flatten_cons(arena, conds, env, d2, n);
    proof {
        crate::k2_load::prefix_chain(start, middle, arena.nodes@);
        items_prefix(middle, arena.nodes@, c1.items@);
    }
    match c2 {
        Err(e) => Err(e),
        Ok(c2) => Ok((c1, c2)),
    }
}

pub fn split_variants(
    arena: &mut ETermArena,
    shared: &Vec<T>,
    dom1: &Vec<T>,
    conds1: &T,
    dom2: &Vec<T>,
    conds2: &T,
    cconds: &T,
    adom: &Vec<T>,
    env: &Env,
    map: &Vec<Binding>,
) -> (out: Result<Vec<Group>, T>)
    requires
        arena_ok(old(arena)),
        valid_all(old(arena).nodes@, shared@),
        valid_all(old(arena).nodes@, dom1@),
        valid_all(old(arena).nodes@, dom2@),
        valid_all(old(arena).nodes@, adom@),
        valid(old(arena).nodes@, conds1),
        valid(old(arena).nodes@, conds2),
        valid(old(arena).nodes@, cconds),
        map_valid(old(arena).nodes@, map@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        groups_result_valid(final(arena).nodes@, &out),
        groups_result(out) == spec::split_variants(
            models(shared@),
            models(dom1@),
            conds1@,
            models(dom2@),
            conds2@,
            cconds@,
            models(adom@),
            env.s as nat,
            env.docid@,
            map_model(map@),
            env.base as nat,
        ),
{
    hide(spec::flatten_seq);
    hide(spec::flatten_list);
    hide(spec::flatten_cons);
    hide(spec::variant);
    let ghost start = arena.nodes@;
    let d1terms = concat(adom, dom1);
    let d2terms = concat(adom, dom2);
    let deps1 = list(arena, &d1terms);
    let ghost n1 = arena.nodes@;
    proof {
        prefix_all(start, n1, d2terms@);
    }
    let deps2 = list(arena, &d2terms);
    let ghost n2 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n1, n2);
        prefix(n1, n2, &deps1);
        prefix_all(start, n2, shared@);
        prefix(start, n2, conds1);
        prefix(start, n2, conds2);
    }
    let ants_result = antecedents(arena, shared, conds1, conds2, env);
    let ghost n3 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n2, n3);
    }
    let ants = match ants_result {
        Ok(a) => a,
        Err(e) => return Err(e),
    };
    proof {
        prefix(n2, n3, &deps1);
        prefix(n2, n3, &deps2);
        prefix(start, n3, cconds);
    }
    let cons_result = consequents(arena, cconds, &deps1, &deps2, ants.right.n, env);
    let ghost n4 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n3, n4);
        ants_prefix(n3, n4, &ants);
    }
    let (c1, c2) = match cons_result {
        Ok(pair) => pair,
        Err(e) => return Err(e),
    };
    let all1 = concat_items(arena, &ants.shared.items, &ants.left.items);
    let all2 = concat_items(arena, &all1, &ants.right.items);
    let all3 = concat_items(arena, &all2, &c1.items);
    let refs = ref_slots(arena, &all3);
    let empty = Vec::new();
    proof {
        assert_seqs_equal!(models(empty@) == Seq::<Term>::empty());
    }
    let ordered = first_vars(arena, &refs, &empty);
    let a1 = all1;
    let a2 = concat_items(arena, &ants.shared.items, &ants.right.items);
    proof {
        prefix(n3, n4, &deps1);
        prefix(n3, n4, &deps2);
        map_prefix(start, n4, map@);
    }
    let g1_result = variant(arena, &a1, &c1.items, &ordered, map, &deps1, env, 1);
    let ghost n5 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n4, n5);
    }
    let g1 = match g1_result {
        Ok(g) => g,
        Err(e) => return Err(e),
    };
    proof {
        items_prefix(n4, n5, a2@);
        items_prefix(n4, n5, c2.items@);
        prefix_all(n4, n5, ordered@);
        map_prefix(n4, n5, map@);
        prefix(n4, n5, &deps2);
    }
    let g2_result = variant(arena, &a2, &c2.items, &ordered, map, &deps2, env, 2);
    let ghost n6 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n5, n6);
        map_prefix(n5, n6, g1.pairs@);
        clauses_prefix(n5, n6, g1.clauses@);
    }
    match g2_result {
        Err(e) => Err(e),
        Ok(g2) => Ok(two_groups(arena, g1, g2)),
    }
}

pub open spec fn unsplit_relation(
    shared: Seq<Term>,
    adom: Seq<Term>,
    cconds: Term,
    env: &Env,
    map: Seq<(Term, Term)>,
) -> Result<Seq<spec::Group>, Term> {
    match spec::flatten_seq(shared, env.s as nat, env.docid@, 1, env.base as nat) {
        Err(e) => Err(e),
        Ok(fa) => match spec::flatten_cons(
            cconds,
            env.s as nat,
            env.docid@,
            spec::list_of(adom),
            fa.n,
            env.base as nat,
        ) {
            Err(e) => Err(e),
            Ok(fc) => {
                let ordered = spec::first_vars(spec::ref_slots(fa.items + fc.items), Seq::empty());
                match spec::variant(
                    fa.items,
                    fc.items,
                    ordered,
                    map,
                    spec::list_of(adom),
                    env.s as nat,
                    env.docid@,
                    1,
                ) {
                    Err(e) => Err(e),
                    Ok(g) => Ok(seq![g]),
                }
            },
        },
    }
}

pub fn unsplit(
    arena: &mut ETermArena,
    shared: &Vec<T>,
    adom: &Vec<T>,
    cconds: &T,
    env: &Env,
    map: &Vec<Binding>,
) -> (out: Result<Vec<Group>, T>)
    requires
        arena_ok(old(arena)),
        valid_all(old(arena).nodes@, shared@),
        valid_all(old(arena).nodes@, adom@),
        valid(old(arena).nodes@, cconds),
        map_valid(old(arena).nodes@, map@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        groups_result_valid(final(arena).nodes@, &out),
        groups_result(out) == unsplit_relation(
            models(shared@),
            models(adom@),
            cconds@,
            env,
            map_model(map@),
        ),
{
    hide(spec::flatten_seq);
    hide(spec::flatten_cons);
    hide(spec::variant);
    let ghost start = arena.nodes@;
    let deps = list(arena, adom);
    let ghost n1 = arena.nodes@;
    proof {
        prefix_all(start, n1, shared@);
    }
    let fa_result = flatten_seq(arena, shared, env, 1);
    let ghost n2 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n1, n2);
    }
    let fa = match fa_result {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    proof {
        prefix(start, n2, cconds);
        prefix(n1, n2, &deps);
    }
    let fc_result = flatten_cons(arena, cconds, env, &deps, fa.n);
    let ghost n3 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n2, n3);
        items_prefix(n2, n3, fa.items@);
    }
    let fc = match fc_result {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let all = concat_items(arena, &fa.items, &fc.items);
    let refs = ref_slots(arena, &all);
    let empty = Vec::new();
    proof {
        assert_seqs_equal!(models(empty@) == Seq::<Term>::empty());
        prefix(n2, n3, &deps);
        map_prefix(start, n3, map@);
    }
    let ordered = first_vars(arena, &refs, &empty);
    let g_result = variant(arena, &fa.items, &fc.items, &ordered, map, &deps, env, 1);
    proof {
        crate::k2_load::prefix_chain(start, n3, arena.nodes@);
    }
    match g_result {
        Err(e) => Err(e),
        Ok(g) => Ok(one_group(arena, g)),
    }
}

pub fn rule_groups(
    arena: &mut ETermArena,
    ante: &T,
    cons: &T,
    env: &Env,
    map: &Vec<Binding>,
) -> (out: Result<Vec<Group>, T>)
    requires
        arena_ok(old(arena)),
        valid(old(arena).nodes@, ante),
        valid(old(arena).nodes@, cons),
        map_valid(old(arena).nodes@, map@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        groups_result_valid(final(arena).nodes@, &out),
        groups_result(out) == spec::rule_groups(
            ante@,
            cons@,
            env.s as nat,
            env.docid@,
            map_model(map@),
            env.base as nat,
        ),
{
    hide(spec::curry);
    hide(spec::split_scan);
    hide(spec::split_variants);
    hide(spec::flatten_seq);
    hide(spec::flatten_cons);
    hide(spec::variant);
    let ghost start = arena.nodes@;
    let c = match curry(arena, ante, cons, 64) {
        Ok(c) => c,
        Err(e) => return Err(e),
    };
    let b = match box_parts(arena, &c.cons) {
        Some(b) => b,
        None => return Err(error(arena, &Sym::InvalidDrsShape, Ghost(start))),
    };
    let adom = seg_domain(arena, &c.segs);
    proof {
        assert(seg_models(c.segs@).skip(0) == seg_models(c.segs@));
    }
    let split = match split_scan_from(arena, &c.segs, 0) {
        Some(s) => s,
        None => return Err(error(arena, &Sym::InvalidDrsShape, Ghost(start))),
    };
    if split.arms.len() == 0 {
        return unsplit(arena, &split.shared, &adom, &b.conds, env, map);
    }
    if split.arms.len() != 1 {
        return Err(error(arena, &Sym::DisjunctiveAntecedent, Ghost(start)));
    }
    proof {
        assert(binding_valid(arena.nodes@, &split.arms@[0]));
    }
    match (box_parts(arena, &split.arms[0].key), box_parts(arena, &split.arms[0].value)) {
        (Some(left), Some(right)) => {
            if is_comp(arena, &split.arms[0].key, &Sym::V, 2) || is_comp(
                arena,
                &split.arms[0].value,
                &Sym::V,
                2,
            ) {
                return Err(error(arena, &Sym::DisjunctiveAntecedent, Ghost(start)));
            }
            split_variants(
                arena,
                &split.shared,
                &left.dom,
                &left.conds,
                &right.dom,
                &right.conds,
                &b.conds,
                &adom,
                env,
                map,
            )
        },
        _ => Err(error(arena, &Sym::InvalidDrsShape, Ghost(start))),
    }
}

} // verus!
