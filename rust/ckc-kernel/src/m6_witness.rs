use crate::k2_engine::EClause;
#[cfg(verus_keep_ghost)]
use crate::k2_engine::{db_valid, db_view};
use crate::k2_index::EClauseIndex;
#[cfg(verus_keep_ghost)]
use crate::k2_index::index_over;
#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::k2_term::{ENode, ETermArena};
use crate::m6_cands::*;
use crate::m6_model::*;
use crate::m6_project::copy_clauses;
use crate::m6_symbols::Sym;
use crate::m6_term::*;
use ckc_spec::emit as spec;
use ckc_spec::replay::EOut;
use ckc_spec::term::Term;
use ckc_spec::v1text::DocClause;
use ckc_spec::witness as ws;
#[cfg(verus_keep_ghost)]
use vstd::arithmetic::div_mod::{lemma_fundamental_div_mod, lemma_fundamental_div_mod_converse};
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

// Witness selection (contract ws).
pub open spec fn opnd_models(os: Seq<EOpnd>) -> Seq<ws::Opnd> {
    os.map_values(|o: EOpnd| o@)
}

// `ws::operands` over the converted antecedent of a variant.
pub fn operands(arena: &ETermArena, items: &Vec<I>, refs: &Vec<T>) -> (out: Vec<EOpnd>)
    requires
        arena_ok(arena),
        items_valid(arena.nodes@, items@),
        valid_all(arena.nodes@, refs@),
    ensures
        forall|i: int| 0 <= i < out.len() ==> #[trigger] opnd_valid(arena.nodes@, &out@[i]),
        opnd_models(out@) == ws::operands(item_models(items@), models(refs@), Seq::empty()),
{
    let mut out: Vec<EOpnd> = Vec::new();
    let mut seen: Vec<T> = Vec::new();
    let mut i = 0usize;
    proof {
        assert(item_models(items@).skip(0) == item_models(items@));
        assert_seqs_equal!(models(seen@) == Seq::<Term>::empty());
        assert_seqs_equal!(opnd_models(out@) + ws::operands(item_models(items@), models(refs@), Seq::empty())
            == ws::operands(item_models(items@), models(refs@), Seq::empty()));
    }
    while i < items.len()
        invariant
            arena_ok(arena),
            items_valid(arena.nodes@, items@),
            valid_all(arena.nodes@, refs@),
            valid_all(arena.nodes@, seen@),
            i <= items.len(),
            forall|j: int| 0 <= j < out.len() ==> #[trigger] opnd_valid(arena.nodes@, &out@[j]),
            ws::operands(item_models(items@), models(refs@), Seq::empty()) == opnd_models(out@)
                + ws::operands(item_models(items@).skip(i as int), models(refs@), models(seen@)),
        decreases items.len() - i,
    {
        proof {
            items_at(arena.nodes@, items@, i as int);
            reveal(item_valid);
            assert(item_models(items@).skip(i as int).drop_first() == item_models(items@).skip(
                i as int + 1,
            ));
            assert(item_models(items@).skip(i as int)[0] == items@[i as int]@);
            reveal_with_fuel(ws::operands, 1);
            reveal_strlit("$guideline_bound");
            reveal_strlit("$guideline_amount");
        }
        match &items[i].kind {
            Kind::Anch(_, inner) => {
                let bound = is_comp(arena, inner, &Sym::DollarGuidelineBound, 8);
                let amount = if bound {
                    false
                } else {
                    is_comp(arena, inner, &Sym::DollarGuidelineAmount, 6)
                };
                if bound || amount {
                    let xs = args(arena, inner);
                    proof {
                        assert(valid(arena.nodes@, &xs@[0]));
                        assert(ckc_spec::replay::arg(inner@, 0) == xs@[0]@);
                    }
                    let fresh = !crate::m6_refs::contains(arena, &seen, &xs[0]);
                    let take = fresh && (amount || crate::m6_refs::contains(arena, refs, &xs[0]));
                    if take {
                        proof {
                            assert(valid(arena.nodes@, &xs@[4]));
                            assert(valid(arena.nodes@, &xs@[5]));
                            assert(ckc_spec::replay::arg(inner@, 4) == xs@[4]@);
                            assert(ckc_spec::replay::arg(inner@, 5) == xs@[5]@);
                        }
                        let cond = if bound {
                            proof {
                                assert(valid(arena.nodes@, &xs@[6]));
                                assert(valid(arena.nodes@, &xs@[7]));
                                assert(ckc_spec::replay::arg(inner@, 6) == xs@[6]@);
                                assert(ckc_spec::replay::arg(inner@, 7) == xs@[7]@);
                            }
                            Some((xs[6].cp(), xs[7].cp()))
                        } else {
                            None
                        };
                        let next = EOpnd { s: xs[4].cp(), n: xs[5].cp(), cond };
                        let ghost before = out@;
                        let ghost seen0 = seen@;
                        out.push(next);
                        let r = xs[0].cp();
                        proof {
                            extend(arena.nodes@, seen@, r);
                        }
                        seen.push(r);
                        proof {
                            assert_seqs_equal!(opnd_models(out@) == opnd_models(before) + seq![next@]);
                            assert_seqs_equal!(models(seen@) == models(seen0).push(xs@[0]@));
                            assert forall|j: int| 0 <= j < out.len() implies #[trigger] opnd_valid(
                                arena.nodes@,
                                &out@[j],
                            ) by {
                                if j < before.len() {
                                    assert(out@[j] == before[j]);
                                }
                            }
                        }
                    }
                }
            },
            _ => {},
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(ws::operands, 1);
        assert_seqs_equal!(opnd_models(out@) + Seq::<ws::Opnd>::empty() == opnd_models(out@));
    }
    out
}

pub proof fn group_prefix(before: Seq<ENode>, after: Seq<ENode>, g: &Group)
    requires
        before.is_prefix_of(after),
        group_valid(before, g),
    ensures
        group_valid(after, g),
{
    map_prefix(before, after, g.pairs@);
    crate::m6_model::clauses_prefix(before, after, g.clauses@);
    sel_prefix(before, after, &g.sel);
}

pub open spec fn ops_valid(nodes: Seq<ENode>, sel: &ESel) -> bool {
    forall|i: int| 0 <= i < sel.ops.len() ==> #[trigger] opnd_valid(nodes, &sel.ops@[i])
}

pub proof fn ops_prefix_pub(before: Seq<ENode>, after: Seq<ENode>, sel: &ESel)
    requires
        before.is_prefix_of(after),
        ops_valid(before, sel),
    ensures
        ops_valid(after, sel),
{
    ops_prefix(before, after, sel);
}

proof fn ops_prefix(before: Seq<ENode>, after: Seq<ENode>, sel: &ESel)
    requires
        before.is_prefix_of(after),
        ops_valid(before, sel),
    ensures
        ops_valid(after, sel),
{
    assert forall|i: int| 0 <= i < sel.ops.len() implies #[trigger] opnd_valid(
        after,
        &sel.ops@[i],
    ) by {
        assert(opnd_valid(before, &sel.ops@[i]));
        prefix(before, after, &sel.ops@[i].s);
        prefix(before, after, &sel.ops@[i].n);
        match &sel.ops@[i].cond {
            Some((op, b)) => {
                prefix(before, after, op);
                prefix(before, after, b);
            },
            None => {},
        }
    }
}

