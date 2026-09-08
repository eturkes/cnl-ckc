use crate::k2_engine::{EBodyItem, EClause, literal_matches, unifiable_apart};
#[cfg(verus_keep_ghost)]
use crate::k2_engine::{clause_view, db_valid, db_view};
#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::k2_term::{ENode, ETermArena};
#[cfg(verus_keep_ghost)]
use ckc_spec::engine::lit_fa;
#[cfg(verus_keep_ghost)]
use ckc_spec::replay::{indicator_rules, rules};
#[cfg(verus_keep_ghost)]
use ckc_spec::v1text::{DocClause, indicator};
#[cfg(verus_keep_ghost)]
use vstd::assert_seqs_equal;
use vstd::prelude::*;
use vstd::slice::slice_to_vec;

verus! {

pub fn indicator_exec(i: usize) -> (out: (Vec<u8>, usize))
    requires
        i < 9,
    ensures
        (out.0@, out.1 as nat) == indicator(i as int),
{
    let name: &[u8] = match i {
        0 => b"guideline_schema_version",
        1 => b"guideline_document",
        2 => b"guideline_entity",
        3 => b"guideline_cardinality",
        4 => b"guideline_event",
        5 => b"guideline_arg",
        6 => b"guideline_pp",
        7 => b"guideline_property",
        _ => b"guideline_operator",
    };
    let arity: usize = match i {
        0 => 1,
        1 | 4 | 8 => 3,
        3 => 5,
        _ => 4,
    };
    proof {
        reveal_byteslit(b"guideline_schema_version");
        reveal_strlit("guideline_schema_version");
        reveal_byteslit(b"guideline_document");
        reveal_strlit("guideline_document");
        reveal_byteslit(b"guideline_entity");
        reveal_strlit("guideline_entity");
        reveal_byteslit(b"guideline_cardinality");
        reveal_strlit("guideline_cardinality");
        reveal_byteslit(b"guideline_event");
        reveal_strlit("guideline_event");
        reveal_byteslit(b"guideline_arg");
        reveal_strlit("guideline_arg");
        reveal_byteslit(b"guideline_pp");
        reveal_strlit("guideline_pp");
        reveal_byteslit(b"guideline_property");
        reveal_strlit("guideline_property");
        reveal_byteslit(b"guideline_operator");
        reveal_strlit("guideline_operator");
        reveal(ckc_spec::v1text::ascii);
        assert(name@ == indicator(i as int).0);
    }
    (slice_to_vec(name), arity)
}

pub open spec fn rule_indices_ok(db: Seq<EClause>, indices: Seq<usize>) -> bool {
    forall|i: int|
        0 <= i < indices.len() ==> indices[i] < db.len() && db[indices[i] as int].body@.len() > 0
}

pub open spec fn indexed_clauses(nodes: Seq<ENode>, db: Seq<EClause>, indices: Seq<usize>) -> Seq<
    DocClause,
> {
    Seq::new(indices.len(), |i: int| clause_view(nodes, &db[indices[i] as int]))
}

fn indicator_step(
    arena: &ETermArena,
    db: &Vec<EClause>,
    mut out: Vec<usize>,
    index: usize,
    which: usize,
) -> (result: Vec<usize>)
    requires
        arena_ok(arena),
        db_valid(arena.nodes@, db@),
        index < db.len(),
        which < 9,
        rule_indices_ok(db@, out@),
        indexed_clauses(arena.nodes@, db@, out@) == indicator_rules(
            db_view(arena.nodes@, db@).take(index as int),
            which as int,
        ),
    ensures
        rule_indices_ok(db@, result@),
        indexed_clauses(arena.nodes@, db@, result@) == indicator_rules(
            db_view(arena.nodes@, db@).take(index as int + 1),
            which as int,
        ),
{
    let (name, arity) = indicator_exec(which);
    let ghost cs = db_view(arena.nodes@, db@);
    let ghost pred = |c: DocClause|
        lit_fa(c.head) == Some(indicator(which as int)) && c.body.len() > 0;
    proof {
        assert(crate::k2_engine::clause_valid(arena.nodes@, &db@[index as int]));
        assert(cs[index as int] == clause_view(arena.nodes@, &db@[index as int]));
        assert(cs[index as int].head == arena@[db@[index as int].head as int]);
        assert(cs[index as int].body.len() == db@[index as int].body.len());
    }
    let matched = db[index].body.len() > 0 && literal_matches(arena, db[index].head, &name, arity);
    let ghost before = out@;
    proof {
        assert(matched == pred(cs[index as int]));
        assert_seqs_equal!(cs.take(index as int + 1) == cs.take(index as int).push(cs[index as int]));
        cs.take(index as int).lemma_filter_push(cs[index as int], pred);
    }
    if matched {
        out.push(index);
        proof {
            assert forall|j: int| 0 <= j < out.len() implies out@[j] < db.len()
                && db@[out@[j] as int].body.len() > 0 by {
                if j < before.len() {
                    assert(out@[j] == before[j]);
                }
            }
            assert_seqs_equal!(indexed_clauses(arena.nodes@, db@, out@)
                == indexed_clauses(arena.nodes@, db@, before).push(cs[index as int]), j => {
                if j < before.len() { assert(out@[j] == before[j]); }
            });
        }
    }
    out
}

pub open spec fn buckets_ok(
    nodes: Seq<ENode>,
    db: Seq<EClause>,
    buckets: Seq<Vec<usize>>,
    done: nat,
) -> bool {
    buckets.len() == 9 && forall|j: int|
        #![trigger buckets[j]]
        0 <= j < 9 ==> {
            &&& rule_indices_ok(db, buckets[j]@)
            &&& indexed_clauses(nodes, db, buckets[j]@) == indicator_rules(
                db_view(nodes, db).take(done as int),
                j,
            )
        }
}

fn census_row(
    arena: &ETermArena,
    db: &Vec<EClause>,
    mut buckets: Vec<Vec<usize>>,
    index: usize,
) -> (out: Vec<Vec<usize>>)
    requires
        arena_ok(arena),
        db_valid(arena.nodes@, db@),
        index < db.len(),
        buckets_ok(arena.nodes@, db@, buckets@, index as nat),
    ensures
        buckets_ok(arena.nodes@, db@, out@, index as nat + 1),
{
    reveal(buckets_ok);
    let mut j = 0usize;
    while j < 9
        invariant
            arena_ok(arena),
            db_valid(arena.nodes@, db@),
            index < db.len(),
            j <= 9,
            buckets.len() == 9,
            forall|k: int|
                0 <= k < 9 ==> {
                    &&& rule_indices_ok(db@, buckets@[k]@)
                    &&& indexed_clauses(arena.nodes@, db@, buckets@[k]@) == indicator_rules(
                        db_view(arena.nodes@, db@).take(
                            if k < j {
                                index as int + 1
                            } else {
                                index as int
                            },
                        ),
                        k,
                    )
                },
        decreases 9 - j,
    {
        let ghost previous = buckets@;
        let bucket = buckets.remove(j);
        let updated = indicator_step(arena, db, bucket, index, j);
        buckets.insert(j, updated);
        proof {
            assert forall|k: int| 0 <= k < 9 implies {
                &&& rule_indices_ok(db@, buckets@[k]@)
                &&& indexed_clauses(arena.nodes@, db@, buckets@[k]@) == indicator_rules(
                    db_view(arena.nodes@, db@).take(
                        if k < j + 1 {
                            index as int + 1
                        } else {
                            index as int
                        },
                    ),
                    k,
                )
            } by {
                if k != j {
                    assert(buckets@[k] == previous[k]);
                } else {
                    assert(buckets@[k] == updated);
                }
            }
        }
        j += 1;
    }
    buckets
}

pub fn rule_census(arena: &ETermArena, db: &Vec<EClause>) -> (out: Vec<usize>)
    requires
        arena_ok(arena),
        db_valid(arena.nodes@, db@),
    ensures
        rule_indices_ok(db@, out@),
        indexed_clauses(arena.nodes@, db@, out@) == rules(db_view(arena.nodes@, db@)),
{
    reveal(buckets_ok);
    let mut buckets: Vec<Vec<usize>> = Vec::new();
    let mut j = 0usize;
    while j < 9
        invariant
            j <= 9,
            buckets.len() == j,
            forall|k: int| 0 <= k < j ==> buckets@[k].len() == 0,
        decreases 9 - j,
    {
        buckets.push(Vec::new());
        j += 1;
    }
    proof {
        assert_seqs_equal!(db_view(arena.nodes@, db@).take(0) == Seq::empty());
        assert forall|k: int| 0 <= k < 9 implies indexed_clauses(arena.nodes@, db@, buckets@[k]@)
            == indicator_rules(db_view(arena.nodes@, db@).take(0), k) by {
            assert_seqs_equal!(indexed_clauses(arena.nodes@, db@, buckets@[k]@) == Seq::empty());
        }
    }
    let mut c = 0usize;
    while c < db.len()
        invariant
            arena_ok(arena),
            db_valid(arena.nodes@, db@),
            c <= db.len(),
            buckets_ok(arena.nodes@, db@, buckets@, c as nat),
        decreases db.len() - c,
    {
        buckets = census_row(arena, db, buckets, c);
        c += 1;
    }
    let ghost cs = db_view(arena.nodes@, db@);
    let ghost parts = Seq::new(9, |i: int| indicator_rules(cs, i));
    let mut out = Vec::new();
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(cs.take(c as int) == cs);
        assert_seqs_equal!(indexed_clauses(arena.nodes@, db@, out@) == Seq::empty());
        assert_seqs_equal!(parts.take(0) == Seq::empty());
    }
    while i < 9
        invariant
            arena_ok(arena),
            db_valid(arena.nodes@, db@),
            i <= 9,
            buckets_ok(arena.nodes@, db@, buckets@, db.len() as nat),
            cs == db_view(arena.nodes@, db@),
            parts == Seq::new(9, |j: int| indicator_rules(cs, j)),
            rule_indices_ok(db@, out@),
            indexed_clauses(arena.nodes@, db@, out@) == parts.take(i as int).flatten(),
        decreases 9 - i,
    {
        proof {
            assert(rule_indices_ok(db@, buckets@[i as int]@));
            assert_seqs_equal!(db_view(arena.nodes@, db@).take(db.len() as int) == cs);
            assert(indexed_clauses(arena.nodes@, db@, buckets@[i as int]@) == parts[i as int]);
        }
        let mut next = buckets[i].clone();
        let ghost left = out@;
        let ghost right = next@;
        out.append(&mut next);
        proof {
            assert forall|j: int| 0 <= j < out.len() implies out@[j] < db.len()
                && db@[out@[j] as int].body.len() > 0 by {
                if j < left.len() {
                    assert(out@[j] == left[j]);
                } else {
                    assert(out@[j] == right[j - left.len()]);
                }
            }
            assert_seqs_equal!(indexed_clauses(arena.nodes@, db@, out@) == indexed_clauses(arena.nodes@, db@, left)
                + indexed_clauses(arena.nodes@, db@, right), j => {
                if j < left.len() { assert(out@[j] == left[j]); }
                else { assert(out@[j] == right[j - left.len()]); }
            });
            assert_seqs_equal!(parts.take(i as int + 1) == parts.take(i as int).push(parts[i as int]));
            parts.take(i as int).lemma_flatten_push(parts[i as int]);
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(parts.take(i as int) == parts);
    }
    out
}

fn leftmost_root(arena: &ETermArena, clause: &EClause) -> (out: usize)
    requires
        arena_ok(arena),
        crate::k2_engine::clause_valid(arena.nodes@, clause),
        clause.body.len() > 0,
    ensures
        crate::k2_term::root_ok(arena, out),
        arena@[out as int] == ckc_spec::replay::leftmost(clause_view(arena.nodes@, clause)),
{
    proof {
        assert(crate::k2_engine::body_item_valid(arena.nodes@, &clause.body@[0]));
        assert(clause_view(arena.nodes@, clause).body[0] == crate::k2_engine::body_item_view(
            arena.nodes@,
            &clause.body@[0],
        ));
    }
    match &clause.body[0] {
        EBodyItem::Pos { root } => *root,
        EBodyItem::Naf { roots } => roots[0],
    }
}

proof fn indexed_prefix(
    before: Seq<ENode>,
    after: Seq<ENode>,
    db: Seq<EClause>,
    indices: Seq<usize>,
)
    requires
        before.is_prefix_of(after),
        db_valid(before, db),
        rule_indices_ok(db, indices),
    ensures
        db_valid(after, db),
        db_view(before, db) == db_view(after, db),
        indexed_clauses(before, db, indices) == indexed_clauses(after, db, indices),
{
    crate::k2_engine::db_models_prefix(before, after, db);
    assert_seqs_equal!(indexed_clauses(before, db, indices) == indexed_clauses(after, db, indices), i => {
        assert(db_view(before, db)[indices[i] as int] == db_view(after, db)[indices[i] as int]);
    });
}

pub open spec fn recursive_clause_view(
    nodes: Seq<ENode>,
    db: Seq<EClause>,
    index: Option<usize>,
) -> Option<DocClause> {
    match index {
        Some(i) => Some(clause_view(nodes, &db[i as int])),
        None => None,
    }
}

fn first_recursive_inner(input_arena: ETermArena, db: &Vec<EClause>, indices: &Vec<usize>) -> (out:
    (Option<usize>, ETermArena))
    requires
        arena_ok(&input_arena),
        db_valid(input_arena.nodes@, db@),
        rule_indices_ok(db@, indices@),
    ensures
        arena_ok(&out.1),
        input_arena.nodes@.is_prefix_of(out.1.nodes@),
        db_valid(out.1.nodes@, db@),
        db_view(out.1.nodes@, db@) == db_view(input_arena.nodes@, db@),
        out.0 matches Some(i) ==> i < db.len() && db@[i as int].body@.len() > 0,
        recursive_clause_view(out.1.nodes@, db@, out.0) == ckc_spec::replay::first_left_recursive(
            indexed_clauses(input_arena.nodes@, db@, indices@),
        ),
{
    hide(ckc_spec::replay::first_left_recursive);
    let ghost origin = input_arena.nodes@;
    let ghost cs = indexed_clauses(origin, db@, indices@);
    let ghost original_db = db_view(origin, db@);
    let mut arena = input_arena;
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(cs.skip(0) == cs);
    }
    while i < indices.len()
        invariant
            arena_ok(&arena),
            origin == input_arena.nodes@,
            origin.is_prefix_of(arena.nodes@),
            db_valid(arena.nodes@, db@),
            db_view(arena.nodes@, db@) == original_db,
            original_db == db_view(origin, db@),
            cs == indexed_clauses(origin, db@, indices@),
            cs == indexed_clauses(arena.nodes@, db@, indices@),
            rule_indices_ok(db@, indices@),
            i <= indices.len(),
            ckc_spec::replay::first_left_recursive(cs) == ckc_spec::replay::first_left_recursive(
                cs.skip(i as int),
            ),
        decreases indices.len() - i,
    {
        let index = indices[i];
        proof {
            assert(crate::k2_engine::clause_valid(arena.nodes@, &db@[index as int]));
            assert(cs[i as int] == clause_view(arena.nodes@, &db@[index as int]));
        }
        let goal = leftmost_root(&arena, &db[index]);
        let head = db[index].head;
        let ghost before = arena.nodes@;
        let found = unifiable_apart(&mut arena, goal, head);
        proof {
            indexed_prefix(before, arena.nodes@, db@, indices@);
            crate::k2_load::prefix_chain(origin, before, arena.nodes@);
            assert(found == ckc_spec::engine::unifiable_apart(
                ckc_spec::replay::leftmost(cs[i as int]),
                cs[i as int].head,
            ));
            assert_seqs_equal!(cs.skip(i as int).drop_first() == cs.skip(i as int + 1));
            reveal_with_fuel(ckc_spec::replay::first_left_recursive, 1);
        }
        if found {
            proof {
                assert(clause_view(arena.nodes@, &db@[index as int]) == cs[i as int]);
            }
            return (Some(index), arena);
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(cs.skip(i as int) == Seq::empty());
        reveal_with_fuel(ckc_spec::replay::first_left_recursive, 1);
    }
    (None, arena)
}

pub fn first_recursive(arena: &mut ETermArena, db: &Vec<EClause>, indices: &Vec<usize>) -> (out:
    Option<usize>)
    requires
        arena_ok(old(arena)),
        db_valid(old(arena).nodes@, db@),
        rule_indices_ok(db@, indices@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        db_valid(final(arena).nodes@, db@),
        db_view(final(arena).nodes@, db@) == db_view(old(arena).nodes@, db@),
        out matches Some(i) ==> i < db.len() && db@[i as int].body@.len() > 0,
        recursive_clause_view(final(arena).nodes@, db@, out)
            == ckc_spec::replay::first_left_recursive(
            indexed_clauses(old(arena).nodes@, db@, indices@),
        ),
{
    let mut owned = crate::k2_reject::empty_arena();
    std::mem::swap(arena, &mut owned);
    let (out, mut owned) = first_recursive_inner(owned, db, indices);
    std::mem::swap(arena, &mut owned);
    out
}

fn site_root(arena: &mut ETermArena, clause: &EClause) -> (out: usize)
    requires
        arena_ok(old(arena)),
        crate::k2_engine::clause_valid(old(arena).nodes@, clause),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        crate::k2_term::root_ok(final(arena), out),
        final(arena)@[out as int] == ckc_spec::replay::site(clause_view(old(arena).nodes@, clause)),
{
    let ghost origin = arena.nodes@;
    let roots = crate::k2_walk::clause_walk_roots(arena, clause);
    let pairs = crate::k2_walk::gid_pairs_all_exec(arena, &roots);
    proof {
        assert(crate::k2_engine::root_terms(arena.nodes@, pairs@) == ckc_spec::replay::clause_gids(
            clause_view(origin, clause),
        ));
    }
    if pairs.len() == 0 {
        let name: &[u8] = b"unattributed";
        proof {
            reveal_byteslit(b"unattributed");
            reveal_strlit("unattributed");
            reveal(ckc_spec::v1text::ascii);
            assert(name@ == ckc_spec::v1text::ascii("unattributed"@));
        }
        crate::k2_output::atom_root(arena, name)
    } else {
        let root = pairs[0];
        proof {
            assert(crate::k2_walk::gid_pair_shape(arena@[root as int]));
            assert(ckc_spec::engine::args_of(arena@[root as int]).len() == 2);
        }
        let args = crate::k2_engine::args_roots(arena, root);
        let name: &[u8] = b"sentence";
        proof {
            reveal_byteslit(b"sentence");
            reveal_strlit("sentence");
            reveal(ckc_spec::v1text::ascii);
            assert(name@ == ckc_spec::v1text::ascii("sentence"@));
            assert_seqs_equal!(crate::k2_engine::root_terms(arena.nodes@, args@)
                == crate::k2_term::child_terms(arena.nodes@, args@));
        }
        crate::k2_output::comp_root(arena, name, args)
    }
}

pub fn left_recursive_error(arena: &mut ETermArena, clause: &EClause) -> (out: Option<
    ckc_spec::replay::EOut,
>)
    requires
        arena_ok(old(arena)),
        crate::k2_engine::clause_valid(old(arena).nodes@, clause),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        crate::k2_output::option_out_view(out) == match clause_view(
            old(arena).nodes@,
            clause,
        ).head {
            ckc_spec::term::Term::Comp(name, args) => Some(
                ckc_spec::replay::proof_fail(
                    ckc_spec::term::Term::Comp(
                        ckc_spec::v1text::ascii("left_recursive"@),
                        seq![
                            ckc_spec::replay::site(clause_view(old(arena).nodes@, clause)),
                            ckc_spec::term::Term::Atom(name),
                            ckc_spec::term::Term::Int(args.len() as int),
                        ],
                    ),
                ),
            ),
            _ => None,
        },
{
    let ghost origin = arena.nodes@;
    let head = clause.head;
    proof {
        reveal(arena_ok);
        assert(crate::k2_term::node_ok(arena.nodes@, head as int));
        reveal(crate::k2_term::node_ok);
    }
    let (name, arity) = match &arena.nodes[head].kind {
        crate::k2_term::ENodeKind::Comp { name, child_roots, .. } => (
            name.clone(),
            child_roots.len(),
        ),
        _ => return None,
    };
    let site = site_root(arena, clause);
    let functor = crate::k2_output::atom_root(arena, &name);
    let count = crate::k2_output::int_root(arena, arity);
    let label: &[u8] = b"left_recursive";
    let detail = crate::k2_output::comp3(arena, label, site, functor, count);
    proof {
        reveal_byteslit(b"left_recursive");
        reveal_strlit("left_recursive");
        reveal(ckc_spec::v1text::ascii);
        assert(label@ == ckc_spec::v1text::ascii("left_recursive"@));
    }
    Some(crate::k2_output::error_out(arena, detail, true))
}

} // verus!
