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
use crate::m6_safety::{all_head_safe, all_naf_safe, body_doms, pad_doms};
#[cfg(verus_keep_ghost)]
use crate::m6_safety::{all_naf_safe_pad, dom_models, doms_valid, expand_len};
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

fn append_pos(arena: &ETermArena, goals: Vec<Body>, chain: &Vec<T>) -> (out: Vec<Body>)
    requires
        arena_ok(arena),
        bodies_valid(arena.nodes@, goals@),
        valid_all(arena.nodes@, chain@),
    ensures
        bodies_valid(arena.nodes@, out@),
        body_models(out@) == body_models(goals@) + spec::pos_all(models(chain@)),
{
    let mut out = goals;
    let ghost base = goals@;
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(body_models(out@) == body_models(base) + spec::pos_all(models(chain@).take(0)));
    }
    while i < chain.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, chain@),
            bodies_valid(arena.nodes@, out@),
            i <= chain.len(),
            body_models(out@) == body_models(base) + spec::pos_all(models(chain@).take(i as int)),
        decreases chain.len() - i,
    {
        let ghost prev = out@;
        proof {
            assert(valid(arena.nodes@, &chain@[i as int]));
        }
        let b = Body::Pos(chain[i].cp());
        let ghost bm = b@;
        out.push(b);
        proof {
            let t = models(chain@).take(i as int);
            assert(models(chain@).take(i as int + 1) == t.push(chain@[i as int]@));
            assert(bm == BodyItem::Pos(chain@[i as int]@));
            assert_seqs_equal!(spec::pos_all(t.push(chain@[i as int]@)) == spec::pos_all(t).push(bm));
            assert_seqs_equal!(body_models(out@) == body_models(prev).push(bm), x => {
                if x < prev.len() {
                    assert(out@[x] == prev[x]);
                }
            });
            assert_seqs_equal!((body_models(base) + spec::pos_all(t)).push(bm) == body_models(base)
                + spec::pos_all(t).push(bm));
            assert forall|x: int| 0 <= x < out.len() implies #[trigger] body_valid(
                arena.nodes@,
                &out@[x],
            ) by {
                if x < prev.len() {
                    assert(out@[x] == prev[x]);
                }
            }
        }
        i += 1;
    }
    proof {
        assert(models(chain@).take(chain.len() as int) == models(chain@));
    }
    out
}

