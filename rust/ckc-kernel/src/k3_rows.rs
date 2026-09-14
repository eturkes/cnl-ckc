use crate::k2_answers::EQuery;
#[cfg(verus_keep_ghost)]
use crate::k2_answers::{query_ok, result_view};
use crate::k2_engine::EClause;
#[cfg(verus_keep_ghost)]
use crate::k2_engine::{db_valid, db_view, root_terms, roots_valid, roots_work};
use crate::k2_term::{ENode, ENodeKind, ETermArena};
#[cfg(verus_keep_ghost)]
use crate::k2_term::{arena_ok, node_ok, root_ok};
use crate::k3_front::EAnswers;
#[cfg(verus_keep_ghost)]
use crate::k3_front::answers_ok;
use ckc_spec::replay::EOut;
#[cfg(verus_keep_ghost)]
use ckc_spec::term::Term;
use ckc_spec::trace::*;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub open spec fn leaves_all(ts: Seq<Term>) -> Seq<Term>
    decreases ts.len(),
{
    if ts.len() == 0 { Seq::empty() } else { conj_leaves(ts[0]) + leaves_all(ts.drop_first()) }
}
proof fn leaves_concat(a: Seq<Term>, b: Seq<Term>)
    ensures leaves_all(a + b) == leaves_all(a) + leaves_all(b),
    decreases a.len(),
{
    if a.len() > 0 {
        leaves_concat(a.drop_first(), b);
        assert_seqs_equal!((a + b).drop_first() == a.drop_first() + b);
    }
    reveal_with_fuel(leaves_all, 2);
}
proof fn work_terms(nodes: Seq<ENode>, roots: Seq<usize>)
    ensures roots_work(nodes, roots) == crate::k2_engine::terms_size(root_terms(nodes, roots)),
    decreases roots.len(),
{
    if roots.len() > 0 {
        work_terms(nodes, roots.drop_first());
        assert_seqs_equal!(root_terms(nodes, roots).drop_first() == root_terms(nodes, roots.drop_first()));
    }
    reveal(roots_work); reveal(crate::k2_engine::terms_size);
}

fn flatten(arena: &ETermArena, root: usize) -> (out: Vec<usize>)
    requires root_ok(arena, root),
    ensures roots_valid(arena.nodes@, out@), root_terms(arena.nodes@, out@) == conj_leaves(arena@[root as int]),
{
    let ghost model = arena@[root as int];
    let mut tasks = Vec::new(); tasks.push(root);
    let mut out = Vec::new();
    proof {
        assert_seqs_equal!(root_terms(arena.nodes@, tasks@) == seq![model]);
        reveal_with_fuel(leaves_all, 2);
    }
    while tasks.len() > 0
        invariant
            root_ok(arena, root), model == arena@[root as int], roots_valid(arena.nodes@, tasks@), roots_valid(arena.nodes@, out@),
            conj_leaves(model) == root_terms(arena.nodes@, out@) + leaves_all(root_terms(arena.nodes@, tasks@)),
        decreases roots_work(arena.nodes@, tasks@),
    {
        let ghost previous = tasks@;
        let current = tasks.remove(0);
        proof {
            assert(current == previous[0]);
            assert(node_ok(arena.nodes@, current as int));
            assert(root_terms(arena.nodes@, previous)[0] == arena@[current as int]);
            assert_seqs_equal!(root_terms(arena.nodes@, previous).drop_first() == root_terms(arena.nodes@, tasks@));
            reveal(leaves_all); reveal(roots_work);
        }
        let comma: &[u8] = b",";
        proof {
            reveal_byteslit(b","); reveal_strlit(","); reveal(ckc_spec::v1text::ascii);
            assert(comma@ == ckc_spec::engine::comma_name());
        }
        if crate::k3_front::is_comp(arena, current, comma, 2) {
            let mut children = crate::k2_engine::args_roots(arena, current);
            let ghost child_terms = root_terms(arena.nodes@, children@);
            proof {
                assert(child_terms.len() == 2);
                assert(arena@[current as int] == Term::Comp(comma@, child_terms));
                assert_seqs_equal!(child_terms == seq![child_terms[0], child_terms[1]]);
                reveal(conj_leaves); reveal_with_fuel(leaves_all, 3);
                leaves_concat(child_terms, root_terms(arena.nodes@, tasks@));
                crate::k2_engine::roots_work_concat(arena.nodes@, children@, tasks@);
                work_terms(arena.nodes@, children@);
                reveal(crate::k2_engine::term_size);
            }
            let ghost front = children@;
            let ghost rest = tasks@;
            children.append(&mut tasks); tasks = children;
            proof {
                assert_seqs_equal!(root_terms(arena.nodes@, tasks@) == child_terms + root_terms(arena.nodes@, rest));
                assert(roots_valid(arena.nodes@, tasks@));
            }
        } else {
            let ghost old_out = out@;
            out.push(current);
            proof {
                reveal(conj_leaves);
                assert(conj_leaves(arena@[current as int]) == seq![arena@[current as int]]);
                assert_seqs_equal!(root_terms(arena.nodes@, out@) == root_terms(arena.nodes@, old_out).push(arena@[current as int]));
                assert(roots_valid(arena.nodes@, out@));
            }
        }
    }
    proof { assert_seqs_equal!(root_terms(arena.nodes@, tasks@) == Seq::empty()); reveal(leaves_all); }
    out
}

