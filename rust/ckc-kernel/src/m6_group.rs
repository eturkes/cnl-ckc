#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::k2_term::{ENode, ETermArena};
use crate::m6_bind::{cons_locals, mint, skolem, witness_pairs};
use crate::m6_drs::ERoot;
#[cfg(verus_keep_ghost)]
use crate::m6_drs::{root_models, root_valid, roots_prefix, roots_valid};
use crate::m6_expand::{all_pos, expand, pos_terms};
use crate::m6_flat::{Env, flatten_list};
use crate::m6_model::*;
use crate::m6_refs::{first_vars, ref_slots};
#[cfg(verus_keep_ghost)]
use crate::m6_safety::expand_len;
use crate::m6_safety::{all_head_safe, all_naf_safe, body_doms};
use crate::m6_symbols::Sym;
#[cfg(verus_keep_ghost)]
use crate::m6_symbols::symbol;
use crate::m6_term::*;
use ckc_spec::emit as spec;
use ckc_spec::term::Term;
use ckc_spec::v1text::{self, BodyItem, DocClause};
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub fn error(arena: &mut ETermArena, sym: &Sym, Ghost(start): Ghost<Seq<ENode>>) -> (out: T)
    requires
        arena_ok(old(arena)),
        start.is_prefix_of(old(arena).nodes@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        start.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == Term::Atom(symbol(sym)),
{
    let ghost middle = arena.nodes@;
    let out = named(arena, sym);
    proof {
        crate::k2_load::prefix_chain(start, middle, arena.nodes@);
    }
    out
}

pub open spec fn group_result(r: Result<Group, T>) -> Result<spec::Group, Term> {
    match r {
        Ok(g) => Ok(g@),
        Err(e) => Err(e@),
    }
}

pub open spec fn group_result_valid(nodes: Seq<ENode>, r: &Result<Group, T>) -> bool {
    match r {
        Ok(g) => group_valid(nodes, g),
        Err(e) => valid(nodes, e),
    }
}

pub open spec fn fact_result(r: Result<(Group, Vec<Binding>), T>) -> Result<
    (spec::Group, Seq<(Term, Term)>),
    Term,
> {
    match r {
        Ok((g, m)) => Ok((g@, map_model(m@))),
        Err(e) => Err(e@),
    }
}

pub open spec fn fact_result_valid(
    nodes: Seq<ENode>,
    r: &Result<(Group, Vec<Binding>), T>,
) -> bool {
    match r {
        Ok((g, m)) => group_valid(nodes, g) && map_valid(nodes, m@),
        Err(e) => valid(nodes, e),
    }
}

pub fn rule_clauses(arena: &ETermArena, heads: &Vec<T>, body: &Vec<Body>) -> (out: Vec<Clause>)
    requires
        arena_ok(arena),
        valid_all(arena.nodes@, heads@),
        bodies_valid(arena.nodes@, body@),
    ensures
        clauses_valid(arena.nodes@, out@),
        clause_models(out@) == spec::rule_clauses(models(heads@), body_models(body@)),
        forall|i: int| 0 <= i < out.len() ==> (#[trigger] out@[i]).body.len() == body.len(),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < heads.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, heads@),
            bodies_valid(arena.nodes@, body@),
            clauses_valid(arena.nodes@, out@),
            i <= heads.len(),
            out.len() == i,
            clause_models(out@) == spec::rule_clauses(models(heads@), body_models(body@)).take(
                i as int,
            ),
            forall|j: int| 0 <= j < out.len() ==> (#[trigger] out@[j]).body.len() == body.len(),
        decreases heads.len() - i,
    {
        let next = Clause { head: heads[i].cp(), body: copy_body(arena, body) };
        proof {
            assert(body_models(next.body@) == body_models(body@));
            assert(next.body.len() == body.len());
        }
        let ghost before = out@;
        out.push(next);
        proof {
            assert forall|j: int| 0 <= j < out.len() implies #[trigger] clause_valid(
                arena.nodes@,
                &out@[j],
            ) by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                }
            }
            assert forall|j: int| 0 <= j < out.len() implies (#[trigger] out@[j]).body.len()
                == body.len() by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                }
            }
            assert_seqs_equal!(clause_models(out@) == spec::rule_clauses(models(heads@), body_models(body@)).take(i as int + 1), j => {
                if j < before.len() { assert(out@[j] == before[j]); assert(clause_models(before)[j] == spec::rule_clauses(models(heads@), body_models(body@))[j]); }
                else { assert(j == i); assert(out@[j] == next); }
            });
        }
        i += 1;
    }
    proof {
        assert(spec::rule_clauses(models(heads@), body_models(body@)).take(i as int)
            == spec::rule_clauses(models(heads@), body_models(body@)));
    }
    out
}