// `ws::cand_lists`: per operand, its candidates (the default first).
pub fn cand_lists(arena: &mut ETermArena, g: &Group, sel: &ESel) -> (out: Vec<Vec<T>>)
    requires
        arena_ok(old(arena)),
        group_valid(old(arena).nodes@, g),
        ops_valid(old(arena).nodes@, sel),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        seqs_valid(final(arena).nodes@, out@),
        seqs_models(out@) == ws::cand_lists(g@, sel@),
        out.len() == sel.ops.len(),
        forall|i: int| 0 <= i < out.len() ==> (#[trigger] out@[i]).len() >= 1,
{
    let ghost start = arena.nodes@;
    let cs = counts(sel);
    proof {
        assert forall|j: int| 0 <= j < cs.len() implies #[trigger] valid(arena.nodes@, &cs@[j]) by {
            assert(opnd_valid(arena.nodes@, &sel.ops@[j]));
        }
    }
    let op = open_pairs(arena, g, sel, &cs);
    let sk = skeleton(arena, g, sel, &op);
    let ghost n1 = arena.nodes@;
    proof {
        group_prefix(start, n1, g);
        map_prefix(start, n1, op@);
    }
    let nafs = naf_items(arena, g, sel, &op);
    let ghost n2 = arena.nodes@;
    proof {
        prefix_all(n1, n2, sk@);
        ops_prefix(start, n2, sel);
    }
    let mut out: Vec<Vec<T>> = Vec::new();
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(seqs_models(out@) == ws::cand_lists(g@, sel@).take(0));
    }
    while i < sel.ops.len()
        invariant
            arena_ok(arena),
            start.is_prefix_of(arena.nodes@),
            ops_valid(arena.nodes@, sel),
            valid_all(arena.nodes@, sk@),
            seqs_valid(arena.nodes@, nafs@),
            seqs_valid(arena.nodes@, out@),
            models(sk@) == ws::skeleton(g@, sel@),
            seqs_models(nafs@) == ws::naf_items(g@, sel@),
            i <= sel.ops.len(),
            out.len() == i,
            seqs_models(out@) == ws::cand_lists(g@, sel@).take(i as int),
            forall|j: int| 0 <= j < i ==> (#[trigger] out@[j]).len() >= 1,
        decreases sel.ops.len() - i,
    {
        let o = &sel.ops[i];
        proof {
            assert(opnd_valid(arena.nodes@, &sel.ops@[i as int]));
            assert(sel@.ops[i as int] == o@);
            assert(cond_model(o.cond) == sel@.ops[i as int].cond);
        }
        let ghost n3 = arena.nodes@;
        let bs = boundaries(arena, &o.cond, &nafs, &sk, &o.n);
        let ghost n4 = arena.nodes@;
        proof {
            assert(opnd_valid(n3, &sel.ops@[i as int]));
            match &o.cond {
                Some((op, b)) => {
                    prefix(n3, n4, op);
                    prefix(n3, n4, b);
                },
                None => {},
            }
            assert(models(bs@) == ws::boundaries(g@, sel@, i as int));
        }
        let c = candidates(arena, &o.cond, &bs);
        let ghost n5 = arena.nodes@;
        proof {
            let bm = ws::boundaries(g@, sel@, i as int);
            let cm = sel@.ops[i as int].cond;
            reveal_with_fuel(ws::candidates, 1);
            assert((|x: Term| ws::admits(cond_model(o.cond), x) && x != ws::w0(cond_model(o.cond)))
                =~= (|x: Term| ws::admits(cm, x) && x != ws::w0(cm)));
            assert(models(c@) == ws::candidates(g@, sel@, i as int));
            ops_prefix(n3, n5, sel);
            prefix_all(n3, n5, sk@);
            seqs_prefix(n3, n5, nafs@);
            seqs_prefix(n3, n5, out@);
        }
        let ghost before = out@;
        out.push(c);
        proof {
            assert_seqs_equal!(seqs_models(out@) == ws::cand_lists(g@, sel@).take(i as int + 1), j => {
                if j < i {
                    assert(out@[j] == before[j]);
                }
            });
            assert forall|j: int| 0 <= j < out.len() implies valid_all(
                arena.nodes@,
                (#[trigger] out@[j])@,
            ) by {
                if j < i {
                    assert(out@[j] == before[j]);
                }
            }
            assert forall|j: int| 0 <= j < out.len() implies (#[trigger] out@[j]).len() >= 1 by {
                if j < i {
                    assert(out@[j] == before[j]);
                }
            }
        }
        i += 1;
    }
    proof {
        assert(ws::cand_lists(g@, sel@).take(i as int) =~= ws::cand_lists(g@, sel@));
    }
    out
}

// --- D5: the document's program ---
// The program every formula variant replays over (`replay::doc_db`) + its
// clause index, built once per document.
pub struct DocProgram {
    pub db: Vec<EClause>,
    pub ix: EClauseIndex,
}

// Opaque: certification carries it past the projection without its quantifiers.
#[verifier::opaque]
pub open spec fn program_ok(nodes: Seq<ENode>, prog: &DocProgram, p: Seq<DocClause>) -> bool {
    db_valid(nodes, prog.db@) && db_view(nodes, prog.db@) == p && index_over(prog.ix, p)
}

pub proof fn program_prefix(
    before: Seq<ENode>,
    after: Seq<ENode>,
    prog: &DocProgram,
    p: Seq<DocClause>,
)
    requires
        before.is_prefix_of(after),
        program_ok(before, prog, p),
    ensures
        program_ok(after, prog, p),
{
    reveal(program_ok);
    crate::k2_engine::db_models_prefix(before, after, prog.db@);
}

pub fn doc_program(
    arena: &mut ETermArena,
    parsed: &crate::v1_term_impl::EParsedV1,
    Ghost(doc): Ghost<ckc_spec::v1text::DocFile>,
) -> (out: DocProgram)
    requires
        arena_ok(old(arena)),
        crate::v1_term_impl::parsed_doc_roots_ok(old(arena).nodes@, parsed),
        crate::v1_term_impl::parsed_metadata_ok(parsed),
        parsed@ == ckc_spec::v1text::V1File::Doc(doc),
        ckc_spec::v1text::wf_doc(doc),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        program_ok(final(arena).nodes@, &out, ckc_spec::replay::doc_db(doc)),
{
    let db = crate::k2_bridge::document_db(arena, parsed, Ghost(doc));
    let ix = crate::k2_index::build_index(arena, &db);
    let out = DocProgram { db, ix };
    proof {
        reveal(program_ok);
    }
    out
}

// --- D5: one tuple ---
// `ws::op_pairs`: each operand's comparison exact, its count = t's coordinate.
pub fn op_pairs(arena: &mut ETermArena, sel: &ESel, t: &Vec<T>) -> (out: Vec<Binding>)
    requires
        arena_ok(old(arena)),
        ops_valid(old(arena).nodes@, sel),
        valid_all(old(arena).nodes@, t@),
        t.len() == sel.ops.len(),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        map_valid(final(arena).nodes@, out@),
        map_model(out@) == ws::op_pairs(sel@, models(t@)),
{
    let ghost start = arena.nodes@;
    let ex = named(arena, &Sym::Exactly);
    let ghost tm = models(t@);
    let ghost rows = Seq::new(
        sel.ops.len() as nat,
        |i: int| seq![(sel@.ops[i].s, ws::exactly()), (sel@.ops[i].n, tm[i])],
    );
    proof {
        reveal_strlit("exactly");
        ops_prefix(start, arena.nodes@, sel);
        prefix_all(start, arena.nodes@, t@);
        assert(ex@ == ws::exactly());
        assert(rows.take(0) =~= Seq::<Seq<(Term, Term)>>::empty());
    }
    let mut out: Vec<Binding> = Vec::new();
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(map_model(out@) == rows.take(0).flatten());
    }
    while i < sel.ops.len()
        invariant
            arena_ok(arena),
            ops_valid(arena.nodes@, sel),
            valid_all(arena.nodes@, t@),
            valid(arena.nodes@, &ex),
            ex@ == ws::exactly(),
            t.len() == sel.ops.len(),
            tm == models(t@),
            rows == Seq::new(
                sel.ops.len() as nat,
                |i: int| seq![(sel@.ops[i].s, ws::exactly()), (sel@.ops[i].n, tm[i])],
            ),
            i <= sel.ops.len(),
            map_valid(arena.nodes@, out@),
            map_model(out@) == rows.take(i as int).flatten(),
        decreases sel.ops.len() - i,
    {
        let o = &sel.ops[i];
        proof {
            assert(opnd_valid(arena.nodes@, &sel.ops@[i as int]));
            assert(valid(arena.nodes@, &t@[i as int]));
            assert(rows.take(i as int + 1) =~= rows.take(i as int).push(rows[i as int]));
            rows.take(i as int).lemma_flatten_push(rows[i as int]);
            assert(sel@.ops[i as int] == o@);
        }
        let ghost before = out@;
        out.push(Binding { key: o.s.cp(), value: ex.cp() });
        out.push(Binding { key: o.n.cp(), value: t[i].cp() });
        proof {
            assert_seqs_equal!(map_model(out@) == map_model(before) + rows[i as int]);
            assert forall|j: int| 0 <= j < out.len() implies #[trigger] binding_valid(
                arena.nodes@,
                &out@[j],
            ) by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                }
            }
        }
        i += 1;
    }
    proof {
        assert(rows.take(i as int) =~= rows);
        assert(ws::op_pairs(sel@, tm) == rows.flatten());
    }
    out
}

// The witness pairs re-valued by vs: `tuple_pairs`' map over g's pairs.
fn revalue(arena: &ETermArena, pairs: &Vec<Binding>, vs: &Vec<Binding>) -> (out: Vec<Binding>)
    requires
        arena_ok(arena),
        map_valid(arena.nodes@, pairs@),
        map_valid(arena.nodes@, vs@),
    ensures
        map_valid(arena.nodes@, out@),
        out.len() == pairs.len(),
        forall|j: int|
            0 <= j < out.len() ==> (#[trigger] map_model(out@)[j]) == match spec::lookup(
                map_model(vs@),
                map_model(pairs@)[j].0,
            ) {
                Some(v) => (map_model(pairs@)[j].0, v),
                None => map_model(pairs@)[j],
            },
{
    let mut out: Vec<Binding> = Vec::new();
    let mut i = 0usize;
    while i < pairs.len()
        invariant
            arena_ok(arena),
            map_valid(arena.nodes@, pairs@),
            map_valid(arena.nodes@, vs@),
            map_valid(arena.nodes@, out@),
            i <= pairs.len(),
            out.len() == i,
            forall|j: int|
                0 <= j < i ==> (#[trigger] map_model(out@)[j]) == match spec::lookup(
                    map_model(vs@),
                    map_model(pairs@)[j].0,
                ) {
                    Some(v) => (map_model(pairs@)[j].0, v),
                    None => map_model(pairs@)[j],
                },
        decreases pairs.len() - i,
    {
        proof {
            assert(binding_valid(arena.nodes@, &pairs@[i as int]));
            assert(map_model(pairs@)[i as int] == pairs@[i as int]@);
        }
        let b = match crate::m6_refs::lookup(arena, vs, &pairs[i].key) {
            Some(v) => Binding { key: pairs[i].key.cp(), value: v },
            None => Binding { key: pairs[i].key.cp(), value: pairs[i].value.cp() },
        };
        let ghost before = out@;
        out.push(b);
        proof {
            assert forall|j: int| 0 <= j < out.len() implies #[trigger] binding_valid(
                arena.nodes@,
                &out@[j],
            ) by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                }
            }
            assert forall|j: int| 0 <= j < i + 1 implies (#[trigger] map_model(out@)[j])
                == match spec::lookup(map_model(vs@), map_model(pairs@)[j].0) {
                Some(v) => (map_model(pairs@)[j].0, v),
                None => map_model(pairs@)[j],
            } by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                    assert(map_model(out@)[j] == map_model(before)[j]);
                }
            }
        }
        i += 1;
    }
    out
}