fn bind_exec(mut arena: ETermArena, root: usize, vars: &Vec<usize>, values: &Vec<usize>) -> (out: (ETermArena, usize))
    requires root_ok(&arena, root), roots_valid(arena.nodes@, vars@), roots_valid(arena.nodes@, values@),
    ensures
        arena_ok(&out.0), arena.nodes@.is_prefix_of(out.0.nodes@), root_ok(&out.0, out.1),
        out.0@[out.1 as int] == bind(arena@[root as int], root_terms(arena.nodes@, vars@), root_terms(arena.nodes@, values@), 0),
{
    let ghost origin = arena.nodes@;
    let ghost v = root_terms(origin, vars@);
    let ghost vs = root_terms(origin, values@);
    let ghost target = bind(arena@[root as int], v, vs, 0);
    let mut current = root;
    let mut i = 0usize;
    while i < vars.len() && i < values.len()
        invariant
            arena_ok(&arena), origin.is_prefix_of(arena.nodes@), root_ok(&arena, current),
            roots_valid(arena.nodes@, vars@), roots_valid(arena.nodes@, values@),
            v == root_terms(origin, vars@), v == root_terms(arena.nodes@, vars@),
            vs == root_terms(origin, values@), vs == root_terms(arena.nodes@, values@),
            target == bind(origin[root as int].term@, v, vs, 0),
            i <= vars.len(), i <= values.len(), target == bind(arena@[current as int], v, vs, i as nat),
        decreases vars.len() - i,
    {
        let ghost before = arena.nodes@;
        let variable = crate::k2_answers::variable_key(&arena, vars[i]);
        proof { reveal(bind); }
        if let Some(key) = variable {
            current = crate::k2_engine::subst_root(&mut arena, current, key, values[i]);
            proof {
                crate::k2_engine::roots_models_prefix(before, arena.nodes@, vars@);
                crate::k2_engine::roots_models_prefix(before, arena.nodes@, values@);
                assert(origin.is_prefix_of(arena.nodes@));
            }
        }
        i += 1;
    }
    proof { reveal(bind); }
    (arena, current)
}

pub enum ERow { Row { payload: usize, work: usize, nodes: usize, base: usize }, Err(EOut) }
pub open spec fn row_view(nodes: Seq<ENode>, row: &ERow) -> RowOut {
    match row {
        ERow::Row { payload, work, nodes: count, base } => RowOut::Row(nodes[*payload as int].term@, *work as nat + *count as nat, *base as nat),
        ERow::Err(o) => RowOut::Err(o@),
    }
}
pub open spec fn row_valid(nodes: Seq<ENode>, row: &ERow) -> bool {
    match row { ERow::Row { payload, base, .. } => *payload < nodes.len() && *base <= nodes.len(), ERow::Err(_) => true }
}

fn unproved_root(arena: &mut ETermArena, finite: bool) -> (out: usize)
    requires arena_ok(old(arena)),
    ensures arena_ok(final(arena)), old(arena).nodes@.is_prefix_of(final(arena).nodes@), root_ok(final(arena), out),
        final(arena)@[out as int] == unproved(if finite { "finite_failure"@ } else { "limit"@ }),
{
    let name: &[u8] = b"unproved";
    let why: &[u8] = if finite { b"finite_failure" } else { b"limit" };
    proof {
        reveal_byteslit(b"unproved"); reveal_strlit("unproved");
        reveal_byteslit(b"finite_failure"); reveal_strlit("finite_failure");
        reveal_byteslit(b"limit"); reveal_strlit("limit"); reveal(ckc_spec::v1text::ascii);
        assert(name@ == ckc_spec::v1text::ascii("unproved"@));
        assert(why@ == ckc_spec::v1text::ascii(if finite { "finite_failure"@ } else { "limit"@ }));
    }
    let reason = crate::k2_output::atom_root(arena, why);
    crate::k2_output::comp1(arena, name, reason)
}