pub fn variant(
    arena: &mut ETermArena,
    aitems: &Vec<I>,
    citems: &Vec<I>,
    ordered: &Vec<T>,
    map: &Vec<Binding>,
    deps: &T,
    env: &Env,
    k: usize,
) -> (out: Result<Group, T>)
    requires
        arena_ok(old(arena)),
        items_valid(old(arena).nodes@, aitems@),
        items_valid(old(arena).nodes@, citems@),
        valid_all(old(arena).nodes@, ordered@),
        map_valid(old(arena).nodes@, map@),
        valid(old(arena).nodes@, deps),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        group_result_valid(final(arena).nodes@, &out),
        group_result(out) == spec::variant(
            item_models(aitems@),
            item_models(citems@),
            models(ordered@),
            map_model(map@),
            deps@,
            env.s as nat,
            env.docid@,
            k as nat,
        ),
{
    hide(spec::expand);
    hide(spec::skolem);
    hide(spec::witness_pairs);
    hide(spec::cons_locals);
    hide(spec::all_head_safe);
    hide(spec::all_naf_safe);
    let ghost start = arena.nodes@;
    let empty = Vec::new();
    proof {
        assert_seqs_equal!(models(empty@) == Seq::<Term>::empty());
    }
    let arefs = ref_slots(arena, aitems);
    let ante = first_vars(arena, &arefs, &empty);
    let locals = cons_locals(arena, ordered, &ante, map);
    let sko = skolem(arena, &locals, ordered, deps, env);
    let ghost n1 = arena.nodes@;
    proof {
        items_prefix(start, n1, aitems@);
        items_prefix(start, n1, citems@);
        map_prefix(start, n1, map@);
    }
    let goals_result = expand(arena, aitems, map, &sko);
    let ghost n2 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n1, n2);
    }
    let goals = match goals_result {
        Ok(g) => g,
        Err(e) => return Err(e),
    };
    proof {
        items_prefix(n1, n2, citems@);
        map_prefix(n1, n2, map@);
        map_prefix(n1, n2, sko@);
    }
    let heads_result = expand(arena, citems, map, &sko);
    let ghost n3 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n2, n3);
        bodies_prefix(n2, n3, goals@);
    }
    let heads = match heads_result {
        Ok(h) => h,
        Err(e) => return Err(e),
    };
    if goals.len() == 0 {
        return Err(error(arena, &Sym::RuleWithoutAntecedent, Ghost(start)));
    }
    if heads.len() == 0 || !all_pos(&heads) {
        return Err(error(arena, &Sym::RuleWithoutConsequent, Ghost(start)));
    }
    let hterms = pos_terms(arena, &heads);
    let cs = rule_clauses(arena, &hterms, &goals);
    proof {
        items_prefix(start, n3, aitems@);
    }
    let ds = body_doms(arena, aitems);
    proof {
        expand_len(item_models(aitems@), map_model(map@), map_model(sko@));
    }
    if !all_naf_safe(arena, &cs, &ds) {
        return Err(error(arena, &Sym::NafSafety, Ghost(start)));
    }
    if !all_head_safe(arena, &cs) {
        return Err(error(arena, &Sym::HeadVariableNotBoundInBody, Ghost(start)));
    }
    proof {
        prefix_all(start, n3, ordered@);
        map_prefix(start, n3, map@);
    }
    let acc = Vec::new();
    proof {
        assert_seqs_equal!(map_model(acc@) == Seq::<(Term, Term)>::empty());
    }
    let pairs = witness_pairs(arena, aitems, ordered, map, env, k, &acc);
    proof {
        crate::k2_load::prefix_chain(start, n3, arena.nodes@);
        clauses_prefix(n3, arena.nodes@, cs@);
    }
    Ok(Group { k, pairs, clauses: cs })
}

pub open spec fn root_condition_model(r: spec::Root) -> Option<Term> {
    match r {
        spec::Root::Anchored(inner) => Some(
            Term::Comp(
                v1text::ascii("-"@),
                seq![inner, Term::Comp(v1text::ascii("/"@), seq![Term::Int(0), Term::Int(0)])],
            ),
        ),
        spec::Root::Boxed(c) => Some(c),
        spec::Root::Rule(_, _) => None,
    }
}

pub fn root_condition(arena: &mut ETermArena, root: &ERoot) -> (out: Option<T>)
    requires
        arena_ok(old(arena)),
        root_valid(old(arena).nodes@, root),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Some(t) ==> valid(final(arena).nodes@, &t),
        crate::m6_drs::term_opt(out) == root_condition_model(root@),
{
    let ghost start = arena.nodes@;
    match root {
        ERoot::Rule(_, _) => None,
        ERoot::Boxed(t) => Some(t.cp()),
        ERoot::Anchored(inner) => {
            let z = crate::m6_term::int(arena, 0);
            let ghost n1 = arena.nodes@;
            let anchor = c2(arena, &Sym::Slash, &z, &z);
            let ghost n2 = arena.nodes@;
            proof {
                crate::k2_load::prefix_chain(start, n1, n2);
                prefix(start, n2, inner);
            }
            let t = c2(arena, &Sym::Minus, inner, &anchor);
            proof {
                crate::k2_load::prefix_chain(start, n2, arena.nodes@);
            }
            Some(t)
        },
    }
}