pub fn variant(
    arena: &mut ETermArena,
    aitems0: &Vec<I>,
    citems: &Vec<I>,
    ordered: &Vec<T>,
    map: &Vec<Binding>,
    deps: &T,
    env: &Env,
    k: usize,
    cb: usize,
    src: &crate::m6_numeral::ESrc,
    an: &Vec<crate::m6_formula::EAnch>,
) -> (out: Result<Group, T>)
    requires
        arena_ok(old(arena)),
        items_valid(old(arena).nodes@, aitems0@),
        items_valid(old(arena).nodes@, citems@),
        valid_all(old(arena).nodes@, ordered@),
        map_valid(old(arena).nodes@, map@),
        valid(old(arena).nodes@, deps),
        crate::m6_numeral::src_valid(old(arena).nodes@, src),
        crate::m6_formula::anchs_valid(old(arena).nodes@, an@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        group_result_valid(final(arena).nodes@, &out),
        group_result(out) == spec::variant(
            item_models(aitems0@),
            item_models(citems@),
            models(ordered@),
            map_model(map@),
            deps@,
            env.s as nat,
            env.docid@,
            k as nat,
            cb as nat,
            src@,
            crate::m6_formula::anch_models(an@),
        ),
{
    hide(spec::expand);
    hide(spec::compared);
    hide(spec::skolem);
    hide(spec::witness_pairs);
    hide(spec::cons_locals);
    hide(spec::all_head_safe);
    hide(spec::all_naf_safe);
    hide(spec::formula_pass);
    let ghost start = arena.nodes@;
    let empty = Vec::new();
    proof {
        assert_seqs_equal!(models(empty@) == Seq::<Term>::empty());
    }
    let arefs = ref_slots(arena, aitems0);
    let ante = first_vars(arena, &arefs, &empty);
    let locals = cons_locals(arena, ordered, &ante, map);
    let sko = skolem(arena, &locals, ordered, deps, env);
    let ghost ns = arena.nodes@;
    proof {
        items_prefix(start, ns, aitems0@);
    }
    let conv = crate::m6_compare_refs::conv_refs(arena, aitems0);
    let converted = crate::m6_compare::compared(arena, aitems0, cb);
    let ghost n0 = arena.nodes@;
    let c1 = crate::m6_flat::add(arena, cb, conv.len());
    let c2 = crate::m6_flat::add(arena, c1, conv.len());
    let v0 = crate::m6_flat::add(arena, c2, conv.len());
    let ghost n1 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, ns, n0);
        crate::k2_load::prefix_chain(start, n0, n1);
        items_prefix(start, n1, citems@);
        items_prefix(n0, n1, converted@);
        prefix_all(start, n1, ordered@);
        map_prefix(start, n1, map@);
        map_prefix(ns, n1, sko@);
        prefix_all(start, n1, src.toks@);
        assert forall|x: int| 0 <= x < an.len() implies valid(n1, &(#[trigger] an@[x]).inner)
            && valid(n1, &an@[x].k) && an@[x].k@ is Int by {
            prefix(start, n1, &an@[x].inner);
            prefix(start, n1, &an@[x].k);
        }
        assert(v0 as nat == cb as nat + 3 * spec::conv_refs(item_models(aitems0@)).len());
    }
    let fp = match crate::m6_formula::formula_pass(
        arena,
        src,
        an,
        &converted,
        citems,
        ordered,
        map,
        env,
        k,
        v0,
    ) {
        Ok(p) => p,
        Err(e) => return Err(e),
    };
    let ghost n2 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n1, n2);
        map_prefix(n1, n2, map@);
        map_prefix(n1, n2, sko@);
    }
    let goals_result = expand(arena, &fp.aitems, map, &sko);
    let ghost n3 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n2, n3);
    }
    let goals0 = match goals_result {
        Ok(g) => g,
        Err(e) => return Err(e),
    };
    proof {
        items_prefix(n2, n3, fp.citems@);
        map_prefix(n2, n3, map@);
        map_prefix(n2, n3, sko@);
    }
    let heads_result = expand(arena, &fp.citems, map, &sko);
    let ghost n4 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(start, n3, n4);
        crate::k2_load::prefix_chain(n2, n3, n4);
        bodies_prefix(n3, n4, goals0@);
        prefix_all(n2, n4, fp.chain@);
    }
    let heads = match heads_result {
        Ok(h) => h,
        Err(e) => return Err(e),
    };
    proof {
        items_prefix(n2, n4, fp.aitems@);
        expand_len(item_models(fp.aitems@), map_model(map@), map_model(sko@));
    }
    let cs = match group_clauses(arena, goals0, &heads, &fp.chain, &fp.aitems) {
        Ok(cs) => cs,
        Err(e) => {
            proof {
                crate::k2_load::prefix_chain(start, n4, arena.nodes@);
            }
            return Err(e);
        },
    };
    proof {
        crate::k2_load::prefix_chain(start, n4, arena.nodes@);
        map_prefix(n2, arena.nodes@, fp.pairs@);
    }
    Ok(Group { k, pairs: fp.pairs, clauses: cs })
}

// The clause checks of `spec::variant` after both expansions.
pub open spec fn variant_tail(
    goals: Seq<BodyItem>,
    heads: Seq<BodyItem>,
    chain: Seq<Term>,
    aitems: Seq<spec::Item>,
) -> Result<Seq<DocClause>, Term> {
    if goals.len() == 0 {
        Err(ckc_spec::replay::atom("rule_without_antecedent"@))
    } else if heads.len() == 0 || !spec::all_pos(heads) {
        Err(ckc_spec::replay::atom("rule_without_consequent"@))
    } else {
        let body = goals + spec::pos_all(chain);
        let cs = spec::rule_clauses(spec::pos_terms(heads), body);
        if !spec::all_naf_safe(cs, spec::body_doms(aitems)) {
            Err(ckc_spec::replay::atom("naf_safety"@))
        } else if !spec::all_head_safe(cs) {
            Err(ckc_spec::replay::atom("head_variable_not_bound_in_body"@))
        } else {
            Ok(spec::rule_clauses(spec::pos_terms(heads), spec::bind_first(body)))
        }
    }
}