fn prove(mut arena: ETermArena, db: &Vec<EClause>, digests: &Vec<Vec<u8>>, goal: usize, base: usize) -> (out: (ETermArena, ERow))
    requires root_ok(&arena, goal), db_valid(arena.nodes@, db@), base <= arena.nodes.len(),
    ensures
        arena_ok(&out.0), arena.nodes@.is_prefix_of(out.0.nodes@), row_valid(out.0.nodes@, &out.1),
        row_view(out.0.nodes@, &out.1) == prove_row(db_view(arena.nodes@, db@), digests_view(digests@), arena@[goal as int], base as nat),
{
    let ghost origin = arena.nodes@;
    let ghost program = db_view(origin, db@);
    let ghost model = arena@[goal as int];
    let goals = flatten(&arena, goal);
    let initial = crate::k3_machine::roots(&mut arena, &goals, Ghost(db.len() as nat));
    proof {
        crate::k2_engine::db_models_prefix(origin, arena.nodes@, db@);
        crate::k2_engine::roots_models_prefix(origin, arena.nodes@, goals@);
        assert(crate::k3_state::cfg_view(arena.nodes@, &initial) == roots_cfg(conj_leaves(model)));
    }
    let ghost before_run = arena.nodes@;
    let (result, left) = crate::k3_machine::run(&mut arena, db, initial, 100000);
    let work = 100000usize - left;
    proof {
        crate::k2_engine::db_models_prefix(before_run, arena.nodes@, db@);
        crate::k2_engine::roots_models_prefix(before_run, arena.nodes@, goals@);
        reveal(ckc_spec::engine::trace_inf);
        assert((crate::k3_state::out_view(arena.nodes@, &result), left as nat) == trun(program, roots_cfg(conj_leaves(model)), ckc_spec::engine::trace_inf()));
    }
    match result {
        crate::k3_state::EOut::Failed(finite) => {
            let payload = unproved_root(&mut arena, finite);
            (arena, ERow::Row { payload, work, nodes: 0, base })
        },
        crate::k3_state::EOut::Limit => {
            let payload = unproved_root(&mut arena, false);
            (arena, ERow::Row { payload, work, nodes: 0, base })
        },
        crate::k3_state::EOut::Proved(log) => {
            let ghost before_mat = arena.nodes@;
            match crate::k3_materialize::forest(&mut arena, db, digests, &goals, &log, base) {
                crate::k3_materialize::EMat::Err(e) => (arena, ERow::Err(e)),
                crate::k3_materialize::EMat::Ok { roots, base: next, nodes } => {
                    let list = crate::k2_walk::list_root(&mut arena, &roots);
                    let name: &[u8] = b"proved";
                    proof {
                        reveal_byteslit(b"proved"); reveal_strlit("proved"); reveal(ckc_spec::v1text::ascii);
                        assert(name@ == ckc_spec::v1text::ascii("proved"@));
                    }
                    let payload = crate::k2_output::comp1(&mut arena, name, list);
                    (arena, ERow::Row { payload, work, nodes, base: next })
                },
            }
        },
    }
}

pub enum ERows { Ok(Vec<usize>), Trip, Err(EOut) }
pub open spec fn rows_view(nodes: Seq<ENode>, out: &ERows) -> Rows {
    match out { ERows::Ok(rs) => Rows::Ok(root_terms(nodes, rs@)), ERows::Trip => Rows::Trip, ERows::Err(o) => Rows::Err(o@) }
}