pub fn roots_conds_from(arena: &mut ETermArena, roots: &Vec<ERoot>, i: usize) -> (out: Option<T>)
    requires
        arena_ok(old(arena)),
        roots_valid(old(arena).nodes@, roots@),
        i <= roots.len(),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Some(t) ==> valid(final(arena).nodes@, &t),
        crate::m6_drs::term_opt(out) == spec::roots_conds(root_models(roots@).skip(i as int)),
    decreases roots.len() - i,
{
    let ghost start = arena.nodes@;
    proof {
        reveal_with_fuel(spec::roots_conds, 1);
        reveal_strlit("[|]");
        reveal(v1text::ascii);
        assert_seqs_equal!(symbol(&Sym::Cons) == v1text::cons_name());
    }
    if i == roots.len() {
        return Some(nil(arena));
    }
    proof {
        assert(root_models(roots@).skip(i as int).drop_first() == root_models(roots@).skip(
            i as int + 1,
        ));
    }
    let head = match root_condition(arena, &roots[i]) {
        Some(t) => t,
        None => return None,
    };
    let ghost n1 = arena.nodes@;
    proof {
        roots_prefix(start, n1, roots@);
    }
    let tail = roots_conds_from(arena, roots, i + 1);
    let ghost n2 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n1, n2);
    }
    match tail {
        None => None,
        Some(tail) => {
            proof {
                prefix(n1, n2, &head);
            }
            let out = c2(arena, &Sym::Cons, &head, &tail);
            proof {
                crate::k2_load::prefix_chain(start, n2, arena.nodes@);
            }
            Some(out)
        },
    }
}

pub fn fact_group(
    arena: &mut ETermArena,
    roots: &Vec<ERoot>,
    env: &Env,
    map: &Vec<Binding>,
) -> (out: Result<(Group, Vec<Binding>), T>)
    requires
        arena_ok(old(arena)),
        roots_valid(old(arena).nodes@, roots@),
        map_valid(old(arena).nodes@, map@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        fact_result_valid(final(arena).nodes@, &out),
        fact_result(out) == spec::fact_group(
            root_models(roots@),
            env.s as nat,
            env.docid@,
            map_model(map@),
            env.base as nat,
        ),
{
    hide(spec::flatten_list);
    hide(spec::mint);
    hide(spec::expand);
    hide(spec::all_head_safe);
    let ghost start = arena.nodes@;
    proof {
        assert(root_models(roots@).skip(0) == root_models(roots@));
    }
    let conds = match roots_conds_from(arena, roots, 0) {
        Some(c) => c,
        None => return Err(error(arena, &Sym::SentenceShape, Ghost(start))),
    };
    let ghost n1 = arena.nodes@;
    let deps = nil(arena);
    let ghost n2 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n1, n2);
        prefix(n1, n2, &conds);
    }
    let actual = named(arena, &Sym::Actual);
    let ghost n3 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n2, n3);
        prefix(n2, n3, &conds);
        prefix(n2, n3, &deps);
    }
    let flat = flatten_list(arena, &conds, &W::Root, env, &deps, &actual, &E::Top, 1);
    let ghost n4 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n3, n4);
    }
    let flat = match flat {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let refs = ref_slots(arena, &flat.items);
    let empty = Vec::new();
    proof {
        assert_seqs_equal!(models(empty@) == Seq::<Term>::empty());
    }
    let ordered = first_vars(arena, &refs, &empty);
    proof {
        map_prefix(start, n4, map@);
    }
    let map2 = mint(arena, &ordered, 1, env, map);
    let ghost n5 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n4, n5);
        items_prefix(n4, n5, flat.items@);
    }
    let sko = Vec::new();
    proof {
        assert_seqs_equal!(map_model(sko@) == Seq::<(Term, Term)>::empty());
    }
    let heads_result = expand(arena, &flat.items, &map2, &sko);
    let ghost n6 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n5, n6);
        map_prefix(n5, n6, map2@);
    }
    let heads = match heads_result {
        Ok(h) => h,
        Err(e) => return Err(e),
    };
    if !all_pos(&heads) {
        return Err(error(arena, &Sym::SentenceShape, Ghost(start)));
    }
    let hs = pos_terms(arena, &heads);
    let body = Vec::new();
    let cs = rule_clauses(arena, &hs, &body);
    proof {
        assert_seqs_equal!(body_models(body@) == Seq::<BodyItem>::empty());
        assert_seqs_equal!(clause_models(cs@) == models(hs@).map_values(|h: Term| spec::fact_clause(h)));
    }
    if !all_head_safe(arena, &cs) {
        return Err(error(arena, &Sym::HeadVariableNotBoundInBody, Ghost(start)));
    }
    let pairs = Vec::new();
    proof {
        assert_seqs_equal!(map_model(pairs@) == Seq::<(Term, Term)>::empty());
    }
    Ok((Group { k: 1, pairs, clauses: cs }, map2))
}

} // verus!