fn group_clauses(
    arena: &mut ETermArena,
    goals0: Vec<Body>,
    heads: &Vec<Body>,
    chain: &Vec<T>,
    aitems: &Vec<I>,
) -> (out: Result<Vec<Clause>, T>)
    requires
        arena_ok(old(arena)),
        bodies_valid(old(arena).nodes@, goals0@),
        bodies_valid(old(arena).nodes@, heads@),
        valid_all(old(arena).nodes@, chain@),
        items_valid(old(arena).nodes@, aitems@),
        goals0.len() == spec::body_doms(item_models(aitems@)).len(),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        match &out {
            Ok(cs) => clauses_valid(final(arena).nodes@, cs@),
            Err(e) => valid(final(arena).nodes@, e),
        },
        match &out {
            Ok(cs) => Ok::<Seq<DocClause>, Term>(clause_models(cs@)),
            Err(e) => Err::<Seq<DocClause>, Term>(e@),
        } == variant_tail(
            body_models(goals0@),
            body_models(heads@),
            models(chain@),
            item_models(aitems@),
        ),
{
    hide(spec::all_head_safe);
    hide(spec::all_naf_safe);
    let ghost start = arena.nodes@;
    proof {
        reveal_strlit("rule_without_antecedent");
        reveal_strlit("rule_without_consequent");
        reveal_strlit("naf_safety");
        reveal_strlit("head_variable_not_bound_in_body");
    }
    if goals0.len() == 0 {
        return Err(error(arena, &Sym::RuleWithoutAntecedent, Ghost(start)));
    }
    if heads.len() == 0 || !all_pos(heads) {
        return Err(error(arena, &Sym::RuleWithoutConsequent, Ghost(start)));
    }
    let ghost g0 = body_models(goals0@);
    let goals = append_pos(arena, goals0, chain);
    let hterms = pos_terms(arena, heads);
    let cs = rule_clauses(arena, &hterms, &goals);
    let ds0 = body_doms(arena, aitems);
    let ghost d0 = dom_models(ds0@);
    let ghost pad = Seq::new(chain.len() as nat, |i: int| Seq::<Term>::empty());
    proof {
        assert(doms_valid(arena.nodes@, ds0@));
    }
    let ds = pad_doms(ds0, chain.len());
    proof {
        let cms = clause_models(cs@);
        let body = body_models(goals@);
        assert(body == g0 + spec::pos_all(models(chain@)));
        assert(d0.len() == g0.len());
        assert forall|i: int| 0 <= i < cms.len() implies d0.len() <= (#[trigger] cms[i]).body.len()
            && forall|j: int|
            d0.len() <= j < cms[i].body.len() ==> (#[trigger] cms[i].body[j]) is Pos by {
            assert(cms[i].body == body);
            assert forall|j: int| d0.len() <= j < cms[i].body.len() implies (
            #[trigger] cms[i].body[j]) is Pos by {
                assert(body[j] == spec::pos_all(models(chain@))[j - g0.len()]);
            }
        }
        all_naf_safe_pad(cms, d0, pad);
        assert(dom_models(ds@) == d0 + pad);
        assert(dom_models(ds@).len() == ds@.len());
        assert(d0.len() == ds0@.len());
        assert(body_models(goals@).len() == goals@.len());
        assert(spec::pos_all(models(chain@)).len() == chain@.len());
        assert(ds.len() == goals.len());
        assert forall|i: int| 0 <= i < cs.len() implies (#[trigger] cs@[i]).body.len()
            == ds.len() by {
            assert(cs@[i].body.len() == goals.len());
        }
    }
    if !all_naf_safe(arena, &cs, &ds) {
        return Err(error(arena, &Sym::NafSafety, Ghost(start)));
    }
    if !all_head_safe(arena, &cs) {
        return Err(error(arena, &Sym::HeadVariableNotBoundInBody, Ghost(start)));
    }
    let ordered = crate::m6_order::bind_first(arena, goals);
    Ok(rule_clauses(arena, &hterms, &ordered))
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
            crate::m7_annotate::tab_view(&env.tab),
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
    let flat = crate::m7_strength::flatten_doc(
        arena,
        &env.tab,
        &conds,
        &W::Root,
        env,
        &deps,
        &actual,
        &E::Top,
        1,
    );
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
        assert_seqs_equal!(clause_models(cs@) == models(hs@).map_values(|h: Term| ckc_spec::engine::fact_clause(h)));
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