fn rows(mut arena: ETermArena, db: &Vec<EClause>, digests: &Vec<Vec<u8>>, goal: usize, vars: &Vec<usize>, sols: &Vec<usize>) -> (out: (ETermArena, ERows))
    requires root_ok(&arena, goal), db_valid(arena.nodes@, db@), roots_valid(arena.nodes@, vars@), roots_valid(arena.nodes@, sols@),
    ensures
        arena_ok(&out.0), arena.nodes@.is_prefix_of(out.0.nodes@), out.1 matches ERows::Ok(rs) ==> roots_valid(out.0.nodes@, rs@),
        rows_view(out.0.nodes@, &out.1) == trace_rows(db_view(arena.nodes@, db@), digests_view(digests@), arena@[goal as int], root_terms(arena.nodes@, vars@), root_terms(arena.nodes@, sols@), 0, trace_run_inf(), 0, Seq::empty()),
{
    hide(prove_row); hide(trace_rows);
    let ghost origin = arena.nodes@;
    let ghost program = db_view(origin, db@);
    let ghost model = arena@[goal as int];
    let ghost v = root_terms(origin, vars@);
    let ghost ss = root_terms(origin, sols@);
    let ghost target = trace_rows(program, digests_view(digests@), model, v, ss, 0, trace_run_inf(), 0, Seq::empty());
    let mut acc = Vec::new();
    let mut left = 1000000usize;
    let mut base = 0usize;
    let mut i = 0usize;
    proof {
        reveal(trace_run_inf);
        assert_seqs_equal!(root_terms(arena.nodes@, acc@) == Seq::empty());
    }
    while i < sols.len()
        invariant
            arena_ok(&arena), origin.is_prefix_of(arena.nodes@), db_valid(arena.nodes@, db@), db_view(arena.nodes@, db@) == program,
            program == db_view(origin, db@), root_ok(&arena, goal), goal < origin.len(), model == arena@[goal as int], model == origin[goal as int].term@,
            roots_valid(arena.nodes@, vars@), v == root_terms(arena.nodes@, vars@), v == root_terms(origin, vars@),
            roots_valid(arena.nodes@, sols@), ss == root_terms(arena.nodes@, sols@), ss == root_terms(origin, sols@),
            roots_valid(arena.nodes@, acc@), i <= sols.len(), left <= 1000000, base <= arena.nodes.len(),
            target == trace_rows(program, digests_view(digests@), model, v, ss, 0, trace_run_inf(), 0, Seq::empty()),
            target == trace_rows(program, digests_view(digests@), model, v, ss, i as nat, left as nat, base as nat, root_terms(arena.nodes@, acc@)),
        decreases sols.len() - i,
    {
        let ghost before = arena.nodes@;
        let ghost old_acc = root_terms(before, acc@);
        let values = crate::k2_walk::arg_root(&mut arena, sols[i], 0);
        proof {
            assert(origin.is_prefix_of(arena.nodes@));
            reveal(trace_rows);
        }
        let value_roots = match crate::k2_walk::list_items_exec(&arena, values) {
            None => return (arena, ERows::Ok(acc)),
            Some(rs) => rs,
        };
        proof { crate::k2_engine::roots_models_prefix(before, arena.nodes@, vars@); }
        let (next_arena, bound) = bind_exec(arena, goal, vars, &value_roots);
        arena = next_arena;
        proof { crate::k2_engine::db_models_prefix(before, arena.nodes@, db@); }
        let (next_arena, payload) = prove(arena, db, digests, bound, base);
        arena = next_arena;
        proof {
            crate::k2_engine::db_models_prefix(before, arena.nodes@, db@);
            crate::k2_engine::roots_models_prefix(before, arena.nodes@, vars@);
            crate::k2_engine::roots_models_prefix(before, arena.nodes@, sols@);
            crate::k2_engine::roots_models_prefix(before, arena.nodes@, acc@);
            assert(origin.is_prefix_of(arena.nodes@));
        }
        match payload {
            ERow::Err(e) => return (arena, ERows::Err(e)),
            ERow::Row { payload, work, nodes, base: next } => {
                if work > left || nodes > left - work { return (arena, ERows::Trip); }
                left -= work; left -= nodes; base = next;
                let name: &[u8] = b"sol";
                proof {
                    reveal_byteslit(b"sol"); reveal_strlit("sol"); reveal(ckc_spec::v1text::ascii);
                    assert(name@ == ckc_spec::v1text::ascii("sol"@));
                }
                let ghost prior = arena.nodes@;
                let entry = crate::k2_output::comp2(&mut arena, name, values, payload);
                let ghost previous = acc@;
                acc.push(entry);
                proof {
                    crate::k2_engine::db_models_prefix(prior, arena.nodes@, db@);
                    crate::k2_engine::roots_models_prefix(prior, arena.nodes@, vars@);
                    crate::k2_engine::roots_models_prefix(prior, arena.nodes@, sols@);
                    crate::k2_engine::roots_models_prefix(prior, arena.nodes@, previous);
                    assert_seqs_equal!(root_terms(arena.nodes@, acc@) == old_acc.push(arena@[entry as int]));
                    assert(roots_valid(arena.nodes@, acc@));
                }
                i += 1;
            },
        }
    }
    proof { reveal(trace_rows); }
    (arena, ERows::Ok(acc))
}