// The obligation's facts + heads for g under the pairs (`spec::obligation`).
fn ob_parts(arena: &mut ETermArena, g: &Group, pairs: &Vec<Binding>) -> (out: (Vec<T>, Vec<T>))
    requires
        arena_ok(old(arena)),
        group_valid(old(arena).nodes@, g),
        map_valid(old(arena).nodes@, pairs@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid_all(final(arena).nodes@, out.0@),
        valid_all(final(arena).nodes@, out.1@),
        models(out.0@) == spec::obligation(
            spec::Group { pairs: map_model(pairs@), ..g@ },
            Seq::empty(),
            0,
        ).facts,
        models(out.1@) == spec::obligation(
            spec::Group { pairs: map_model(pairs@), ..g@ },
            Seq::empty(),
            0,
        ).heads,
{
    hide(spec::bind_all);
    let ghost start = arena.nodes@;
    let ghost gm = spec::Group { pairs: map_model(pairs@), ..g@ };
    let unbound_facts = if g.clauses.len() == 0 {
        let empty = Vec::new();
        proof {
            assert_seqs_equal!(models(empty@) == Seq::<Term>::empty());
        }
        empty
    } else {
        proof {
            assert(clause_valid(arena.nodes@, &g.clauses@[0]));
        }
        crate::m6_safety::positive_goals(arena, &g.clauses[0].body)
    };
    let facts = crate::m6_payload::bind_terms(arena, &unbound_facts, pairs);
    let ghost middle = arena.nodes@;
    proof {
        group_prefix(start, middle, g);
        map_prefix(start, middle, pairs@);
    }
    let unbound_heads = crate::m6_payload::clause_heads(arena, &g.clauses);
    let heads = crate::m6_payload::bind_terms(arena, &unbound_heads, pairs);
    proof {
        prefix_all(middle, arena.nodes@, facts@);
        reveal(spec::obligation);
        assert_seqs_equal!(models(heads@) == spec::obligation(gm, Seq::empty(), 0).heads);
        assert_seqs_equal!(models(facts@) == spec::obligation(gm, Seq::empty(), 0).facts);
    }
    (facts, heads)
}

// t's pairs when t wins (`ws::win`), else None.
fn tuple_win(
    arena: &mut ETermArena,
    g: &Group,
    sel: &ESel,
    t: &Vec<T>,
    db: &Vec<EClause>,
    ix: &EClauseIndex,
    p: Ghost<Seq<DocClause>>,
) -> (out: Option<Vec<Binding>>)
    requires
        arena_ok(old(arena)),
        group_valid(old(arena).nodes@, g),
        ops_valid(old(arena).nodes@, sel),
        valid_all(old(arena).nodes@, sel.chain@),
        arith_goals(sel.chain@),
        valid_all(old(arena).nodes@, t@),
        t.len() == sel.ops.len(),
        db_valid(old(arena).nodes@, db@),
        db_view(old(arena).nodes@, db@) == p@,
        index_over(*ix, p@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Some(ps) ==> map_valid(final(arena).nodes@, ps@),
        (out is Some) == ws::win(p@, g@, sel@, models(t@)),
        out matches Some(ps) ==> ws::tuple_pairs(g@, sel@, models(t@)) == Some(map_model(ps@)),
{
    let ghost start = arena.nodes@;
    let ghost tm = models(t@);
    let op = op_pairs(arena, sel, t);
    let ghost n1 = arena.nodes@;
    proof {
        prefix_all(start, n1, sel.chain@);
        group_prefix(start, n1, g);
        assert(models(sel.chain@) == sel@.chain);
    }
    let vs = match crate::m6_formula::chain_pairs(arena, &sel.chain, &op) {
        Some(v) => v,
        None => {
            proof {
                reveal(ws::tuple_pairs);
                reveal(ws::win);
            }
            return None;
        },
    };
    let ghost n2 = arena.nodes@;
    proof {
        group_prefix(n1, n2, g);
    }
    let pairs = revalue(arena, &g.pairs, &vs);
    proof {
        reveal(ws::tuple_pairs);
        assert(map_model(pairs@) =~= ws::tuple_pairs(g@, sel@, tm)->Some_0);
    }
    let (facts, heads) = ob_parts(arena, g, &pairs);
    let ghost n3 = arena.nodes@;
    proof {
        crate::k2_engine::db_models_prefix(start, n3, db@);
        map_prefix(n2, n3, pairs@);
    }
    let fr = roots(arena, &facts);
    let hr = roots(arena, &heads);
    let witnessed = crate::k2_bridge::witness_db_exec(arena, &fr, db);
    proof {
        let w = ckc_spec::engine::witness_db(models(facts@), p@);
        assert(w.skip(fr.len() as int) =~= p@);
    }
    let proved = crate::k2_engine::heads_proved(arena, &witnessed, ix, fr.len(), &hr);
    proof {
        map_prefix(n3, arena.nodes@, pairs@);
        reveal(ws::win);
    }
    if proved {
        Some(pairs)
    } else {
        None
    }
}

// --- D4: the lazy odometer (digits in place, the last operand fastest) ---
pub open spec fn dnat(ds: Seq<usize>) -> Seq<nat> {
    ds.map_values(|d: usize| d as nat)
}

pub open spec fn lens_pos(cs: Seq<Seq<Term>>) -> bool {
    forall|k: int| 0 <= k < cs.len() ==> (#[trigger] cs[k]).len() >= 1
}

// The digits after ds; None past the last tuple.
pub open spec fn advance(cs: Seq<Seq<Term>>, ds: Seq<nat>) -> Option<Seq<nat>>
    decreases ds.len(),
{
    if ds.len() == 0 || cs.len() == 0 {
        None
    } else if ds.last() + 1 < cs.last().len() {
        Some(ds.drop_last().push((ds.last() + 1) as nat))
    } else {
        match advance(cs.drop_last(), ds.drop_last()) {
            Some(q) => Some(q.push(0nat)),
            None => None,
        }
    }
}

pub open spec fn zeros(m: nat) -> Seq<nat> {
    Seq::new(m, |j: int| 0nat)
}

pub open spec fn lift(r: Option<Seq<nat>>, m: nat) -> Option<Seq<nat>> {
    match r {
        Some(q) => Some(q + zeros(m)),
        None => None,
    }
}

proof fn lens_pos_pre(cs: Seq<Seq<Term>>)
    requires
        lens_pos(cs),
        cs.len() > 0,
    ensures
        lens_pos(cs.drop_last()),
        cs.last().len() >= 1,
{
    assert forall|k: int| 0 <= k < cs.drop_last().len() implies (#[trigger] cs.drop_last()[k]).len()
        >= 1 by {
        assert(cs.drop_last()[k] == cs[k]);
    }
    assert(cs.last() == cs[cs.len() - 1]);
}

proof fn tuple_count_pos(cs: Seq<Seq<Term>>)
    requires
        lens_pos(cs),
    ensures
        ws::tuple_count(cs) >= 1,
    decreases cs.len(),
{
    reveal_with_fuel(ws::tuple_count, 1);
    if cs.len() > 0 {
        lens_pos_pre(cs);
        tuple_count_pos(cs.drop_last());
        let a = ws::tuple_count(cs.drop_last());
        let b = cs.last().len();
        assert(a * b >= 1) by (nonlinear_arith)
            requires
                a >= 1,
                b >= 1,
        ;
    }
}

proof fn digits_shape(cs: Seq<Seq<Term>>, i: nat)
    requires
        lens_pos(cs),
    ensures
        ws::digits(cs, i).len() == cs.len(),
        forall|k: int| 0 <= k < cs.len() ==> #[trigger] ws::digits(cs, i)[k] < cs[k].len(),
    decreases cs.len(),
{
    reveal_with_fuel(ws::digits, 1);
    if cs.len() > 0 {
        let pre = cs.drop_last();
        let l = cs.last().len();
        lens_pos_pre(cs);
        digits_shape(pre, i / l);
        let ds = ws::digits(cs, i);
        assert forall|k: int| 0 <= k < cs.len() implies #[trigger] ds[k] < cs[k].len() by {
            if k < cs.len() - 1 {
                assert(ds[k] == ws::digits(pre, i / l)[k]);
                assert(pre[k] == cs[k]);
            } else {
                assert(ds[k] == i % l);
                assert(cs[k] == cs.last());
            }
        }
    }
}

proof fn digits_zero(cs: Seq<Seq<Term>>)
    requires
        lens_pos(cs),
    ensures
        ws::digits(cs, 0) =~= zeros(cs.len()),
    decreases cs.len(),
{
    reveal_with_fuel(ws::digits, 1);
    if cs.len() > 0 {
        let pre = cs.drop_last();
        let l = cs.last().len();
        lens_pos_pre(cs);
        digits_zero(pre);
        assert(0nat / l == 0);
        assert(0nat % l == 0);
    }
}

proof fn advance_digits(cs: Seq<Seq<Term>>, i: nat)
    requires
        lens_pos(cs),
        i < ws::tuple_count(cs),
    ensures
        advance(cs, ws::digits(cs, i)) == if i + 1 < ws::tuple_count(cs) {
            Some(ws::digits(cs, (i + 1) as nat))
        } else {
            None
        },
    decreases cs.len(),
{
    reveal_with_fuel(ws::digits, 1);
    reveal_with_fuel(ws::tuple_count, 1);
    if cs.len() > 0 {
        let pre = cs.drop_last();
        let l = cs.last().len();
        let tp = ws::tuple_count(pre);
        lens_pos_pre(cs);
        tuple_count_pos(pre);
        let q = i / l;
        let r = i % l;
        lemma_fundamental_div_mod(i as int, l as int);
        assert(ws::tuple_count(cs) == tp * l);
        assert(q < tp) by (nonlinear_arith)
            requires
                i == l * q + r,
                0 <= r,
                i < tp * l,
                l >= 1,
                q >= 0,
        ;
        assert(i + 1 == q * l + (r + 1)) by (nonlinear_arith)
            requires
                i == l * q + r,
        ;
        assert(i + 1 == (q + 1) * l + 0 || r + 1 != l) by (nonlinear_arith)
            requires
                i == l * q + r,
        ;
        let ds = ws::digits(cs, i);
        digits_shape(pre, q);
        assert(ds == ws::digits(pre, q) + seq![r]);
        assert(ds.drop_last() =~= ws::digits(pre, q));
        assert(ds.last() == r);
        assert(r < l);
        if r + 1 < l {
            lemma_fundamental_div_mod_converse((i + 1) as int, l as int, q as int, (r + 1) as int);
            assert(i + 1 < tp * l) by (nonlinear_arith)
                requires
                    i == l * q + r,
                    r + 1 < l,
                    q < tp,
                    r >= 0,
                    q >= 0,
            ;
            assert(ws::digits(cs, (i + 1) as nat) == ws::digits(pre, q) + seq![(r + 1) as nat]);
            assert(ws::digits(pre, q).push((r + 1) as nat) =~= ws::digits(pre, q) + seq![
                (r + 1) as nat,
            ]);
        } else {
            advance_digits(pre, q);
            if q + 1 < tp {
                lemma_fundamental_div_mod_converse((i + 1) as int, l as int, (q + 1) as int, 0);
                assert(i + 1 < tp * l) by (nonlinear_arith)
                    requires
                        i == l * q + r,
                        r + 1 == l,
                        q + 1 < tp,
                        q >= 0,
                ;
                assert(ws::digits(cs, (i + 1) as nat) == ws::digits(pre, (q + 1) as nat) + seq![
                    0nat,
                ]);
                assert(ws::digits(pre, (q + 1) as nat).push(0nat) =~= ws::digits(
                    pre,
                    (q + 1) as nat,
                ) + seq![0nat]);
            } else {
                assert(i + 1 == tp * l) by (nonlinear_arith)
                    requires
                        i == l * q + r,
                        r + 1 == l,
                        q + 1 == tp,
                ;
            }
        }
    }
}

// Advance ds in place; false past the last tuple (ds then unspecified).
fn advance_exec(ds: &mut Vec<usize>, cs: &Vec<Vec<T>>) -> (out: bool)
    requires
        old(ds).len() == cs.len(),
        forall|k: int| 0 <= k < cs.len() ==> (#[trigger] old(ds)@[k]) < cs@[k].len(),
        forall|k: int| 0 <= k < cs.len() ==> (#[trigger] cs@[k]).len() >= 1,
    ensures
        final(ds).len() == cs.len(),
        forall|k: int| 0 <= k < cs.len() ==> (#[trigger] final(ds)@[k]) < cs@[k].len(),
        out == advance(seqs_models(cs@), dnat(old(ds)@)) is Some,
        out ==> advance(seqs_models(cs@), dnat(old(ds)@)) == Some(dnat(final(ds)@)),
{
    let ghost cms = seqs_models(cs@);
    let ghost o = dnat(ds@);
    let ghost n = cs.len() as nat;
    let mut k = ds.len();
    proof {
        assert(cms.take(n as int) =~= cms);
        assert(o.take(n as int) =~= o);
        match advance(cms, o) {
            Some(q) => {
                assert(q + zeros(0) =~= q);
            },
            None => {},
        }
    }
    while k > 0
        invariant
            k <= n,
            ds.len() == n,
            n == cs.len(),
            cms == seqs_models(cs@),
            o == dnat(old(ds)@),
            o.len() == n,
            forall|j: int| 0 <= j < k ==> #[trigger] dnat(ds@)[j] == o[j],
            forall|j: int| k <= j < n ==> #[trigger] ds@[j] == 0,
            forall|j: int| 0 <= j < n ==> (#[trigger] ds@[j]) < cs@[j].len(),
            forall|j: int| 0 <= j < n ==> (#[trigger] cs@[j]).len() >= 1,
            advance(cms, o) == lift(advance(cms.take(k as int), o.take(k as int)), (n - k) as nat),
        decreases k,
    {
        k -= 1;
        let ghost ck = cms.take(k as int + 1);
        let ghost ok = o.take(k as int + 1);
        proof {
            assert(ck.drop_last() =~= cms.take(k as int));
            assert(ok.drop_last() =~= o.take(k as int));
            assert(ck.last() == cms[k as int]);
            assert(ok.last() == o[k as int]);
            assert(cms[k as int].len() == cs@[k as int].len());
            assert(dnat(ds@)[k as int] == ds@[k as int] as nat);
            reveal_with_fuel(advance, 1);
        }
        let ghost pre = ds@;
        if ds[k] + 1 < cs[k].len() {
            let v = ds[k] + 1;
            ds.set(k, v);
            proof {
                let fin = o.take(k as int).push((o[k as int] + 1) as nat) + zeros(
                    (n - k - 1) as nat,
                );
                assert(advance(ck, ok) == Some(o.take(k as int).push((o[k as int] + 1) as nat)));
                assert forall|j: int| 0 <= j < n implies #[trigger] dnat(ds@)[j] == fin[j] by {
                    if j < k {
                        assert(dnat(pre)[j] == o[j]);
                        assert(ds@[j] == pre[j]);
                    } else if j > k {
                        assert(pre[j] == 0);
                        assert(ds@[j] == pre[j]);
                    }
                }
                assert(dnat(ds@) =~= fin);
            }
            return true;
        }
        ds.set(k, 0);
        proof {
            assert forall|j: int| 0 <= j < k implies #[trigger] dnat(ds@)[j] == o[j] by {
                assert(dnat(pre)[j] == o[j]);
                assert(ds@[j] == pre[j]);
            }
            assert forall|j: int| k <= j < n implies #[trigger] ds@[j] == 0 by {
                if j > k {
                    assert(pre[j] == 0);
                }
            }
        }
        proof {
            match advance(cms.take(k as int), o.take(k as int)) {
                Some(q) => {
                    assert(q.push(0nat) + zeros((n - k - 1) as nat) =~= q + zeros((n - k) as nat));
                },
                None => {},
            }
        }
    }
    proof {
        assert(cms.take(0) =~= Seq::<Seq<Term>>::empty());
        reveal_with_fuel(advance, 1);
    }
    false
}

// The tuple at the digits ds.
fn tuple_at(arena: &ETermArena, cs: &Vec<Vec<T>>, ds: &Vec<usize>) -> (out: Vec<T>)
    requires
        arena_ok(arena),
        seqs_valid(arena.nodes@, cs@),
        ds.len() == cs.len(),
        forall|k: int| 0 <= k < cs.len() ==> (#[trigger] ds@[k]) < cs@[k].len(),
    ensures
        valid_all(arena.nodes@, out@),
        models(out@) =~= Seq::new(
            cs.len() as nat,
            |k: int| seqs_models(cs@)[k][dnat(ds@)[k] as int],
        ),
{
    let mut out: Vec<T> = Vec::new();
    let mut k = 0usize;
    while k < cs.len()
        invariant
            arena_ok(arena),
            seqs_valid(arena.nodes@, cs@),
            ds.len() == cs.len(),
            forall|j: int| 0 <= j < cs.len() ==> (#[trigger] ds@[j]) < cs@[j].len(),
            k <= cs.len(),
            out.len() == k,
            valid_all(arena.nodes@, out@),
            forall|j: int| 0 <= j < k ==> (#[trigger] out@[j])@ == cs@[j]@[ds@[j] as int]@,
        decreases cs.len() - k,
    {
        proof {
            assert(valid_all(arena.nodes@, cs@[k as int]@));
            assert(valid(arena.nodes@, &cs@[k as int]@[ds@[k as int] as int]));
        }
        let x = cs[k][ds[k]].cp();
        proof {
            extend(arena.nodes@, out@, x);
        }
        out.push(x);
        k += 1;
    }
    out
}

// --- D6: one group ---
// The first winning tuple (`ws::first_win` from 0): its digits, its pairs, its index.
fn first_winner(
    arena: &mut ETermArena,
    g: &Group,
    sel: &ESel,
    cs: &Vec<Vec<T>>,
    db: &Vec<EClause>,
    ix: &EClauseIndex,
    p: Ghost<Seq<DocClause>>,
) -> (out: Option<(Vec<usize>, Vec<Binding>, Ghost<nat>)>)
    requires
        arena_ok(old(arena)),
        group_valid(old(arena).nodes@, g),
        ops_valid(old(arena).nodes@, sel),
        valid_all(old(arena).nodes@, sel.chain@),
        arith_goals(sel.chain@),
        seqs_valid(old(arena).nodes@, cs@),
        seqs_models(cs@) == ws::cand_lists(g@, sel@),
        cs.len() == sel.ops.len(),
        forall|j: int| 0 <= j < cs.len() ==> (#[trigger] cs@[j]).len() >= 1,
        db_valid(old(arena).nodes@, db@),
        db_view(old(arena).nodes@, db@) == p@,
        index_over(*ix, p@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        match out {
            Some((ds, pairs, i)) => {
                &&& ws::first_win(p@, g@, sel@, 0) == Some(i@)
                &&& dnat(ds@) == ws::digits(seqs_models(cs@), i@)
                &&& map_valid(final(arena).nodes@, pairs@)
                &&& ws::tuple_pairs(g@, sel@, ws::nth_tuple(seqs_models(cs@), i@)) == Some(
                    map_model(pairs@),
                )
            },
            None => ws::first_win(p@, g@, sel@, 0) is None,
        },
{
    let ghost start = arena.nodes@;
    let ghost cms = seqs_models(cs@);
    proof {
        assert(lens_pos(cms)) by {
            assert forall|k: int| 0 <= k < cms.len() implies (#[trigger] cms[k]).len() >= 1 by {
                assert(cms[k].len() == cs@[k].len());
            }
        }
        tuple_count_pos(cms);
        digits_zero(cms);
    }
    let mut ds: Vec<usize> = Vec::new();
    let mut k = 0usize;
    while k < cs.len()
        invariant
            k <= cs.len(),
            ds.len() == k,
            forall|j: int| 0 <= j < k ==> #[trigger] ds@[j] == 0,
        decreases cs.len() - k,
    {
        ds.push(0);
        k += 1;
    }
    let ghost mut i: nat = 0;
    proof {
        assert(dnat(ds@) =~= ws::digits(cms, 0));
    }
    loop
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            group_valid(arena.nodes@, g),
            ops_valid(arena.nodes@, sel),
            valid_all(arena.nodes@, sel.chain@),
            arith_goals(sel.chain@),
            seqs_valid(arena.nodes@, cs@),
            cms == seqs_models(cs@),
            cms == ws::cand_lists(g@, sel@),
            cs.len() == sel.ops.len(),
            lens_pos(cms),
            forall|j: int| 0 <= j < cs.len() ==> (#[trigger] cs@[j]).len() >= 1,
            db_valid(arena.nodes@, db@),
            db_view(arena.nodes@, db@) == p@,
            index_over(*ix, p@),
            ds.len() == cs.len(),
            forall|j: int| 0 <= j < cs.len() ==> (#[trigger] ds@[j]) < cs@[j].len(),
            i < ws::tuple_count(cms),
            dnat(ds@) == ws::digits(cms, i),
            ws::first_win(p@, g@, sel@, 0) == ws::first_win(p@, g@, sel@, i),
        decreases ws::tuple_count(cms) - i,
    {
        let t = tuple_at(arena, cs, &ds);
        proof {
            assert(models(t@) =~= ws::nth_tuple(cms, i));
        }
        let ghost before = arena.nodes@;
        let won = tuple_win(arena, g, sel, &t, db, ix, p);
        proof {
            crate::k2_load::prefix_chain(start, before, arena.nodes@);
            group_prefix(before, arena.nodes@, g);
            ops_prefix(before, arena.nodes@, sel);
            prefix_all(before, arena.nodes@, sel.chain@);
            seqs_prefix(before, arena.nodes@, cs@);
            crate::k2_engine::db_models_prefix(before, arena.nodes@, db@);
            reveal_with_fuel(ws::first_win, 1);
        }
        match won {
            Some(pairs) => {
                proof {
                    assert(ws::first_win(p@, g@, sel@, i) == Some(i));
                }
                return Some((ds, pairs, Ghost(i)));
            },
            None => {},
        }
        proof {
            advance_digits(cms, i);
            digits_shape(cms, i);
        }
        if !advance_exec(&mut ds, cs) {
            proof {
                assert(i + 1 == ws::tuple_count(cms));
                assert(ws::first_win(p@, g@, sel@, i + 1) == None::<nat>);
            }
            return None;
        }
        proof {
            i = i + 1;
        }
    }
}

// `first_winner` over the document's program.
fn first_winner_in(
    arena: &mut ETermArena,
    g: &Group,
    sel: &ESel,
    cs: &Vec<Vec<T>>,
    prog: &DocProgram,
    p: Ghost<Seq<DocClause>>,
) -> (out: Option<(Vec<usize>, Vec<Binding>, Ghost<nat>)>)
    requires
        arena_ok(old(arena)),
        group_valid(old(arena).nodes@, g),
        ops_valid(old(arena).nodes@, sel),
        valid_all(old(arena).nodes@, sel.chain@),
        arith_goals(sel.chain@),
        seqs_valid(old(arena).nodes@, cs@),
        seqs_models(cs@) == ws::cand_lists(g@, sel@),
        cs.len() == sel.ops.len(),
        forall|j: int| 0 <= j < cs.len() ==> (#[trigger] cs@[j]).len() >= 1,
        program_ok(old(arena).nodes@, prog, p@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        match out {
            Some((ds, _, i)) => ws::first_win(p@, g@, sel@, 0) == Some(i@) && dnat(ds@)
                == ws::digits(seqs_models(cs@), i@),
            None => ws::first_win(p@, g@, sel@, 0) is None,
        },
{
    proof {
        reveal(program_ok);
    }
    first_winner(arena, g, sel, cs, &prog.db, &prog.ix, p)
}

fn select_group(
    arena: &mut ETermArena,
    g: &Group,
    db: &Vec<EClause>,
    ix: &EClauseIndex,
    p: Ghost<Seq<DocClause>>,
) -> (out: Group)
    requires
        arena_ok(old(arena)),
        group_valid(old(arena).nodes@, g),
        db_valid(old(arena).nodes@, db@),
        db_view(old(arena).nodes@, db@) == p@,
        index_over(*ix, p@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        group_valid(final(arena).nodes@, &out),
        out@ == ws::select_group(p@, g@),
{
    let ghost start = arena.nodes@;
    match &g.sel {
        None => {
            proof {
                reveal(sel_model);
            }
            copy_group(arena, g)
        },
        Some(sel) => {
            proof {
                reveal(sel_model);
                assert(g@.sel == Some(sel@));
                assert(ops_valid(arena.nodes@, sel));
            }
            let cs = cand_lists(arena, g, sel);
            let ghost n1 = arena.nodes@;
            proof {
                group_prefix(start, n1, g);
                ops_prefix(start, n1, sel);
                prefix_all(start, n1, sel.chain@);
                crate::k2_engine::db_models_prefix(start, n1, db@);
            }
            match first_winner(arena, g, sel, &cs, db, ix, p) {
                Some((_, pairs, _)) => {
                    let ghost n2 = arena.nodes@;
                    proof {
                        group_prefix(n1, n2, g);
                        crate::k2_load::prefix_chain(start, n1, n2);
                    }
                    Group {
                        k: g.k,
                        pairs,
                        clauses: copy_clauses(arena, &g.clauses),
                        sel: copy_sel(&g.sel),
                    }
                },
                None => {
                    let ghost n2 = arena.nodes@;
                    proof {
                        group_prefix(n1, n2, g);
                        crate::k2_load::prefix_chain(start, n1, n2);
                    }
                    copy_group(arena, g)
                },
            }
        },
    }
}

// The selection over a document's projection (D6); db = the document's
// program (`replay::doc_db`), ix its clause index.
pub fn select_doc(
    arena: &mut ETermArena,
    ps: &Vec<Projected>,
    prog: &DocProgram,
    p: Ghost<Seq<DocClause>>,
) -> (out: Vec<Projected>)
    requires
        arena_ok(old(arena)),
        projections_valid(old(arena).nodes@, ps@),
        program_ok(old(arena).nodes@, prog, p@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        projections_valid(final(arena).nodes@, out@),
        project_models(out@) == ws::select_doc(project_models(ps@), p@),
{
    hide(ws::select_group);
    proof {
        reveal(program_ok);
    }
    let db = &prog.db;
    let ix = &prog.ix;
    let ghost start = arena.nodes@;
    let mut out: Vec<Projected> = Vec::new();
    let mut i = 0usize;
    while i < ps.len()
        invariant
            arena_ok(arena),
            start.is_prefix_of(arena.nodes@),
            projections_valid(arena.nodes@, ps@),
            db_valid(arena.nodes@, db@),
            db_view(arena.nodes@, db@) == p@,
            index_over(*ix, p@),
            i <= ps.len(),
            out.len() == i,
            projections_valid(arena.nodes@, out@),
            forall|j: int|
                0 <= j < i ==> (#[trigger] out@[j])@ == ws::select_doc(project_models(ps@), p@)[j],
        decreases ps.len() - i,
    {
        let pr = &ps[i];
        proof {
            assert(projected_valid(arena.nodes@, &ps@[i as int]));
        }
        let mut groups: Vec<Group> = Vec::new();
        let mut k = 0usize;
        while k < pr.groups.len()
            invariant
                arena_ok(arena),
                start.is_prefix_of(arena.nodes@),
                projections_valid(arena.nodes@, ps@),
                projections_valid(arena.nodes@, out@),
                groups_valid(arena.nodes@, pr.groups@),
                clauses_valid(arena.nodes@, pr.records@),
                db_valid(arena.nodes@, db@),
                db_view(arena.nodes@, db@) == p@,
                index_over(*ix, p@),
                k <= pr.groups.len(),
                groups.len() == k,
                groups_valid(arena.nodes@, groups@),
                forall|j: int|
                    0 <= j < k ==> (#[trigger] groups@[j])@ == ws::select_group(p@, pr.groups@[j]@),
            decreases pr.groups.len() - k,
        {
            proof {
                assert(group_valid(arena.nodes@, &pr.groups@[k as int]));
            }
            let ghost before = arena.nodes@;
            let next = select_group(arena, &pr.groups[k], db, ix, p);
            proof {
                projections_prefix(before, arena.nodes@, ps@);
                projections_prefix(before, arena.nodes@, out@);
                groups_prefix(before, arena.nodes@, pr.groups@);
                groups_prefix(before, arena.nodes@, groups@);
                crate::m6_model::clauses_prefix(before, arena.nodes@, pr.records@);
                crate::k2_engine::db_models_prefix(before, arena.nodes@, db@);
            }
            let ghost prior = groups@;
            groups.push(next);
            proof {
                assert forall|j: int| 0 <= j < groups.len() implies #[trigger] group_valid(
                    arena.nodes@,
                    &groups@[j],
                ) by {
                    if j < k {
                        assert(groups@[j] == prior[j]);
                    }
                }
                assert forall|j: int| 0 <= j < k + 1 implies (#[trigger] groups@[j])@
                    == ws::select_group(p@, pr.groups@[j]@) by {
                    if j < k {
                        assert(groups@[j] == prior[j]);
                    }
                }
            }
            k += 1;
        }
        let records = copy_clauses(arena, &pr.records);
        let next = Projected { s: pr.s, groups, records };
        proof {
            reveal(ws::select_doc);
            let want = ws::select_doc(project_models(ps@), p@)[i as int];
            assert(project_models(ps@)[i as int] == pr@);
            assert(group_models(next.groups@) =~= want.groups) by {
                assert forall|j: int| 0 <= j < next.groups.len() implies group_models(
                    next.groups@,
                )[j] == want.groups[j] by {
                    assert(group_models(next.groups@)[j] == next.groups@[j]@);
                    assert(pr@.groups[j] == pr.groups@[j]@);
                }
            }
            assert(next@ == want);
        }
        let ghost prior = out@;
        out.push(next);
        proof {
            assert forall|j: int| 0 <= j < out.len() implies #[trigger] projected_valid(
                arena.nodes@,
                &out@[j],
            ) by {
                if j < i {
                    assert(out@[j] == prior[j]);
                }
            }
            assert forall|j: int| 0 <= j < i + 1 implies (#[trigger] out@[j])@ == ws::select_doc(
                project_models(ps@),
                p@,
            )[j] by {
                if j < i {
                    assert(out@[j] == prior[j]);
                }
            }
        }
        i += 1;
    }
    proof {
        reveal(ws::select_doc);
        assert_seqs_equal!(project_models(out@) == ws::select_doc(project_models(ps@), p@), j => {
            assert(project_models(out@)[j] == out@[j]@);
        });
    }
    out
}

fn copy_group(arena: &ETermArena, g: &Group) -> (out: Group)
    requires
        arena_ok(arena),
        group_valid(arena.nodes@, g),
    ensures
        group_valid(arena.nodes@, &out),
        out@ == g@,
{
    Group {
        k: g.k,
        pairs: copy_map(arena, &g.pairs),
        clauses: copy_clauses(arena, &g.clauses),
        sel: copy_sel(&g.sel),
    }
}

fn copy_projections(arena: &ETermArena, ps: &Vec<Projected>) -> (out: Vec<Projected>)
    requires
        arena_ok(arena),
        projections_valid(arena.nodes@, ps@),
    ensures
        projections_valid(arena.nodes@, out@),
        project_models(out@) == project_models(ps@),
{
    let mut out: Vec<Projected> = Vec::new();
    let mut i = 0usize;
    while i < ps.len()
        invariant
            arena_ok(arena),
            projections_valid(arena.nodes@, ps@),
            i <= ps.len(),
            out.len() == i,
            projections_valid(arena.nodes@, out@),
            forall|j: int| 0 <= j < i ==> (#[trigger] out@[j])@ == ps@[j]@,
        decreases ps.len() - i,
    {
        let pr = &ps[i];
        proof {
            assert(projected_valid(arena.nodes@, &ps@[i as int]));
        }
        let mut groups: Vec<Group> = Vec::new();
        let mut k = 0usize;
        while k < pr.groups.len()
            invariant
                arena_ok(arena),
                groups_valid(arena.nodes@, pr.groups@),
                k <= pr.groups.len(),
                groups.len() == k,
                groups_valid(arena.nodes@, groups@),
                forall|j: int| 0 <= j < k ==> (#[trigger] groups@[j])@ == pr.groups@[j]@,
            decreases pr.groups.len() - k,
        {
            proof {
                assert(group_valid(arena.nodes@, &pr.groups@[k as int]));
            }
            let next = copy_group(arena, &pr.groups[k]);
            let ghost before = groups@;
            groups.push(next);
            proof {
                assert forall|j: int| 0 <= j < groups.len() implies #[trigger] group_valid(
                    arena.nodes@,
                    &groups@[j],
                ) by {
                    if j < k {
                        assert(groups@[j] == before[j]);
                    }
                }
            }
            k += 1;
        }
        let records = copy_clauses(arena, &pr.records);
        let next = Projected { s: pr.s, groups, records };
        proof {
            assert_seqs_equal!(group_models(next.groups@) == group_models(pr.groups@));
        }
        let ghost before = out@;
        out.push(next);
        proof {
            assert forall|j: int| 0 <= j < out.len() implies #[trigger] projected_valid(
                arena.nodes@,
                &out@[j],
            ) by {
                if j < i {
                    assert(out@[j] == before[j]);
                }
            }
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(project_models(out@) == project_models(ps@));
    }
    out
}

// --- the walk report (contract ws P6) ---
// The candidate lists as a list of lists.
fn cands_term(arena: &mut ETermArena, cs: &Vec<Vec<T>>) -> (out: T)
    requires
        arena_ok(old(arena)),
        seqs_valid(old(arena).nodes@, cs@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == ckc_spec::engine::list_term(
            seqs_models(cs@).map_values(|c: Seq<Term>| ckc_spec::engine::list_term(c)),
        ),
{
    let ghost start = arena.nodes@;
    let ghost cms = seqs_models(cs@);
    let mut ls: Vec<T> = Vec::new();
    let mut q = 0usize;
    while q < cs.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            seqs_valid(arena.nodes@, cs@),
            cms == seqs_models(cs@),
            q <= cs.len(),
            ls.len() == q,
            valid_all(arena.nodes@, ls@),
            forall|j: int|
                0 <= j < q ==> (#[trigger] ls@[j])@ == ckc_spec::engine::list_term(cms[j]),
        decreases cs.len() - q,
    {
        let ghost here = arena.nodes@;
        let l = list(arena, &cs[q]);
        proof {
            seqs_prefix(here, arena.nodes@, cs@);
            prefix_all(here, arena.nodes@, ls@);
            crate::k2_load::prefix_chain(start, here, arena.nodes@);
            extend(arena.nodes@, ls@, l);
        }
        let ghost before = ls@;
        ls.push(l);
        proof {
            assert forall|j: int| 0 <= j < q + 1 implies (#[trigger] ls@[j])@
                == ckc_spec::engine::list_term(cms[j]) by {
                if j < q {
                    assert(ls@[j] == before[j]);
                }
            }
        }
        q += 1;
    }
    proof {
        assert(models(ls@) =~= cms.map_values(|c: Seq<Term>| ckc_spec::engine::list_term(c)));
    }
    list(arena, &ls)
}

// `winner([digits])` for a winner, else `none`.
fn winner_term(arena: &mut ETermArena, won: Option<(Vec<usize>, Vec<Binding>, Ghost<nat>)>) -> (out:
    T)
    requires
        arena_ok(old(arena)),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == match won {
            Some((ds, _, _)) => Term::Comp(
                ckc_spec::v1text::ascii("winner"@),
                seq![
                    ckc_spec::engine::list_term(dnat(ds@).map_values(|d: nat| Term::Int(d as int))),
                ],
            ),
            None => ckc_spec::replay::atom("none"@),
        },
{
    let ghost start = arena.nodes@;
    match won {
        Some((ds, _, _)) => {
            let mut ints: Vec<T> = Vec::new();
            let mut q = 0usize;
            while q < ds.len()
                invariant
                    arena_ok(arena),
                    start == old(arena).nodes@,
                    start.is_prefix_of(arena.nodes@),
                    q <= ds.len(),
                    ints.len() == q,
                    valid_all(arena.nodes@, ints@),
                    forall|j: int|
                        0 <= j < q ==> (#[trigger] ints@[j])@ == Term::Int(ds@[j] as int),
                decreases ds.len() - q,
            {
                let ghost here = arena.nodes@;
                let t = crate::m6_term::int(arena, ds[q]);
                proof {
                    prefix_all(here, arena.nodes@, ints@);
                    crate::k2_load::prefix_chain(start, here, arena.nodes@);
                    extend(arena.nodes@, ints@, t);
                }
                let ghost before = ints@;
                ints.push(t);
                proof {
                    assert forall|j: int| 0 <= j < q + 1 implies (#[trigger] ints@[j])@
                        == Term::Int(ds@[j] as int) by {
                        if j < q {
                            assert(ints@[j] == before[j]);
                        }
                    }
                }
                q += 1;
            }
            let ghost here = arena.nodes@;
            let l = list(arena, &ints);
            let ghost here2 = arena.nodes@;
            proof {
                crate::k2_load::prefix_chain(start, here, here2);
                assert(models(ints@) =~= dnat(ds@).map_values(|d: nat| Term::Int(d as int)));
                reveal_strlit("winner");
            }
            let w = c1(arena, &Sym::Winner, &l);
            proof {
                crate::k2_load::prefix_chain(start, here2, arena.nodes@);
            }
            w
        },
        None => {
            proof {
                reveal_strlit("none");
            }
            named(arena, &Sym::None)
        },
    }
}

fn fragment_term(arena: &mut ETermArena, fr: bool) -> (out: T)
    requires
        arena_ok(old(arena)),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == Term::Comp(
            ckc_spec::v1text::ascii("fragment"@),
            seq![
                ckc_spec::replay::atom(
                    if fr {
                        "true"@
                    } else {
                        "false"@
                    },
                ),
            ],
        ),
{
    let ghost start = arena.nodes@;
    proof {
        reveal_strlit("fragment");
        reveal_strlit("true");
        reveal_strlit("false");
    }
    let a = if fr {
        named(arena, &Sym::True)
    } else {
        named(arena, &Sym::False)
    };
    let ghost here = arena.nodes@;
    let out = c1(arena, &Sym::Fragment, &a);
    proof {
        crate::k2_load::prefix_chain(start, here, arena.nodes@);
    }
    out
}

// `ws::walk_term`: candidates per operand, the winner's digits, fragment.
fn walk_term(
    arena: &mut ETermArena,
    prog: &DocProgram,
    g: &Group,
    sel: &ESel,
    s: usize,
    p: Ghost<Seq<DocClause>>,
) -> (out: T)
    requires
        arena_ok(old(arena)),
        program_ok(old(arena).nodes@, prog, p@),
        group_valid(old(arena).nodes@, g),
        g@.sel == Some(sel@),
        ops_valid(old(arena).nodes@, sel),
        valid_all(old(arena).nodes@, sel.chain@),
        arith_goals(sel.chain@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        valid(final(arena).nodes@, &out),
        out@ == ws::walk_term(p@, g@, sel@, s as nat),
{
    let ghost start = arena.nodes@;
    let cs = cand_lists(arena, g, sel);
    let ghost cms = seqs_models(cs@);
    let ghost n1 = arena.nodes@;
    proof {
        group_prefix(start, n1, g);
        ops_prefix(start, n1, sel);
        prefix_all(start, n1, sel.chain@);
        program_prefix(start, n1, prog, p@);
        assert forall|j: int| 0 <= j < cs.len() implies (#[trigger] cs@[j]).len() >= 1 by {
            assert(cms[j].len() == cs@[j].len());
        }
    }
    let lists = cands_term(arena, &cs);
    let ghost n2 = arena.nodes@;
    proof {
        group_prefix(n1, n2, g);
        ops_prefix(n1, n2, sel);
        prefix_all(n1, n2, sel.chain@);
        seqs_prefix(n1, n2, cs@);
        program_prefix(n1, n2, prog, p@);
        crate::k2_load::prefix_chain(start, n1, n2);
    }
    let won = first_winner_in(arena, g, sel, &cs, prog, p);
    let ghost wm = match &won {
        Some((ds, _, i)) => Some((dnat(ds@), i@)),
        None => None,
    };
    let ghost n3 = arena.nodes@;
    proof {
        prefix(n2, n3, &lists);
        crate::k2_load::prefix_chain(start, n2, n3);
    }
    let win = winner_term(arena, won);
    let ghost n4 = arena.nodes@;
    proof {
        prefix(n3, n4, &lists);
        crate::k2_load::prefix_chain(start, n3, n4);
        group_prefix(start, n4, g);
        ops_prefix(start, n4, sel);
        prefix_all(start, n4, sel.chain@);
        program_prefix(start, n4, prog, p@);
    }
    let fr = crate::m6_fragment::in_fragment(arena, prog, g, sel, p);
    let ghost n5 = arena.nodes@;
    let frag = fragment_term(arena, fr);
    let ghost n6 = arena.nodes@;
    let st = crate::m6_term::int(arena, s);
    let ghost n7 = arena.nodes@;
    let kt = crate::m6_term::int(arena, g.k);
    let ghost n8 = arena.nodes@;
    let var = c1(arena, &Sym::Variant, &kt);
    let ghost n9 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(n4, n5, n6);
        crate::k2_load::prefix_chain(n4, n6, n7);
        crate::k2_load::prefix_chain(n4, n7, n8);
        crate::k2_load::prefix_chain(n4, n8, n9);
        crate::k2_load::prefix_chain(start, n4, n9);
        prefix(n4, n9, &lists);
        prefix(n4, n9, &win);
        prefix(n6, n9, &frag);
        prefix(n7, n9, &st);
        reveal_strlit("variant");
        reveal_strlit("$guideline_walk");
    }
    let mut parts: Vec<T> = Vec::new();
    parts.push(st);
    parts.push(var);
    parts.push(lists);
    parts.push(win);
    parts.push(frag);
    let out = c(arena, &Sym::DollarGuidelineWalk, &parts);
    proof {
        crate::k2_load::prefix_chain(start, n9, arena.nodes@);
        walk_term_probe(p@, g@, sel@, s as nat);
        let cl = ws::cand_lists(g@, sel@);
        assert(cms == cl);
        match wm {
            Some((dm, i)) => {
                assert(ws::first_win(p@, g@, sel@, 0) == Some(i));
                assert(dm == ws::digits(cl, i));
            },
            None => {
                assert(ws::first_win(p@, g@, sel@, 0) is None);
            },
        }
        assert(var@ == ckc_spec::emit::variant_slot(g@.k));
        assert(models(parts@) =~= seq![st@, var@, lists@, win@, frag@]);
        let wt = ws::walk_term(p@, g@, sel@, s as nat);
        assert(ckc_spec::engine::args_of(wt)[0] == st@);
        assert(ckc_spec::engine::args_of(wt)[1] == var@);
        assert(ckc_spec::engine::args_of(wt)[2] == lists@);
        assert(ckc_spec::engine::args_of(wt)[3] == win@);
        assert(ckc_spec::engine::args_of(wt)[4] == frag@);
        assert(ckc_spec::engine::args_of(wt).len() == 5);
        assert(ckc_spec::engine::args_of(wt) =~= models(parts@));
        assert(crate::m6_symbols::symbol(&Sym::DollarGuidelineWalk) == ckc_spec::v1text::ascii(
            "$guideline_walk"@,
        ));
        assert(out@ == Term::Comp(
            crate::m6_symbols::symbol(&Sym::DollarGuidelineWalk),
            models(parts@),
        ));
        assert(wt == Term::Comp(
            ckc_spec::v1text::ascii("$guideline_walk"@),
            ckc_spec::engine::args_of(wt),
        ));
    }
    out
}

proof fn walk_term_probe(p: Seq<DocClause>, g: spec::Group, sel: ws::Sel, s: nat)
    ensures
        ws::walk_term(p, g, sel, s) == Term::Comp(
            ckc_spec::v1text::ascii("$guideline_walk"@),
            seq![
                Term::Int(s as int),
                ckc_spec::emit::variant_slot(g.k),
                ckc_spec::engine::list_term(
                    ws::cand_lists(g, sel).map_values(
                        |c: Seq<Term>| ckc_spec::engine::list_term(c),
                    ),
                ),
                match ws::first_win(p, g, sel, 0) {
                    Some(i) => Term::Comp(
                        ckc_spec::v1text::ascii("winner"@),
                        seq![
                            ckc_spec::engine::list_term(
                                ws::digits(ws::cand_lists(g, sel), i).map_values(
                                    |d: nat| Term::Int(d as int),
                                ),
                            ),
                        ],
                    ),
                    None => ckc_spec::replay::atom("none"@),
                },
                Term::Comp(
                    ckc_spec::v1text::ascii("fragment"@),
                    seq![
                        ckc_spec::replay::atom(
                            if ws::in_fragment(p, g, sel) {
                                "true"@
                            } else {
                                "false"@
                            },
                        ),
                    ],
                ),
            ],
        ),
{
}

// The walk lines of a document's projection (`ws::walks`, one term_line each).
fn walk_lines(
    arena: &mut ETermArena,
    ps: &Vec<Projected>,
    prog: &DocProgram,
    p: Ghost<Seq<DocClause>>,
) -> (out: Vec<u8>)
    requires
        arena_ok(old(arena)),
        projections_valid(old(arena).nodes@, ps@),
        program_ok(old(arena).nodes@, prog, p@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out@ == ws::walks(project_models(ps@), p@).map_values(
            |t: Term| ckc_spec::v1text::term_line(t),
        ).flatten(),
{
    hide(ws::walk_term);
    let ghost start = arena.nodes@;
    let ghost pm = project_models(ps@);
    let ghost f = |pr: spec::Projected|
        pr.groups.filter(|g: spec::Group| g.sel is Some).map_values(
            |g: spec::Group| ws::walk_term(p@, g, g.sel.unwrap(), pr.s),
        );
    let ghost line = |t: Term| ckc_spec::v1text::term_line(t);
    let mut out: Vec<u8> = Vec::new();
    let ghost mut terms: Seq<Term> = Seq::empty();
    let mut i = 0usize;
    proof {
        assert(pm.take(0).map_values(f) =~= Seq::<Seq<Term>>::empty());
        assert(terms =~= pm.take(0).map_values(f).flatten());
        assert(terms.map_values(line) =~= Seq::<Seq<u8>>::empty());
        assert(out@ =~= terms.map_values(line).flatten());
    }
    while i < ps.len()
        invariant
            arena_ok(arena),
            start == old(arena).nodes@,
            start.is_prefix_of(arena.nodes@),
            projections_valid(arena.nodes@, ps@),
            program_ok(arena.nodes@, prog, p@),
            pm == project_models(ps@),
            f == (|pr: spec::Projected|
                pr.groups.filter(|g: spec::Group| g.sel is Some).map_values(
                    |g: spec::Group| ws::walk_term(p@, g, g.sel.unwrap(), pr.s),
                )),
            line == (|t: Term| ckc_spec::v1text::term_line(t)),
            i <= ps.len(),
            terms == pm.take(i as int).map_values(f).flatten(),
            out@ == terms.map_values(line).flatten(),
        decreases ps.len() - i,
    {
        let pr = &ps[i];
        proof {
            assert(projected_valid(arena.nodes@, &ps@[i as int]));
            assert(pm[i as int] == pr@);
        }
        let ghost gm = pr@.groups;
        let ghost keep = |g: spec::Group| g.sel is Some;
        let ghost wt = |g: spec::Group| ws::walk_term(p@, g, g.sel.unwrap(), pr@.s);
        let ghost mut inner: Seq<Term> = Seq::empty();
        let mut k = 0usize;
        proof {
            reveal(Seq::filter);
            assert(gm.take(0).filter(keep).map_values(wt) =~= Seq::<Term>::empty());
        }
        while k < pr.groups.len()
            invariant
                arena_ok(arena),
                start.is_prefix_of(arena.nodes@),
                projections_valid(arena.nodes@, ps@),
                groups_valid(arena.nodes@, pr.groups@),
                program_ok(arena.nodes@, prog, p@),
                gm == pr@.groups,
                keep == (|g: spec::Group| g.sel is Some),
                wt == (|g: spec::Group| ws::walk_term(p@, g, g.sel.unwrap(), pr@.s)),
                line == (|t: Term| ckc_spec::v1text::term_line(t)),
                k <= pr.groups.len(),
                inner == gm.take(k as int).filter(keep).map_values(wt),
                out@ == (terms + inner).map_values(line).flatten(),
            decreases pr.groups.len() - k,
        {
            let g = &pr.groups[k];
            proof {
                assert(group_valid(arena.nodes@, &pr.groups@[k as int]));
                assert(gm[k as int] == g@);
                assert(gm.take(k as int + 1) == gm.take(k as int).push(gm[k as int]));
                gm.take(k as int).lemma_filter_push(gm[k as int], keep);
                reveal(sel_model);
            }
            match &g.sel {
                Some(sel) => {
                    proof {
                        assert(g@.sel == Some(sel@));
                        assert(ops_valid(arena.nodes@, sel));
                    }
                    let ghost here = arena.nodes@;
                    let t = walk_term(arena, prog, g, sel, pr.s, p);
                    let ln = crate::k2_term::term_line(arena, t.root);
                    proof {
                        projections_prefix(here, arena.nodes@, ps@);
                        groups_prefix(here, arena.nodes@, pr.groups@);
                        program_prefix(here, arena.nodes@, prog, p@);
                        crate::k2_load::prefix_chain(start, here, arena.nodes@);
                        assert(arena@[t.root as int] == t@);
                        assert(wt(gm[k as int]) == t@);
                    }
                    let ghost before = out@;
                    let mut ln2 = ln;
                    out.append(&mut ln2);
                    proof {
                        let f0 = gm.take(k as int).filter(keep);
                        assert(f0.push(gm[k as int]).map_values(wt) =~= f0.map_values(wt).push(t@));
                        let old_all = terms + inner;
                        let new_inner = inner.push(t@);
                        assert(terms + new_inner =~= old_all.push(t@));
                        assert(old_all.push(t@).map_values(line) =~= old_all.map_values(line).push(
                            line(t@),
                        ));
                        old_all.map_values(line).lemma_flatten_push(line(t@));
                        inner = new_inner;
                    }
                },
                None => {
                    proof {
                        assert(!keep(gm[k as int]));
                    }
                },
            }
            k += 1;
        }
        proof {
            assert(gm.take(k as int) =~= gm);
            assert(f(pm[i as int]) == inner);
            assert(pm.take(i as int + 1).map_values(f) =~= pm.take(i as int).map_values(f).push(
                f(pm[i as int]),
            ));
            pm.take(i as int).map_values(f).lemma_flatten_push(f(pm[i as int]));
            terms = terms + inner;
        }
        i += 1;
    }
    proof {
        assert(pm.take(i as int) =~= pm);
        assert(ws::walks(pm, p@) == pm.map_values(f).flatten());
    }
    out
}

// The walk report of a document past intake: certification's verdict first.
fn walks_after_intake(i: crate::m6_doc::Intake, docid: &[u8]) -> (out: EOut)
    requires
        crate::m6_doc::intake_ok(&i, docid@),
    ensures
        out@ == match spec::certified(
            i.doc@,
            project_models(i.ps@),
            docid@,
            crate::m6_custody::lines_model(i.lines@),
        ) {
            Err(why) => spec::certify_reject(docid@, why),
            Ok(_) => ckc_spec::replay::ok(
                ws::walks(project_models(i.ps@), ckc_spec::replay::doc_db(i.doc@)).map_values(
                    |t: Term| ckc_spec::v1text::term_line(t),
                ).flatten(),
            ),
        },
{
    hide(spec::certified);
    hide(ws::walks);
    let crate::m6_doc::Intake { mut arena, parsed, actual, ps, lines, prog, doc } = i;
    let ghost before = arena.nodes@;
    let r = crate::m6_doc::compare_projection(
        &mut arena,
        &parsed.bundles,
        &actual,
        &ps,
        &lines,
        &parsed.docid,
        &prog,
        doc,
    );
    proof {
        reveal(crate::m6_doc::comparison_output);
    }
    if r.rc != 0 {
        return r;
    }
    proof {
        projections_prefix(before, arena.nodes@, ps@);
        program_prefix(before, arena.nodes@, &prog, ckc_spec::replay::doc_db(doc@));
    }
    let bytes = walk_lines(&mut arena, &ps, &prog, Ghost(ckc_spec::replay::doc_db(doc@)));
    EOut { rc: 0, out: bytes, err: Vec::new() }
}

// `ckc certify-one walks` (contract ws P6).
#[verifier::spinoff_prover]
pub fn certify_walks_impl(
    ace: &[u8],
    asha: &[u8],
    usha: Option<&Vec<u8>>,
    docid: &[u8],
    dump: &[u8],
    pl: &[u8],
    traw: Option<&Vec<u8>>,
    tsha: Option<&Vec<u8>>,
) -> (out: EOut)
    ensures
        out@ == ws::certify_walks_output(
            ace@,
            asha@,
            ckc_spec::emit::opt_view(usha),
            docid@,
            dump@,
            pl@,
            ckc_spec::emit::opt_view(traw),
            ckc_spec::emit::opt_view(tsha),
        ),
{
    hide(ws::certify_walks_output);
    hide(spec::certify_doc);
    hide(crate::m6_doc::staged);
    hide(spec::certified);
    hide(spec::certify_reject);
    hide(ws::walks);
    hide(crate::m6_doc::intake_ok);
    proof {
        crate::m6_doc::staged_certify(
            ace@,
            asha@,
            spec::opt_view(usha),
            docid@,
            dump@,
            pl@,
            spec::opt_view(traw),
            spec::opt_view(tsha),
        );
        staged_walks(
            ace@,
            asha@,
            spec::opt_view(usha),
            docid@,
            dump@,
            pl@,
            spec::opt_view(traw),
            spec::opt_view(tsha),
        );
    }
    match crate::m6_doc::intake(ace, asha, usha, docid, dump, pl, traw, tsha) {
        Err(o) => o,
        Ok(i) => walks_after_intake(i, docid),
    }
}

// On a staged document, `certify_walks_output` reads the staged projection.
proof fn staged_walks(
    ace: Seq<u8>,
    asha: Seq<u8>,
    usha: Option<Seq<u8>>,
    docid: Seq<u8>,
    dump: Seq<u8>,
    pl: Seq<u8>,
    traw: Option<Seq<u8>>,
    tsha: Option<Seq<u8>>,
)
    ensures
        ws::certify_walks_output(ace, asha, usha, docid, dump, pl, traw, tsha)
            == match spec::certify_doc(ace, asha, usha, docid, dump, pl, traw, tsha) {
            Err(why) => spec::certify_reject(docid, why),
            Ok(_) => match crate::m6_doc::staged(ace, asha, usha, docid, dump, pl, traw, tsha) {
                Ok((doc, ps, _)) => ckc_spec::replay::ok(
                    ws::walks(ps, ckc_spec::replay::doc_db(doc)).map_values(
                        |t: Term| ckc_spec::v1text::term_line(t),
                    ).flatten(),
                ),
                Err(_) => spec::certify_reject(docid, ckc_spec::replay::atom("walks"@)),
            },
        },
{
    crate::m6_doc::staged_certify(ace, asha, usha, docid, dump, pl, traw, tsha);
}

pub proof fn cell_invariance(
    p: Seq<DocClause>,
    g: ckc_spec::emit::Group,
    sel: ws::Sel,
    t: Seq<Term>,
    u: Seq<Term>,
)
    requires
        ws::in_fragment(p, g, sel),
        t.len() == sel.ops.len(),
        u.len() == sel.ops.len(),
        forall|i: int|
            0 <= i < sel.ops.len() ==> ws::same_cell(
                ws::boundaries(g, sel, i),
                #[trigger] t[i],
                u[i],
            ),
    ensures
        forall|h: int|
            0 <= h < g.clauses.len() ==> #[trigger] ws::replay_of(p, g, sel, t, h) == ws::replay_of(
                p,
                g,
                sel,
                u,
                h,
            ),
        ws::win(p, g, sel, t) == ws::win(p, g, sel, u),
{
    // A lockstep simulation of the two replays (m6_step … m6_assemble).
    crate::m6_setup::cell_defined(g, sel, t, u);
    if ws::tuple_pairs(g, sel, t) is Some {
        let ot = crate::m6_ctx::tob(g, sel, t);
        let ou = crate::m6_ctx::tob(g, sel, u);
        let dbt = ckc_spec::engine::witness_db(ot.facts, p);
        let dbu = ckc_spec::engine::witness_db(ou.facts, p);
        assert forall|h: int| 0 <= h < g.clauses.len() implies #[trigger] ws::replay_of(
            p,
            g,
            sel,
            t,
            h,
        ) == ws::replay_of(p, g, sel, u, h) by {
            crate::m6_assemble::replay_eq(p, g, sel, h, t, u);
        }
        assert(ot.heads.len() == g.clauses.len() && ou.heads.len() == g.clauses.len());
        assert forall|h: int| 0 <= h < g.clauses.len() implies ckc_spec::engine::head_proved(
            dbt,
            #[trigger] ot.heads[h],
        ) == ckc_spec::engine::head_proved(dbu, ou.heads[h]) by {
            crate::m6_assemble::replay_eq(p, g, sel, h, t, u);
        }
        if ckc_spec::engine::heads_proved(dbt, ot.heads) {
            assert forall|h: int| 0 <= h < ou.heads.len() implies ckc_spec::engine::head_proved(
                dbu,
                #[trigger] ou.heads[h],
            ) by {
                assert(ckc_spec::engine::head_proved(dbt, ot.heads[h]));
            }
        }
        if ckc_spec::engine::heads_proved(dbu, ou.heads) {
            assert forall|h: int| 0 <= h < ot.heads.len() implies ckc_spec::engine::head_proved(
                dbt,
                #[trigger] ot.heads[h],
            ) by {
                assert(ckc_spec::engine::head_proved(dbu, ou.heads[h]));
            }
        }
    }
}

pub proof fn complete(p: Seq<DocClause>, g: ckc_spec::emit::Group, sel: ws::Sel, t: Seq<Term>)
    requires
        ws::in_fragment(p, g, sel),
        t.len() == sel.ops.len(),
        forall|i: int| 0 <= i < t.len() ==> ws::admits(sel.ops[i].cond, #[trigger] t[i]),
        ws::win(p, g, sel, t),
    ensures
        ws::first_win(p, g, sel, 0) is Some,
{
    crate::m6_cover::complete(p, g, sel, t);
}

pub proof fn default_chain(g: ckc_spec::emit::Group, sel: ws::Sel, t: Seq<Term>)
    requires
        ws::chain_shape(sel),
        t.len() == sel.ops.len(),
        forall|i: int| 0 <= i < t.len() ==> ws::admits(sel.ops[i].cond, #[trigger] t[i]),
        ws::tuple_pairs(g, sel, t) is Some,
    ensures
        ws::tuple_pairs(g, sel, ws::defaults(sel)) is Some,
{
    crate::m6_default::default_chain_holds(g, sel, t);
}

} // verus!
