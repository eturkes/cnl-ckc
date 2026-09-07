#[cfg(verus_keep_ghost)]
use crate::k2_engine::{root_terms, roots_valid, roots_work, term_size, terms_size};
use crate::k2_output::{atom_root, comp1, comp2, int_root};
use crate::k2_term::{ENode, ENodeKind, ETermArena};
#[cfg(verus_keep_ghost)]
use crate::k2_term::{arena_ok, node_ok, root_ok};
#[cfg(verus_keep_ghost)]
use ckc_spec::answers::{goal_walk, query_reject};
use ckc_spec::replay::{EOut, ESrc};
#[cfg(verus_keep_ghost)]
use ckc_spec::term::Term;
use ckc_spec::v1text::QueryFile;
#[cfg(verus_keep_ghost)]
use vstd::assert_seqs_equal;
use vstd::prelude::*;
use vstd::slice::slice_to_vec;

verus! {

fn semantic_name(name: &Vec<u8>, arity: usize) -> (out: bool)
    ensures
        out == ckc_spec::v1text::is_semantic_pred(name@, arity as nat),
{
    let mut i = 2usize;
    while i < 9
        invariant
            2 <= i <= 9,
            forall|j: int|
                2 <= j < i ==> #[trigger] ckc_spec::v1text::indicator(j) != (name@, arity as nat),
        decreases 9 - i,
    {
        let (candidate, count) = crate::k2_recursion::indicator_exec(i);
        if count == arity && crate::k2_engine::vec_equal(&candidate, name) {
            proof {
                assert(ckc_spec::v1text::indicator(i as int) == (name@, arity as nat));
            }
            return true;
        }
        i += 1;
    }
    false
}

spec fn walk_goals(goals: Seq<Term>) -> Option<Term>
    decreases goals.len(),
{
    if goals.len() == 0 {
        None
    } else {
        match goal_walk(goals[0]) {
            Some(why) => Some(why),
            None => walk_goals(goals.drop_first()),
        }
    }
}

proof fn walk_goals_concat(left: Seq<Term>, right: Seq<Term>)
    ensures
        walk_goals(left + right) == match walk_goals(left) {
            Some(why) => Some(why),
            None => walk_goals(right),
        },
    decreases left.len(),
{
    reveal_with_fuel(walk_goals, 1);
    if left.len() > 0 {
        assert_seqs_equal!((left + right).drop_first() == left.drop_first() + right);
        walk_goals_concat(left.drop_first(), right);
    } else {
        assert_seqs_equal!(left + right == right);
    }
}

spec fn comma_goal(t: Term) -> bool {
    match t {
        Term::Comp(name, args) => name == ckc_spec::engine::comma_name() && args.len() == 2,
        _ => false,
    }
}

fn goal_offender(arena: &ETermArena, goal: usize) -> (out: Option<usize>)
    requires
        root_ok(arena, goal),
    ensures
        out.is_none() == goal_walk(arena@[goal as int]).is_none(),
        out matches Some(root) ==> root < arena.nodes.len() && !comma_goal(arena@[root as int])
            && goal_walk(arena@[root as int]).is_some() && goal_walk(arena@[root as int])
            == goal_walk(arena@[goal as int]),
{
    hide(goal_walk);
    hide(walk_goals);
    let mut pending = Vec::new();
    pending.push(goal);
    let comma = slice_to_vec(b",");
    proof {
        reveal_byteslit(b",");
        reveal_strlit(",");
        reveal(ckc_spec::v1text::ascii);
        assert(comma@ == ckc_spec::engine::comma_name());
        assert_seqs_equal!(root_terms(arena.nodes@, pending@) == seq![arena@[goal as int]]);
        reveal_with_fuel(walk_goals, 2);
    }
    while pending.len() > 0
        invariant
            root_ok(arena, goal),
            roots_valid(arena.nodes@, pending@),
            goal_walk(arena@[goal as int]) == walk_goals(root_terms(arena.nodes@, pending@)),
            comma@ == ckc_spec::engine::comma_name(),
        decreases roots_work(arena.nodes@, pending@),
    {
        let ghost before = pending@;
        let root = pending.remove(0);
        proof {
            assert_seqs_equal!(pending@ == before.drop_first());
            assert_seqs_equal!(root_terms(arena.nodes@, before).drop_first() == root_terms(arena.nodes@, pending@));
            assert(node_ok(arena.nodes@, root as int));
            reveal_with_fuel(walk_goals, 1);
            reveal_with_fuel(goal_walk, 1);
            reveal_with_fuel(roots_work, 1);
            reveal_with_fuel(term_size, 1);
        }
        match &arena.nodes[root].kind {
            ENodeKind::Comp { name, child_roots, .. } => {
                let args = crate::k2_engine::args_roots(arena, root);
                if args.len() == 2 && crate::k2_engine::vec_equal(name, &comma) {
                    let ghost children = args@;
                    let ghost rest = pending@;
                    let mut next = args;
                    next.append(&mut pending);
                    pending = next;
                    proof {
                        assert forall|j: int| 0 <= j < pending.len() implies pending@[j]
                            < arena.nodes.len() by {
                            if j < children.len() {
                                assert(pending@[j] == children[j]);
                            } else {
                                assert(pending@[j] == rest[j - children.len()]);
                            }
                        }
                        assert_seqs_equal!(root_terms(arena.nodes@, pending@)
                            == root_terms(arena.nodes@, children) + root_terms(arena.nodes@, rest));
                        let models = root_terms(arena.nodes@, children);
                        assert_seqs_equal!(models == seq![models[0], models[1]]);
                        assert_seqs_equal!(models.drop_first() == seq![models[1]]);
                        assert_seqs_equal!(seq![models[1]].drop_first() == Seq::empty());
                        reveal_with_fuel(walk_goals, 3);
                        walk_goals_concat(models, root_terms(arena.nodes@, rest));
                        crate::k2_engine::roots_work_concat(arena.nodes@, children, rest);
                        crate::k2_engine::terms_size_root_terms(arena.nodes@, children);
                    }
                } else if !semantic_name(name, args.len()) {
                    return Some(root);
                }
            },
            _ => return Some(root),
        }
    }
    proof {
        assert_seqs_equal!(root_terms(arena.nodes@, pending@) == Seq::empty());
        reveal_with_fuel(walk_goals, 1);
    }
    None
}

pub fn query_error(arena: &mut ETermArena, why: usize) -> (out: EOut)
    requires
        root_ok(old(arena), why),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out@ == query_reject(old(arena)@[why as int]),
{
    let name: &[u8] = b"query_file";
    proof {
        reveal_byteslit(b"query_file");
        reveal_strlit("query_file");
        reveal(ckc_spec::v1text::ascii);
        assert(name@ == ckc_spec::v1text::ascii("query_file"@));
    }
    let detail = comp1(arena, name, why);
    crate::k2_output::error_out(arena, detail, false)
}

pub fn query_atom_error(why: &[u8]) -> (out: EOut)
    ensures
        out@ == query_reject(Term::Atom(why@)),
{
    let mut arena = crate::k2_reject::empty_arena();
    let why = atom_root(&mut arena, why);
    query_error(&mut arena, why)
}

fn goal_leaf_error(arena: &mut ETermArena, root: usize) -> (out: EOut)
    requires
        root_ok(old(arena), root),
        !comma_goal(old(arena)@[root as int]),
        goal_walk(old(arena)@[root as int]).is_some(),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        Some(out@) == (match goal_walk(old(arena)@[root as int]) {
            Some(why) => Some(query_reject(why)),
            None => None,
        }),
{
    proof {
        assert(node_ok(arena.nodes@, root as int));
    }
    let variable: &[u8] = b"goal_variable";
    let foreign: &[u8] = b"goal_foreign";
    proof {
        reveal_byteslit(b"goal_variable");
        reveal_strlit("goal_variable");
        reveal_byteslit(b"goal_foreign");
        reveal_strlit("goal_foreign");
        reveal(ckc_spec::v1text::ascii);
        assert(variable@ == ckc_spec::v1text::ascii("goal_variable"@));
        assert(foreign@ == ckc_spec::v1text::ascii("goal_foreign"@));
    }
    let (name, count) = match &arena.nodes[root].kind {
        ENodeKind::Var { .. } => {
            let why = atom_root(arena, variable);
            return query_error(arena, why);
        },
        ENodeKind::Comp { name, child_roots, .. } => {
            let bytes = name.clone();
            let count = child_roots.len();
            (atom_root(arena, bytes.as_slice()), count)
        },
        _ => (root, 0usize),
    };
    let count = int_root(arena, count);
    let why = comp2(arena, foreign, name, count);
    query_error(arena, why)
}

pub fn goal_check_exec(arena: &mut ETermArena, goal: usize) -> (out: Option<EOut>)
    requires
        root_ok(old(arena), goal),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        crate::k2_output::option_out_view(out) == (match goal_walk(old(arena)@[goal as int]) {
            Some(why) => Some(query_reject(why)),
            None => None,
        }),
{
    match goal_offender(arena, goal) {
        Some(root) => Some(goal_leaf_error(arena, root)),
        None => None,
    }
}

pub fn variable_key(arena: &ETermArena, root: usize) -> (out: Option<usize>)
    requires
        root_ok(arena, root),
    ensures
        out.is_some() == (arena@[root as int] is Var),
        out matches Some(key) ==> arena@[root as int] == Term::Var(key as nat),
{
    proof {
        assert(node_ok(arena.nodes@, root as int));
    }
    match &arena.nodes[root].kind {
        ENodeKind::Var { key, .. } => Some(*key),
        _ => None,
    }
}

fn atom_is(arena: &ETermArena, root: usize, bytes: &Vec<u8>) -> (out: bool)
    requires
        root_ok(arena, root),
    ensures
        out == (arena@[root as int] == Term::Atom(bytes@)),
{
    proof {
        assert(node_ok(arena.nodes@, root as int));
    }
    match &arena.nodes[root].kind {
        ENodeKind::Atom { name } => crate::k2_engine::vec_equal(name, bytes),
        _ => false,
    }
}

fn is_atom(arena: &ETermArena, root: usize) -> (out: bool)
    requires
        root_ok(arena, root),
    ensures
        out == (arena@[root as int] is Atom),
{
    proof {
        assert(node_ok(arena.nodes@, root as int));
    }
    matches!(&arena.nodes[root].kind, ENodeKind::Atom { .. })
}

fn descriptor_ok(arena: &ETermArena, root: usize) -> (out: bool)
    requires
        root_ok(arena, root),
    ensures
        out == ckc_spec::answers::desc_ok(arena@[root as int]),
{
    let noun = slice_to_vec(b"noun");
    let wh = slice_to_vec(b"wh");
    let who = slice_to_vec(b"who");
    let what = slice_to_vec(b"what");
    proof {
        reveal_byteslit(b"noun");
        reveal_strlit("noun");
        reveal_byteslit(b"wh");
        reveal_strlit("wh");
        reveal_byteslit(b"who");
        reveal_strlit("who");
        reveal_byteslit(b"what");
        reveal_strlit("what");
        reveal(ckc_spec::v1text::ascii);
        assert(noun@ == ckc_spec::v1text::ascii("noun"@));
        assert(wh@ == ckc_spec::v1text::ascii("wh"@));
        assert(who@ == ckc_spec::v1text::ascii("who"@));
        assert(what@ == ckc_spec::v1text::ascii("what"@));
    }
    let args = crate::k2_engine::args_roots(arena, root);
    if crate::k2_engine::literal_matches(arena, root, &noun, 2) {
        is_atom(arena, args[0]) && is_atom(arena, args[1])
    } else if crate::k2_engine::literal_matches(arena, root, &wh, 1) {
        atom_is(arena, args[0], &who) || atom_is(arena, args[0], &what)
    } else {
        false
    }
}

fn key_seen(seen: &Vec<usize>, key: usize) -> (out: bool)
    ensures
        out == seen@.contains(key),
{
    let mut i = 0usize;
    while i < seen.len()
        invariant
            i <= seen.len(),
            forall|j: int| 0 <= j < i ==> seen@[j] != key,
        decreases seen.len() - i,
    {
        if seen[i] == key {
            proof {
                assert(seen@[i as int] == key);
            }
            return true;
        }
        i += 1;
    }
    false
}

fn answer_var_error(i: usize, why: &[u8]) -> (out: EOut)
    ensures
        out@ == query_reject(
            Term::Comp(
                ckc_spec::v1text::ascii("answer_var"@),
                seq![Term::Int(i as int), Term::Atom(why@)],
            ),
        ),
{
    let mut arena = crate::k2_reject::empty_arena();
    let index = int_root(&mut arena, i);
    let reason = atom_root(&mut arena, why);
    let name: &[u8] = b"answer_var";
    proof {
        reveal_byteslit(b"answer_var");
        reveal_strlit("answer_var");
        reveal(ckc_spec::v1text::ascii);
        assert(name@ == ckc_spec::v1text::ascii("answer_var"@));
    }
    let detail = comp2(&mut arena, name, index, reason);
    query_error(&mut arena, detail)
}

fn answer_row_error(i: usize, name: &[u8]) -> (out: EOut)
    ensures
        out@ == query_reject(Term::Comp(name@, seq![Term::Int(i as int)])),
{
    let mut arena = crate::k2_reject::empty_arena();
    let index = int_root(&mut arena, i);
    let detail = comp1(&mut arena, name, index);
    query_error(&mut arena, detail)
}

pub fn rows_check_exec(arena: &ETermArena, rows: &Vec<usize>, goal: usize) -> (out: Option<EOut>)
    requires
        root_ok(arena, goal),
        roots_valid(arena.nodes@, rows@),
    ensures
        crate::k2_output::option_out_view(out) == match ckc_spec::answers::rows_check(
            root_terms(arena.nodes@, rows@),
            1,
            arena@[goal as int],
            Seq::empty(),
        ) {
            Some(why) => Some(query_reject(why)),
            None => None,
        },
{
    hide(ckc_spec::answers::rows_check);
    let ghost models = root_terms(arena.nodes@, rows@);
    let answer = slice_to_vec(b"answer");
    let shape: &[u8] = b"answer_shape";
    let nonvar: &[u8] = b"nonvar";
    let absent: &[u8] = b"absent";
    let duplicate: &[u8] = b"duplicate";
    let desc: &[u8] = b"answer_desc";
    let mut seen = Vec::new();
    let mut i = 0usize;
    proof {
        reveal_byteslit(b"answer");
        reveal_strlit("answer");
        reveal_byteslit(b"answer_shape");
        reveal_strlit("answer_shape");
        reveal_byteslit(b"nonvar");
        reveal_strlit("nonvar");
        reveal_byteslit(b"absent");
        reveal_strlit("absent");
        reveal_byteslit(b"duplicate");
        reveal_strlit("duplicate");
        reveal_byteslit(b"answer_desc");
        reveal_strlit("answer_desc");
        reveal(ckc_spec::v1text::ascii);
        assert(answer@ == ckc_spec::v1text::ascii("answer"@));
        assert(shape@ == ckc_spec::v1text::ascii("answer_shape"@));
        assert(nonvar@ == ckc_spec::v1text::ascii("nonvar"@));
        assert(absent@ == ckc_spec::v1text::ascii("absent"@));
        assert(duplicate@ == ckc_spec::v1text::ascii("duplicate"@));
        assert(desc@ == ckc_spec::v1text::ascii("answer_desc"@));
        assert_seqs_equal!(models.skip(0) == models);
        assert_seqs_equal!(seen@.map_values(|x: usize| x as nat) == Seq::empty());
    }
    while i < rows.len()
        invariant
            root_ok(arena, goal),
            roots_valid(arena.nodes@, rows@),
            i <= rows.len(),
            models == root_terms(arena.nodes@, rows@),
            ckc_spec::answers::rows_check(models, 1, arena@[goal as int], Seq::empty())
                == ckc_spec::answers::rows_check(
                models.skip(i as int),
                i as nat + 1,
                arena@[goal as int],
                seen@.map_values(|x: usize| x as nat),
            ),
            answer@ == ckc_spec::v1text::ascii("answer"@),
            shape@ == ckc_spec::v1text::ascii("answer_shape"@),
            nonvar@ == ckc_spec::v1text::ascii("nonvar"@),
            absent@ == ckc_spec::v1text::ascii("absent"@),
            duplicate@ == ckc_spec::v1text::ascii("duplicate"@),
            desc@ == ckc_spec::v1text::ascii("answer_desc"@),
        decreases rows.len() - i,
    {
        let root = rows[i];
        proof {
            reveal_with_fuel(ckc_spec::answers::rows_check, 1);
            assert_seqs_equal!(models.skip(i as int).drop_first() == models.skip(i as int + 1));
        }
        if !crate::k2_engine::literal_matches(arena, root, &answer, 2) {
            return Some(answer_row_error(i + 1, shape));
        }
        let args = crate::k2_engine::args_roots(arena, root);
        let key = match variable_key(arena, args[0]) {
            None => return Some(answer_var_error(i + 1, nonvar)),
            Some(key) => key,
        };
        if !crate::k2_engine::occurs_root(arena, key, goal) {
            return Some(answer_var_error(i + 1, absent));
        }
        proof {
            let nats = seen@.map_values(|x: usize| x as nat);
            assert(seen@.contains(key) == nats.contains(key as nat)) by {
                if seen@.contains(key) {
                    let j = choose|j: int| 0 <= j < seen.len() && seen@[j] == key;
                    assert(nats[j] == key as nat);
                } else if nats.contains(key as nat) {
                    let j = choose|j: int| 0 <= j < nats.len() && #[trigger] nats[j] == key as nat;
                    assert(seen@[j] == key);
                }
            }
        }
        if key_seen(&seen, key) {
            return Some(answer_var_error(i + 1, duplicate));
        }
        if !descriptor_ok(arena, args[1]) {
            return Some(answer_row_error(i + 1, desc));
        }
        let ghost before = seen@;
        seen.push(key);
        proof {
            assert_seqs_equal!(seen@.map_values(|x: usize| x as nat) == before.map_values(|x: usize| x as nat).push(key as nat));
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(models.skip(i as int) == Seq::empty());
        reveal_with_fuel(ckc_spec::answers::rows_check, 1);
    }
    None
}

proof fn clauses_byte_bound(clauses: Seq<ckc_spec::v1text::DocClause>)
    ensures
        clauses.len() <= ckc_spec::v1text::clauses_bytes(clauses).len(),
    decreases clauses.len(),
{
    reveal_with_fuel(ckc_spec::v1text::clauses_bytes, 1);
    if clauses.len() > 0 {
        clauses_byte_bound(clauses.drop_first());
        reveal_strlit(".\n");
        reveal(ckc_spec::v1text::ascii);
        assert(ckc_spec::v1text::clause_line(clauses[0]).len() >= 2);
    }
}

proof fn bundles_census(bundles: Seq<ckc_spec::v1text::Bundle>)
    ensures
        ckc_spec::answers::clause_count(bundles) == bundles.map_values(
            |b: ckc_spec::v1text::Bundle| b.clauses,
        ).flatten().len(),
        ckc_spec::answers::clause_count(bundles) <= ckc_spec::v1text::bundles_bytes(bundles).len(),
    decreases bundles.len(),
{
    reveal_with_fuel(ckc_spec::answers::clause_count, 1);
    reveal_with_fuel(ckc_spec::v1text::bundles_bytes, 1);
    reveal_with_fuel(Seq::<_>::flatten, 1);
    if bundles.len() > 0 {
        bundles_census(bundles.drop_first());
        clauses_byte_bound(bundles[0].clauses);
        assert_seqs_equal!(bundles.map_values(|b: ckc_spec::v1text::Bundle| b.clauses).drop_first()
            == bundles.drop_first().map_values(|b: ckc_spec::v1text::Bundle| b.clauses));
    }
}

fn parsed_term_count(
    arena: &ETermArena,
    bytes: &[u8],
    parsed: &crate::v1_term_impl::EParsedV1,
) -> (out: usize)
    requires
        crate::v1_term_impl::parsed_v1_ok(bytes@, parsed),
        crate::v1_term_impl::parsed_metadata_ok(parsed),
        crate::v1_term_impl::parsed_doc_roots_ok(arena.nodes@, parsed),
    ensures
        out as nat == ckc_spec::answers::term_count(parsed@),
{
    match parsed.class {
        crate::v1_term_impl::EV1Class::Doc => {
            let ghost doc = choose|d: ckc_spec::v1text::DocFile|
                parsed@ == ckc_spec::v1text::V1File::Doc(d);
            proof {
                assert(parsed@ is Doc);
                crate::v1_term_impl::parsed_doc_roots_elim(arena.nodes@, parsed);
                crate::v1_term_impl::doc_clause_models_flatten(doc.bundles);
                crate::v1_term_impl::doc_clauses_roots_elim(
                    arena.nodes@,
                    parsed.clauses@,
                    crate::v1_term_impl::doc_clause_models(doc.bundles),
                );
                bundles_census(doc.bundles);
                reveal_strlit(
                    ".pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.\n",
                );
                reveal(ckc_spec::v1text::ascii);
                assert(ckc_spec::v1text::doc_line1(doc.docid).len() >= 20);
                assert(parsed.clauses.len() + 20 <= bytes.len());
            }
            parsed.clauses.len() + 20
        },
        crate::v1_term_impl::EV1Class::Query => 2,
        _ => 1,
    }
}

pub struct EQuery {
    pub qid: Vec<u8>,
    pub goal: usize,
    pub rows: Vec<usize>,
    pub file: Ghost<QueryFile>,
}

pub open spec fn query_ok(nodes: Seq<ENode>, query: &EQuery) -> bool {
    &&& query.qid@ == query.file@.qid
    &&& query.goal < nodes.len()
    &&& nodes[query.goal as int].term@ == query.file@.goal
    &&& roots_valid(nodes, query.rows@)
    &&& ckc_spec::answers::list_items(query.file@.answers) == Some(root_terms(nodes, query.rows@))
}

pub proof fn query_prefix(before: Seq<ENode>, after: Seq<ENode>, query: &EQuery)
    requires
        before.is_prefix_of(after),
        query_ok(before, query),
    ensures
        query_ok(after, query),
        root_terms(after, query.rows@) == root_terms(before, query.rows@),
{
    crate::k2_engine::roots_models_prefix(before, after, query.rows@);
    assert(after[query.goal as int] == before[query.goal as int]);
}

pub open spec fn custody_view(nodes: Seq<ENode>, result: Result<EQuery, EOut>) -> Result<
    (QueryFile, Seq<Term>),
    ckc_spec::replay::Out,
> {
    match result {
        Ok(query) => Ok((query.file@, root_terms(nodes, query.rows@))),
        Err(out) => Err(out@),
    }
}

pub fn custody_exec(arena: &mut ETermArena, source: &ESrc) -> (out: Result<EQuery, EOut>)
    requires
        arena_ok(old(arena)),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Ok(query) ==> query_ok(final(arena).nodes@, &query),
        custody_view(final(arena).nodes@, out) == ckc_spec::answers::custody(source@),
{
    hide(ckc_spec::answers::custody);
    hide(ckc_spec::answers::rows_check);
    hide(goal_walk);
    let bytes = match source {
        ESrc::Missing => {
            proof {
                reveal(ckc_spec::answers::custody);
            }
            return Err(crate::k2_reject::unreadable_out());
        },
        ESrc::Bad(off) => {
            proof {
                reveal(ckc_spec::answers::custody);
            }
            return Err(crate::k2_reject::utf8_out(*off));
        },
        ESrc::Bytes(bytes) => bytes,
    };
    let parsed = match crate::v1_impl::v1_parse(bytes.as_slice(), arena) {
        None => {
            let name: &[u8] = b"noncanonical";
            proof {
                reveal(ckc_spec::answers::custody);
                reveal_byteslit(b"noncanonical");
                reveal_strlit("noncanonical");
                reveal(ckc_spec::v1text::ascii);
                assert(name@ == ckc_spec::v1text::ascii("noncanonical"@));
            }
            return Err(query_atom_error(name));
        },
        Some(parsed) => parsed,
    };
    if !matches!(parsed.class, crate::v1_term_impl::EV1Class::Query) {
        let count = parsed_term_count(arena, bytes.as_slice(), &parsed);
        let count_root = int_root(arena, count);
        let name: &[u8] = b"term_count";
        proof {
            reveal(ckc_spec::answers::custody);
            reveal_byteslit(b"term_count");
            reveal_strlit("term_count");
            reveal(ckc_spec::v1text::ascii);
            assert(name@ == ckc_spec::v1text::ascii("term_count"@));
            assert(!(parsed@ is Query));
        }
        let detail = comp1(arena, name, count_root);
        return Err(query_error(arena, detail));
    }
    let ghost query = choose|q: QueryFile| parsed@ == ckc_spec::v1text::V1File::Query(q);
    proof {
        assert(parsed@ is Query);
        crate::v1_term_impl::parsed_query_roots_elim(arena.nodes@, &parsed);
    }
    let goal = parsed.goal_root;
    let answers = parsed.answers_root;
    let ghost before_goal = arena.nodes@;
    let failed = goal_check_exec(arena, goal);
    proof {
        crate::k2_term::arena_prefix_stable(before_goal, arena);
        reveal(ckc_spec::answers::custody);
    }
    if let Some(out) = failed {
        return Err(out);
    }
    let rows = match crate::k2_walk::list_items_exec(arena, answers) {
        None => {
            let name: &[u8] = b"answers_list";
            proof {
                reveal_byteslit(b"answers_list");
                reveal_strlit("answers_list");
                reveal(ckc_spec::v1text::ascii);
                assert(name@ == ckc_spec::v1text::ascii("answers_list"@));
            }
            return Err(query_atom_error(name));
        },
        Some(rows) => rows,
    };
    if let Some(out) = rows_check_exec(arena, &rows, goal) {
        return Err(out);
    }
    Ok(EQuery { qid: parsed.qid, goal, rows, file: Ghost(query) })
}

fn project_roots_inner(mut arena: ETermArena, rows: &Vec<usize>) -> (out: (Vec<usize>, ETermArena))
    requires
        arena_ok(&arena),
        roots_valid(arena.nodes@, rows@),
    ensures
        arena_ok(&out.1),
        arena.nodes@.is_prefix_of(out.1.nodes@),
        roots_valid(out.1.nodes@, out.0@),
        root_terms(out.1.nodes@, out.0@) == root_terms(arena.nodes@, rows@).map_values(
            |r: Term| ckc_spec::replay::arg(r, 0),
        ),
{
    let ghost base = arena.nodes@;
    let ghost models = root_terms(base, rows@);
    let mut result = Vec::new();
    let mut i = 0usize;
    while i < rows.len()
        invariant
            arena_ok(&arena),
            base.is_prefix_of(arena.nodes@),
            roots_valid(base, rows@),
            roots_valid(arena.nodes@, rows@),
            models == root_terms(base, rows@),
            models == root_terms(arena.nodes@, rows@),
            i <= rows.len(),
            result.len() == i,
            roots_valid(arena.nodes@, result@),
            root_terms(arena.nodes@, result@) == models.take(i as int).map_values(
                |r: Term| ckc_spec::replay::arg(r, 0),
            ),
        decreases rows.len() - i,
    {
        let ghost before = arena.nodes@;
        let ghost previous = result@;
        let root = crate::k2_walk::arg_root(&mut arena, rows[i], 0);
        proof {
            crate::k2_engine::roots_models_prefix(before, arena.nodes@, rows@);
            crate::k2_engine::roots_models_prefix(before, arena.nodes@, previous);
        }
        result.push(root);
        proof {
            assert_seqs_equal!(root_terms(arena.nodes@, result@)
                == root_terms(arena.nodes@, previous).push(arena@[root as int]));
            assert_seqs_equal!(models.take(i as int + 1).map_values(|r: Term| ckc_spec::replay::arg(r, 0))
                == models.take(i as int).map_values(|r: Term| ckc_spec::replay::arg(r, 0)).push(ckc_spec::replay::arg(models[i as int], 0)));
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(models.take(i as int) == models);
    }
    (result, arena)
}

fn project_roots(arena: &mut ETermArena, rows: &Vec<usize>) -> (out: Vec<usize>)
    requires
        arena_ok(old(arena)),
        roots_valid(old(arena).nodes@, rows@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        roots_valid(final(arena).nodes@, out@),
        root_terms(final(arena).nodes@, out@) == root_terms(old(arena).nodes@, rows@).map_values(
            |r: Term| ckc_spec::replay::arg(r, 0),
        ),
{
    let mut owned = crate::k2_reject::empty_arena();
    core::mem::swap(arena, &mut owned);
    let (out, mut owned) = project_roots_inner(owned, rows);
    core::mem::swap(arena, &mut owned);
    out
}

proof fn rows_prefix(before: Seq<ENode>, after: Seq<ENode>, rows: Seq<Vec<usize>>)
    requires
        before.is_prefix_of(after),
        crate::k2_engine::rows_valid(before, rows),
    ensures
        crate::k2_engine::rows_valid(after, rows),
        crate::k2_engine::rows_view(after, rows) == crate::k2_engine::rows_view(before, rows),
{
    assert forall|i: int| 0 <= i < rows.len() implies #[trigger] roots_valid(after, rows[i]@)
        && root_terms(after, rows[i]@) == root_terms(before, rows[i]@) by {
        crate::k2_engine::roots_models_prefix(before, after, rows[i]@);
    }
    assert forall|i: int| 0 <= i < rows.len() implies #[trigger] crate::k2_engine::rows_view(
        after,
        rows,
    )[i] == crate::k2_engine::rows_view(before, rows)[i] by {
        crate::k2_engine::roots_models_prefix(before, after, rows[i]@);
    }
    assert_seqs_equal!(crate::k2_engine::rows_view(after, rows) == crate::k2_engine::rows_view(before, rows));
}

fn row_ground_exec(arena: &ETermArena, row: &Vec<usize>) -> (out: bool)
    requires
        arena_ok(arena),
        roots_valid(arena.nodes@, row@),
    ensures
        out == ckc_spec::answers::row_ground(root_terms(arena.nodes@, row@)),
{
    let mut i = 0usize;
    while i < row.len()
        invariant
            arena_ok(arena),
            roots_valid(arena.nodes@, row@),
            i <= row.len(),
            forall|j: int|
                0 <= j < i ==> ckc_spec::term::ground(#[trigger] root_terms(arena.nodes@, row@)[j]),
        decreases row.len() - i,
    {
        if !crate::k2_walk::ground_root(arena, row[i]) {
            proof {
                assert(!ckc_spec::term::ground(root_terms(arena.nodes@, row@)[i as int]));
            }
            return false;
        }
        i += 1;
    }
    true
}

fn some_nonground_exec(arena: &ETermArena, rows: &Vec<Vec<usize>>) -> (out: bool)
    requires
        arena_ok(arena),
        crate::k2_engine::rows_valid(arena.nodes@, rows@),
    ensures
        out == ckc_spec::answers::some_nonground(crate::k2_engine::rows_view(arena.nodes@, rows@)),
{
    let mut i = 0usize;
    while i < rows.len()
        invariant
            arena_ok(arena),
            crate::k2_engine::rows_valid(arena.nodes@, rows@),
            i <= rows.len(),
            forall|j: int|
                0 <= j < i ==> ckc_spec::answers::row_ground(
                    #[trigger] crate::k2_engine::rows_view(arena.nodes@, rows@)[j],
                ),
        decreases rows.len() - i,
    {
        if !row_ground_exec(arena, &rows[i]) {
            proof {
                assert(!ckc_spec::answers::row_ground(
                    crate::k2_engine::rows_view(arena.nodes@, rows@)[i as int],
                ));
            }
            return true;
        }
        i += 1;
    }
    false
}

proof fn list_ground(terms: Seq<Term>)
    requires
        ckc_spec::answers::row_ground(terms),
    ensures
        ckc_spec::term::ground(ckc_spec::engine::list_term(terms)),
    decreases terms.len(),
{
    reveal_with_fuel(ckc_spec::engine::list_term, 1);
    if terms.len() > 0 {
        assert forall|i: int| 0 <= i < terms.drop_first().len() implies ckc_spec::term::ground(
            #[trigger] terms.drop_first()[i],
        ) by {
            assert(terms.drop_first()[i] == terms[i + 1]);
        }
        list_ground(terms.drop_first());
        let tail = ckc_spec::engine::list_term(terms.drop_first());
        assert(ckc_spec::term::ground(terms[0]));
        assert_seqs_equal!(seq![terms[0], tail].drop_first() == seq![tail]);
        assert_seqs_equal!(seq![tail].drop_first() == Seq::empty());
        reveal_with_fuel(ckc_spec::term::ground_all, 3);
        assert(ckc_spec::term::ground_all(seq![terms[0], tail]));
    }
    reveal(ckc_spec::term::ground);
}

fn sol_terms_inner(mut arena: ETermArena, rows: &Vec<Vec<usize>>) -> (out: (Vec<usize>, ETermArena))
    requires
        arena_ok(&arena),
        crate::k2_engine::rows_valid(arena.nodes@, rows@),
        !ckc_spec::answers::some_nonground(crate::k2_engine::rows_view(arena.nodes@, rows@)),
    ensures
        arena_ok(&out.1),
        arena.nodes@.is_prefix_of(out.1.nodes@),
        roots_valid(out.1.nodes@, out.0@),
        root_terms(out.1.nodes@, out.0@) == crate::k2_engine::rows_view(
            arena.nodes@,
            rows@,
        ).map_values(|r: Seq<Term>| ckc_spec::answers::sol_term(r)),
        ckc_spec::answers::row_ground(root_terms(out.1.nodes@, out.0@)),
{
    let ghost base = arena.nodes@;
    let ghost models = crate::k2_engine::rows_view(base, rows@);
    let mut result = Vec::new();
    let mut i = 0usize;
    let name: &[u8] = b"sol";
    proof {
        reveal_byteslit(b"sol");
        reveal_strlit("sol");
        reveal(ckc_spec::v1text::ascii);
        assert(name@ == ckc_spec::v1text::ascii("sol"@));
    }
    while i < rows.len()
        invariant
            arena_ok(&arena),
            base.is_prefix_of(arena.nodes@),
            crate::k2_engine::rows_valid(base, rows@),
            crate::k2_engine::rows_valid(arena.nodes@, rows@),
            models == crate::k2_engine::rows_view(base, rows@),
            models == crate::k2_engine::rows_view(arena.nodes@, rows@),
            !ckc_spec::answers::some_nonground(models),
            i <= rows.len(),
            result.len() == i,
            roots_valid(arena.nodes@, result@),
            ckc_spec::answers::row_ground(root_terms(arena.nodes@, result@)),
            root_terms(arena.nodes@, result@) == models.take(i as int).map_values(
                |r: Seq<Term>| ckc_spec::answers::sol_term(r),
            ),
            name@ == ckc_spec::v1text::ascii("sol"@),
        decreases rows.len() - i,
    {
        let ghost before = arena.nodes@;
        let ghost previous = result@;
        let list = crate::k2_walk::list_root(&mut arena, &rows[i]);
        let root = comp1(&mut arena, name, list);
        proof {
            rows_prefix(before, arena.nodes@, rows@);
            crate::k2_engine::roots_models_prefix(before, arena.nodes@, previous);
            assert(ckc_spec::answers::row_ground(models[i as int]));
            list_ground(models[i as int]);
            let list_model = ckc_spec::engine::list_term(models[i as int]);
            assert_seqs_equal!(seq![list_model].drop_first() == Seq::empty());
            reveal_with_fuel(ckc_spec::term::ground_all, 2);
            assert(ckc_spec::term::ground_all(seq![list_model]));
            reveal(ckc_spec::term::ground);
            assert(ckc_spec::term::ground(arena@[root as int]));
        }
        result.push(root);
        proof {
            assert_seqs_equal!(root_terms(arena.nodes@, result@)
                == root_terms(arena.nodes@, previous).push(arena@[root as int]));
            assert_seqs_equal!(models.take(i as int + 1).map_values(|r: Seq<Term>| ckc_spec::answers::sol_term(r))
                == models.take(i as int).map_values(|r: Seq<Term>| ckc_spec::answers::sol_term(r)).push(ckc_spec::answers::sol_term(models[i as int])));
            assert forall|j: int| 0 <= j < result.len() implies ckc_spec::term::ground(
                #[trigger] root_terms(arena.nodes@, result@)[j],
            ) by {
                if j < previous.len() {
                    assert(result@[j] == previous[j]);
                } else {
                    assert(result@[j] == root);
                }
            }
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(models.take(i as int) == models);
    }
    (result, arena)
}

fn solutions_root(arena: &mut ETermArena, rows: &Vec<Vec<usize>>) -> (out: usize)
    requires
        arena_ok(old(arena)),
        crate::k2_engine::rows_valid(old(arena).nodes@, rows@),
        !ckc_spec::answers::some_nonground(crate::k2_engine::rows_view(old(arena).nodes@, rows@)),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        root_ok(final(arena), out),
        final(arena)@[out as int] == Term::Comp(
            ckc_spec::v1text::ascii("solutions"@),
            seq![
                ckc_spec::engine::list_term(
                    ckc_spec::engine::sort_unique(
                        crate::k2_engine::rows_view(old(arena).nodes@, rows@).map_values(
                            |r: Seq<Term>| ckc_spec::answers::sol_term(r),
                        ),
                    ),
                ),
            ],
        ),
{
    let mut owned = crate::k2_reject::empty_arena();
    core::mem::swap(arena, &mut owned);
    let (terms, mut owned) = sol_terms_inner(owned, rows);
    core::mem::swap(arena, &mut owned);
    proof {
        crate::k2_aggregate::sort_views(arena, terms@);
        assert forall|i: int| 0 <= i < terms.len() implies ckc_spec::term::ground(
            arena@[#[trigger] terms@[i] as int],
        ) by {
            assert(ckc_spec::term::ground(root_terms(arena.nodes@, terms@)[i]));
        }
    }
    let sorted = crate::k2_sort::sort_unique(arena, &terms);
    proof {
        crate::k2_aggregate::sort_views(arena, sorted@);
    }
    let list = crate::k2_walk::list_root(arena, &sorted);
    let name: &[u8] = b"solutions";
    proof {
        reveal_byteslit(b"solutions");
        reveal_strlit("solutions");
        reveal(ckc_spec::v1text::ascii);
        assert(name@ == ckc_spec::v1text::ascii("solutions"@));
    }
    comp1(arena, name, list)
}

fn limit_root(arena: &mut ETermArena) -> (out: usize)
    requires
        arena_ok(old(arena)),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        root_ok(final(arena), out),
        final(arena)@[out as int] == ckc_spec::answers::limit(),
{
    let limit: &[u8] = b"limit";
    let indeterminate: &[u8] = b"indeterminate";
    proof {
        reveal_byteslit(b"limit");
        reveal_strlit("limit");
        reveal_byteslit(b"indeterminate");
        reveal_strlit("indeterminate");
        reveal(ckc_spec::v1text::ascii);
        assert(limit@ == ckc_spec::v1text::ascii("limit"@));
        assert(indeterminate@ == ckc_spec::v1text::ascii("indeterminate"@));
    }
    let why = atom_root(arena, limit);
    comp1(arena, indeterminate, why)
}

pub open spec fn result_view(nodes: Seq<ENode>, result: Result<usize, EOut>) -> Result<
    Term,
    ckc_spec::replay::Out,
> {
    match result {
        Ok(root) => Ok(nodes[root as int].term@),
        Err(out) => Err(out@),
    }
}

pub fn answer_result_exec(
    arena: &mut ETermArena,
    db: &Vec<crate::k2_engine::EClause>,
    query: &EQuery,
) -> (out: Result<usize, EOut>)
    requires
        arena_ok(old(arena)),
        crate::k2_engine::db_valid(old(arena).nodes@, db@),
        query_ok(old(arena).nodes@, query),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Ok(root) ==> root_ok(final(arena), root),
        result_view(final(arena).nodes@, out) == ckc_spec::answers::answer_result(
            crate::k2_engine::db_view(old(arena).nodes@, db@),
            query.file@,
            root_terms(old(arena).nodes@, query.rows@),
        ),
{
    hide(ckc_spec::answers::answer_result);
    let ghost base = arena.nodes@;
    let sol = project_roots(arena, &query.rows);
    proof {
        query_prefix(base, arena.nodes@, query);
        crate::k2_engine::db_models_prefix(base, arena.nodes@, db@);
    }
    let result = crate::k2_engine::solve(
        arena,
        db,
        query.goal,
        100,
        sol,
        query.rows.len() > 0,
        100000,
    );
    proof {
        reveal(ckc_spec::answers::answer_result);
        reveal(ckc_spec::engine::answer_depth);
        reveal(ckc_spec::engine::answer_inf);
    }
    match result {
        crate::k2_engine::EROut::Sol => {
            let name: &[u8] = b"yes";
            proof {
                reveal_byteslit(b"yes");
                reveal_strlit("yes");
                reveal(ckc_spec::v1text::ascii);
                assert(name@ == ckc_spec::v1text::ascii("yes"@));
            }
            Ok(atom_root(arena, name))
        },
        crate::k2_engine::EROut::End { complete, rows } => {
            if query.rows.len() == 0 {
                if !complete {
                    return Ok(limit_root(arena));
                }
                let name: &[u8] = b"no";
                let why: &[u8] = b"finite_failure";
                proof {
                    reveal_byteslit(b"no");
                    reveal_strlit("no");
                    reveal_byteslit(b"finite_failure");
                    reveal_strlit("finite_failure");
                    reveal(ckc_spec::v1text::ascii);
                    assert(name@ == ckc_spec::v1text::ascii("no"@));
                    assert(why@ == ckc_spec::v1text::ascii("finite_failure"@));
                }
                let detail = atom_root(arena, why);
                return Ok(comp1(arena, name, detail));
            }
            if some_nonground_exec(arena, &rows) {
                let name: &[u8] = b"nonground_solution";
                proof {
                    reveal_byteslit(b"nonground_solution");
                    reveal_strlit("nonground_solution");
                    reveal(ckc_spec::v1text::ascii);
                    assert(name@ == ckc_spec::v1text::ascii("nonground_solution"@));
                }
                return Err(crate::k2_output::named_atom_error(name, query.qid.as_slice(), true));
            }
            if !complete {
                return Ok(limit_root(arena));
            }
            Ok(solutions_root(arena, &rows))
        },
    }
}

pub fn print_answers_exec(arena: &mut ETermArena, qid: &[u8], qsha: &[u8], result: usize) -> (out:
    EOut)
    requires
        root_ok(old(arena), result),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out@ == ckc_spec::replay::ok(
            ckc_spec::v1text::print_answers(
                ckc_spec::v1text::AnswersFile {
                    qid: qid@,
                    qsha: qsha@,
                    result: old(arena)@[result as int],
                },
            ),
        ),
{
    let ghost before = arena.nodes@;
    let version: &[u8] = b"v1";
    let query_name: &[u8] = b"query_sha256";
    let result_name: &[u8] = b"result";
    let record_name: &[u8] = b"$guideline_answers";
    let prefix: &[u8] = b"% ";
    let suffix: &[u8] =
        b" answered against the loaded composition by ace_to_pl answer mode; do not edit.\n";
    proof {
        reveal_byteslit(b"v1");
        reveal_strlit("v1");
        reveal_byteslit(b"query_sha256");
        reveal_strlit("query_sha256");
        reveal_byteslit(b"result");
        reveal_strlit("result");
        reveal_byteslit(b"$guideline_answers");
        reveal_strlit("$guideline_answers");
        reveal_byteslit(b"% ");
        reveal_strlit("% ");
        reveal_byteslit(
            b" answered against the loaded composition by ace_to_pl answer mode; do not edit.\n",
        );
        reveal_strlit(
            " answered against the loaded composition by ace_to_pl answer mode; do not edit.\n",
        );
        reveal(ckc_spec::v1text::ascii);
        assert(version@ == ckc_spec::v1text::ascii("v1"@));
        assert(query_name@ == ckc_spec::v1text::ascii("query_sha256"@));
        assert(result_name@ == ckc_spec::v1text::ascii("result"@));
        assert(record_name@ == ckc_spec::v1text::ascii("$guideline_answers"@));
        assert(prefix@ == ckc_spec::v1text::ascii("% "@));
        assert(suffix@ == ckc_spec::v1text::ascii(
            " answered against the loaded composition by ace_to_pl answer mode; do not edit.\n"@,
        ));
    }
    let v1 = atom_root(arena, version);
    let qid_root = atom_root(arena, qid);
    let qsha_root = atom_root(arena, qsha);
    let query_root = comp1(arena, query_name, qsha_root);
    let result_root = comp1(arena, result_name, result);
    let mut children = Vec::new();
    children.push(v1);
    children.push(qid_root);
    children.push(query_root);
    children.push(result_root);
    proof {
        crate::k2_term::arena_prefix_stable(before, arena);
        crate::k2_term::child_terms_match(
            arena.nodes@,
            children@,
            seq![
                Term::Atom(version@),
                Term::Atom(qid@),
                Term::Comp(query_name@, seq![Term::Atom(qsha@)]),
                Term::Comp(result_name@, seq![before[result as int].term@]),
            ],
        );
    }
    let record = crate::k2_output::comp_root(arena, record_name, children);
    let mut bytes = slice_to_vec(prefix);
    let mut id_bytes = slice_to_vec(qid);
    bytes.append(&mut id_bytes);
    let mut suffix_bytes = slice_to_vec(suffix);
    bytes.append(&mut suffix_bytes);
    let mut record_bytes = crate::k2_term::term_line(arena, record);
    bytes.append(&mut record_bytes);
    EOut { rc: 0, out: bytes, err: Vec::new() }
}

} // verus!