pub fn result_exec(arena: &mut ETermArena, db: &Vec<EClause>, digests: &Vec<Vec<u8>>, query: &EQuery, answer: &EAnswers) -> (out: Result<usize, EOut>)
    requires arena_ok(old(arena)), db_valid(old(arena).nodes@, db@), query_ok(old(arena).nodes@, query), answers_ok(old(arena).nodes@, answer),
    ensures
        arena_ok(final(arena)), old(arena).nodes@.is_prefix_of(final(arena).nodes@), out matches Ok(root) ==> root_ok(final(arena), root),
        result_view(final(arena).nodes@, out) == trace_result(db_view(old(arena).nodes@, db@), digests_view(digests@), query.file@, root_terms(old(arena).nodes@, query.rows@), answer.file@),
{
    let mut owned = crate::k2_reject::empty_arena();
    core::mem::swap(arena, &mut owned);
    let (mut owned, result) = result_inner(owned, db, digests, query, answer);
    core::mem::swap(arena, &mut owned);
    result
}

fn result_inner(mut arena: ETermArena, db: &Vec<EClause>, digests: &Vec<Vec<u8>>, query: &EQuery, answer: &EAnswers) -> (out: (ETermArena, Result<usize, EOut>))
    requires arena_ok(&arena), db_valid(arena.nodes@, db@), query_ok(arena.nodes@, query), answers_ok(arena.nodes@, answer),
    ensures
        arena_ok(&out.0), arena.nodes@.is_prefix_of(out.0.nodes@), out.1 matches Ok(root) ==> root_ok(&out.0, root),
        result_view(out.0.nodes@, out.1) == trace_result(db_view(arena.nodes@, db@), digests_view(digests@), query.file@, root_terms(arena.nodes@, query.rows@), answer.file@),
{
    hide(prove_row); hide(trace_rows);
    let ghost origin = arena.nodes@;
    let yes: &[u8] = b"yes";
    let solutions: &[u8] = b"solutions";
    proof {
        reveal_byteslit(b"yes"); reveal_strlit("yes");
        reveal_byteslit(b"solutions"); reveal_strlit("solutions"); reveal(ckc_spec::v1text::ascii);
        assert(yes@ == ckc_spec::v1text::ascii("yes"@));
        assert(solutions@ == ckc_spec::v1text::ascii("solutions"@));
        reveal(trace_run_inf);
    }
    if query.rows.len() == 0 {
        if !crate::k3_front::is_atom(&arena, answer.result, yes) { return (arena, Ok(answer.result)); }
        let (next_arena, result) = prove(arena, db, digests, query.goal, 0);
        arena = next_arena;
        match result {
            ERow::Err(e) => (arena, Err(e)),
            ERow::Row { payload, work, nodes, .. } => {
                if work > 1000000 || nodes > 1000000 - work {
                    let limit = crate::k2_answers::limit_root(&mut arena);
                    return (arena, Ok(limit));
                }
                let result = crate::k2_output::comp1(&mut arena, yes, payload);
                (arena, Ok(result))
            },
        }
    } else if crate::k3_front::is_comp(&arena, answer.result, solutions, 1) {
        let arguments = crate::k2_engine::args_roots(&arena, answer.result);
        let sols = match crate::k2_walk::list_items_exec(&arena, arguments[0]) {
            None => return (arena, Ok(answer.result)),
            Some(rs) => rs,
        };
        let vars = crate::k2_answers::project_roots(&mut arena, &query.rows);
        proof {
            crate::k2_engine::db_models_prefix(origin, arena.nodes@, db@);
            crate::k2_engine::roots_models_prefix(origin, arena.nodes@, sols@);
        }
        let (next_arena, result) = rows(arena, db, digests, query.goal, &vars, &sols);
        arena = next_arena;
        match result {
            ERows::Err(e) => (arena, Err(e)),
            ERows::Trip => { let limit = crate::k2_answers::limit_root(&mut arena); (arena, Ok(limit)) },
            ERows::Ok(rs) => {
                let list = crate::k2_walk::list_root(&mut arena, &rs);
                let result = crate::k2_output::comp1(&mut arena, solutions, list);
                (arena, Ok(result))
            },
        }
    } else { (arena, Ok(answer.result)) }
}

} // verus!
