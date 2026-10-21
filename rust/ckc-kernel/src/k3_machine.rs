use crate::k2_engine::{EClause, EPair};
#[cfg(verus_keep_ghost)]
use crate::k2_engine::{clause_valid, db_valid, db_view, root_terms, roots_valid};
use crate::k2_term::{ENode, ENodeKind, ETermArena};
#[cfg(verus_keep_ghost)]
use crate::k2_term::{arena_ok, node_ok};
use crate::k3_adapter::EUni;
#[cfg(verus_keep_ghost)]
use crate::k3_adapter::{uni_valid, uni_view};
use crate::k3_state::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::term::Term;
use ckc_spec::trace::*;
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub open spec fn predicate_dispatch(c: TCfg) -> bool {
    c.stack.len() > 0 && match c.stack[0] {
        TGoal::Lit(Term::Comp(name, args), depth, _) => depth > 0 && !(name
            == ckc_spec::engine::comma_name() && args.len() == 2) && !(name
            == ckc_spec::engine::naf_name() && args.len() == 1),
        _ => false,
    }
}

proof fn push_log(nodes: Seq<ENode>, db_len: nat, before: Seq<EEntry>, entry: EEntry)
    requires
        log_valid(nodes, db_len, before),
        entry_valid(nodes, db_len, &entry),
    ensures
        log_valid(nodes, db_len, before.push(entry)),
        log_view(nodes, before.push(entry)) == log_view(nodes, before).push(
            entry_view(nodes, &entry),
        ),
{
    assert forall|i: int| 0 <= i < before.push(entry).len() implies #[trigger] entry_valid(
        nodes,
        db_len,
        &before.push(entry)[i],
    ) by {
        if i < before.len() {
            assert(before.push(entry)[i] == before[i]);
        }
    }
    assert_seqs_equal!(log_view(nodes, before.push(entry)) == log_view(nodes, before).push(entry_view(nodes, &entry)));
}

proof fn push_alt(nodes: Seq<ENode>, log: Seq<EEntry>, before: Seq<EAlt>, entry: EAlt)
    requires
        alts_valid(nodes, log.len(), before),
        marks_sorted(before),
        alt_valid(nodes, log.len(), &entry, before.len()),
        alt_mark(&entry) == log.len(),
    ensures
        alts_valid(nodes, log.len(), before.push(entry)),
        marks_sorted(before.push(entry)),
        alts_view(nodes, log, before.push(entry)) == alts_view(nodes, log, before).push(
            alt_view(nodes, log, &entry),
        ),
{
    assert forall|i: int| 0 <= i < before.push(entry).len() implies #[trigger] alt_valid(
        nodes,
        log.len(),
        &before.push(entry)[i],
        i as nat,
    ) by {
        if i < before.len() {
            assert(before.push(entry)[i] == before[i]);
        }
    }
    assert forall|i: int, j: int|
        0 <= i <= j < before.push(entry).len() implies #[trigger] alt_mark(&before.push(entry)[i])
        <= #[trigger] alt_mark(&before.push(entry)[j]) by {
        if j < before.len() {
            assert(before.push(entry)[i] == before[i]);
            assert(before.push(entry)[j] == before[j]);
        } else if i < before.len() {
            assert(alt_valid(nodes, log.len(), &before[i], i as nat));
        }
    }
    assert_seqs_equal!(alts_view(nodes, log, before.push(entry)) == alts_view(nodes, log, before).push(alt_view(nodes, log, &entry)));
}

fn enter_clause(
    arena: &ETermArena,
    initial: ECfg,
    stack: Vec<EGoal>,
    fresh: usize,
    matched: usize,
    path: Vec<usize>,
    Ghost(db_len): Ghost<nat>,
) -> (out: EStep)
    requires
        cfg_valid(arena.nodes@, db_len, &initial),
        goals_valid(arena.nodes@, stack@),
        goals_levels(stack@, initial.alts.len() as nat),
        fresh <= arena.nodes.len(),
        matched < db_len,
        db_len <= usize::MAX,
    ensures
        step_valid(arena.nodes@, db_len, &out),
        step_view(arena.nodes@, &out) == TStep::Next(
            TCfg {
                stack: goals_view(arena.nodes@, stack@),
                fresh: fresh as nat,
                ci: 0,
                alts: cfg_view(arena.nodes@, &initial).alts.push(
                    TAlt::Cl {
                        stack: cfg_view(arena.nodes@, &initial).stack,
                        fresh: initial.fresh as nat,
                        ci: matched as nat + 1,
                        log: cfg_view(arena.nodes@, &initial).log,
                    },
                ),
                log: if in_naf(cfg_view(arena.nodes@, &initial).stack) {
                    cfg_view(arena.nodes@, &initial).log
                } else {
                    cfg_view(arena.nodes@, &initial).log.push(
                        (path_view(path@), TEv::Clause(matched as nat)),
                    )
                },
                pruned: initial.pruned,
            },
        ),
{
    let ghost old_model = cfg_view(arena.nodes@, &initial);
    let record = !in_naf_exec(arena, &initial.stack);
    let mut c = initial;
    let ghost old_alts = c.alts@;
    let ghost old_log = c.log@;
    proof {
        assert_seqs_equal!(old_log.take(old_log.len() as int) == old_log);
    }
    let alt = EAlt::Cl { stack: c.stack, fresh: c.fresh, ci: matched + 1, log_len: c.log.len() };
    proof {
        push_alt(arena.nodes@, c.log@, c.alts@, alt);
    }
    c.alts.push(alt);
    if record {
        let entry = EEntry { path, event: EEvent::Clause(matched) };
        proof {
            push_log(arena.nodes@, db_len, c.log@, entry);
        }
        c.log.push(entry);
        proof {
            alts_log_prefix(arena.nodes@, old_log, c.log@, c.alts@);
        }
    }
    proof {
        levels_weaken(stack@, old_alts.len(), c.alts.len() as nat);
        assert(alts_view(arena.nodes@, c.log@, c.alts@) == old_model.alts.push(
            TAlt::Cl {
                stack: old_model.stack,
                fresh: old_model.fresh,
                ci: matched as nat + 1,
                log: old_model.log,
            },
        ));
        assert(log_view(arena.nodes@, c.log@) == if record {
            old_model.log.push((path_view(path@), TEv::Clause(matched as nat)))
        } else {
            old_model.log
        });
    }
    c.stack = stack;
    c.fresh = fresh;
    c.ci = 0;
    EStep::Next(c)
}

fn enter_builtin(
    arena: &ETermArena,
    initial: ECfg,
    stack: Vec<EGoal>,
    root: usize,
    path: Vec<usize>,
    Ghost(db_len): Ghost<nat>,
) -> (out: EStep)
    requires
        cfg_valid(arena.nodes@, db_len, &initial),
        goals_valid(arena.nodes@, stack@),
        goals_levels(stack@, initial.alts.len() as nat),
        root < arena.nodes.len(),
    ensures
        step_valid(arena.nodes@, db_len, &out),
        step_view(arena.nodes@, &out) == TStep::Next(
            TCfg {
                stack: goals_view(arena.nodes@, stack@),
                log: if in_naf(cfg_view(arena.nodes@, &initial).stack) {
                    cfg_view(arena.nodes@, &initial).log
                } else {
                    cfg_view(arena.nodes@, &initial).log.push(
                        (path_view(path@), TEv::Builtin(arena@[root as int])),
                    )
                },
                ..cfg_view(arena.nodes@, &initial)
            },
        ),
{
    let record = !in_naf_exec(arena, &initial.stack);
    let mut c = initial;
    if record {
        let ghost old_log = c.log@;
        let entry = EEntry { path, event: EEvent::Builtin(root) };
        proof {
            push_log(arena.nodes@, db_len, c.log@, entry);
        }
        c.log.push(entry);
        proof {
            alts_log_prefix(arena.nodes@, old_log, c.log@, c.alts@);
        }
    }
    c.stack = stack;
    EStep::Next(c)
}

// The spec's arithmetic branch of `tstep` (contract da1 D8).
pub open spec fn arith_model(
    c: TCfg,
    name: Seq<u8>,
    args: Seq<Term>,
    pre: Seq<TGoal>,
    rest: Seq<TGoal>,
    path: Seq<nat>,
) -> TStep {
    match ckc_spec::arith::arith_result(Term::Comp(name, args)) {
        Option::Some((s, n)) => match tunify(
            ckc_spec::engine::zip(args, args.take(4) + seq![s, n]),
            pre + rest,
        ) {
            TUni::Ok(stack) => TStep::Next(
                TCfg {
                    stack,
                    log: if in_naf(c.stack) {
                        c.log
                    } else {
                        c.log.push(
                            (path, TEv::Builtin(Term::Comp(name, args.take(4) + seq![s, n]))),
                        )
                    },
                    ..c
                },
            ),
            TUni::Fail => tfail(c),
            TUni::Out => TStep::Limit,
        },
        Option::None => tfail(c),
    }
}

// An arithmetic goal: bind its outputs, record the goal with its outputs bound.
#[verifier::rlimit(5000)]
fn arith_enter(
    input: ETermArena,
    initial: ECfg,
    name: &Vec<u8>,
    args: &Vec<usize>,
    pre: Vec<EGoal>,
    rest: Vec<EGoal>,
    path: Vec<usize>,
    Ghost(db_len): Ghost<nat>,
) -> (out: (ETermArena, EStep))
    requires
        arena_ok(&input),
        cfg_valid(input.nodes@, db_len, &initial),
        roots_valid(input.nodes@, args@),
        args@.len() == 6,
        ckc_spec::arith::is_arith(name@, 6),
        goals_valid(input.nodes@, pre@),
        goals_levels(pre@, initial.alts.len() as nat),
        goals_valid(input.nodes@, rest@),
        goals_levels(rest@, initial.alts.len() as nat),
    ensures
        arena_ok(&out.0),
        input.nodes@.is_prefix_of(out.0.nodes@),
        step_valid(out.0.nodes@, db_len, &out.1),
        step_view(out.0.nodes@, &out.1) == arith_model(
            cfg_view(input.nodes@, &initial),
            name@,
            root_terms(input.nodes@, args@),
            goals_view(input.nodes@, pre@),
            goals_view(input.nodes@, rest@),
            path_view(path@),
        ),
{
    let ghost base = input.nodes@;
    let ghost terms = root_terms(base, args@);
    let ghost prefix = goals_view(base, pre@);
    let ghost continuation = goals_view(base, rest@);
    let level = initial.alts.len();
    let mut arena = input;
    let res = crate::arith_impl::arith_result_exec(&mut arena, name, args);
    proof {
        cfg_prefix(base, arena.nodes@, db_len, &initial);
        crate::k2_engine::roots_models_prefix(base, arena.nodes@, args@);
        goals_prefix(base, arena.nodes@, pre@);
        goals_prefix(base, arena.nodes@, rest@);
    }
    match res {
        Option::None => {
            let result = fail(&arena, initial, Ghost(db_len));
            (arena, result)
        },
        Option::Some((s, n)) => {
            let ghost mid = arena.nodes@;
            let mut kids: Vec<usize> = Vec::new();
            kids.push(args[0]);
            kids.push(args[1]);
            kids.push(args[2]);
            kids.push(args[3]);
            kids.push(s);
            kids.push(n);
            proof {
                reveal(roots_valid);
            }
            let kids_copy = kids.clone();
            let leaf = crate::k2_term::push_comp(&mut arena, name.clone(), kids);
            proof {
                reveal(crate::k2_term::child_terms);
                assert(crate::k2_term::child_terms(mid, kids@) =~= terms.take(4) + seq![
                    mid[s as int].term@,
                    mid[n as int].term@,
                ]);
                crate::k2_term::arena_prefix_stable(mid, &arena);
                cfg_prefix(base, arena.nodes@, db_len, &initial);
                crate::k2_engine::roots_models_prefix(base, arena.nodes@, args@);
                goals_prefix(base, arena.nodes@, pre@);
                goals_prefix(base, arena.nodes@, rest@);
            }
            let ghost with_leaf = arena.nodes@;
            let mut stack = pre;
            let mut tail = rest;
            proof {
                goals_concat(with_leaf, stack@, tail@, level as nat);
            }
            stack.append(&mut tail);
            proof {
                crate::k2_engine::roots_models_prefix(mid, arena.nodes@, kids_copy@);
                assert(root_terms(arena.nodes@, kids_copy@) =~= terms.take(4) + seq![
                    mid[s as int].term@,
                    mid[n as int].term@,
                ]);
            }
            let pairs = crate::k2_engine::zip_pairs(&arena, args, &kids_copy);
            proof {
                assert(crate::k2_engine::pairs_view(with_leaf, pairs@) == ckc_spec::engine::zip(
                    terms,
                    terms.take(4) + seq![mid[s as int].term@, mid[n as int].term@],
                ));
                assert(goals_view(with_leaf, stack@) =~= prefix + continuation);
            }
            let result = crate::k3_adapter::tunify_exec(
                &mut arena,
                pairs,
                stack,
                Ghost(level as nat),
            );
            proof {
                cfg_prefix(base, arena.nodes@, db_len, &initial);
                crate::k2_term::arena_prefix_stable(with_leaf, &arena);
            }
            match result {
                EUni::Ok(stack) => {
                    let out = enter_builtin(&arena, initial, stack, leaf, path, Ghost(db_len));
                    (arena, out)
                },
                EUni::Fail => {
                    let out = fail(&arena, initial, Ghost(db_len));
                    (arena, out)
                },
            }
        },
    }
}

proof fn call_none(
    db: Seq<ckc_spec::v1text::DocClause>,
    c: TCfg,
    name: Seq<u8>,
    args: Seq<Term>,
    depth: nat,
    pre: Seq<TGoal>,
    rest: Seq<TGoal>,
    from: nat,
    path: Seq<nat>,
)
    requires
        ckc_spec::engine::next_match(db, name, args.len(), from) is None,
    ensures
        tcall(db, c, name, args, depth, pre, rest, from, path) == tfail(c),
{
    reveal(tcall);
}

proof fn call_some(
    db: Seq<ckc_spec::v1text::DocClause>,
    c: TCfg,
    name: Seq<u8>,
    args: Seq<Term>,
    depth: nat,
    pre: Seq<TGoal>,
    rest: Seq<TGoal>,
    from: nat,
    path: Seq<nat>,
    matched: nat,
)
    requires
        ckc_spec::engine::next_match(db, name, args.len(), from) == Some(matched),
    ensures
        tcall(db, c, name, args, depth, pre, rest, from, path) == match tunify(
            ckc_spec::engine::zip(
                args,
                ckc_spec::engine::args_of(
                    ckc_spec::engine::shift(db[matched as int].head, c.fresh),
                ),
            ),
            pre + tbody_goals(db[matched as int].body, c.fresh, (depth - 1) as nat, path) + rest,
        ) {
            TUni::Ok(stack) => TStep::Next(
                TCfg {
                    stack,
                    fresh: c.fresh + ckc_spec::engine::clause_nvars(db[matched as int]),
                    ci: 0,
                    alts: c.alts.push(
                        TAlt::Cl { stack: c.stack, fresh: c.fresh, ci: matched + 1, log: c.log },
                    ),
                    log: if in_naf(c.stack) {
                        c.log
                    } else {
                        c.log.push((path, TEv::Clause(matched)))
                    },
                    pruned: c.pruned,
                },
            ),
            TUni::Fail => tcall(db, c, name, args, depth, pre, rest, matched + 1, path),
            TUni::Out => TStep::Limit,
        },
{
    reveal(tcall);
}

fn call(
    input: ETermArena,
    db: &Vec<EClause>,
    initial: ECfg,
    name: Vec<u8>,
    args: Vec<usize>,
    depth: usize,
    pre: Vec<EGoal>,
    rest: Vec<EGoal>,
    from: usize,
    path: Vec<usize>,
) -> (out: (ETermArena, EStep))
    requires
        arena_ok(&input),
        db_valid(input.nodes@, db@),
        cfg_valid(input.nodes@, db.len() as nat, &initial),
        roots_valid(input.nodes@, args@),
        goals_valid(input.nodes@, pre@),
        goals_levels(pre@, initial.alts.len() as nat),
        goals_valid(input.nodes@, rest@),
        goals_levels(rest@, initial.alts.len() as nat),
        depth > 0,
    ensures
        arena_ok(&out.0),
        input.nodes@.is_prefix_of(out.0.nodes@),
        step_valid(out.0.nodes@, db.len() as nat, &out.1),
        step_view(out.0.nodes@, &out.1) == tcall(
            db_view(input.nodes@, db@),
            cfg_view(input.nodes@, &initial),
            name@,
            root_terms(input.nodes@, args@),
            depth as nat,
            goals_view(input.nodes@, pre@),
            goals_view(input.nodes@, rest@),
            from as nat,
            path_view(path@),
        ),
{
    hide(tcall);
    let ghost base = input.nodes@;
    let ghost program = db_view(base, db@);
    let ghost c = cfg_view(base, &initial);
    let ghost actual = root_terms(base, args@);
    let ghost prefix = goals_view(base, pre@);
    let ghost continuation = goals_view(base, rest@);
    let mut arena = input;
    let mark = arena.nodes.len();
    let mut ci = from;
    while ci < db.len()
        invariant
            base == input.nodes@,
            mark == base.len(),
            arena_ok(&arena),
            base.is_prefix_of(arena.nodes@),
            db_valid(base, db@),
            cfg_valid(base, db.len() as nat, &initial),
            roots_valid(base, args@),
            goals_valid(base, pre@),
            goals_levels(pre@, initial.alts.len() as nat),
            goals_valid(base, rest@),
            goals_levels(rest@, initial.alts.len() as nat),
            depth > 0,
            program == db_view(base, db@),
            c == cfg_view(base, &initial),
            actual == root_terms(base, args@),
            prefix == goals_view(base, pre@),
            continuation == goals_view(base, rest@),
            tcall(
                program,
                c,
                name@,
                actual,
                depth as nat,
                prefix,
                continuation,
                from as nat,
                path_view(path@),
            ) == tcall(
                program,
                c,
                name@,
                actual,
                depth as nat,
                prefix,
                continuation,
                ci as nat,
                path_view(path@),
            ),
        decreases db.len() - ci,
    {
        proof {
            crate::k2_engine::db_models_prefix(base, arena.nodes@, db@);
            cfg_prefix(base, arena.nodes@, db.len() as nat, &initial);
            crate::k2_engine::roots_models_prefix(base, arena.nodes@, args@);
            goals_prefix(base, arena.nodes@, pre@);
            goals_prefix(base, arena.nodes@, rest@);
        }
        let matched = crate::k2_engine::next_match(&arena, db, &name, args.len(), ci);
        match matched {
            None => {
                proof {
                    call_none(
                        program,
                        c,
                        name@,
                        actual,
                        depth as nat,
                        prefix,
                        continuation,
                        ci as nat,
                        path_view(path@),
                    );
                }
                let result = fail(&arena, initial, Ghost(db.len() as nat));
                return (arena, result);
            },
            Some(m) => {
                proof {
                    assert(clause_valid(arena.nodes@, &db@[m as int]));
                    reveal(ckc_spec::engine::lit_fa);
                }
                if crate::k2_engine::head_clash(
                    &arena,
                    &args,
                    db[m].head,
                    Ghost(initial.fresh as nat),
                ) {
                    proof {
                        let pairs = ckc_spec::engine::zip(
                            actual,
                            ckc_spec::engine::args_of(
                                ckc_spec::engine::shift(program[m as int].head, c.fresh),
                            ),
                        );
                        let stack = prefix + tbody_goals(
                            program[m as int].body,
                            c.fresh,
                            (depth - 1) as nat,
                            path_view(path@),
                        ) + continuation;
                        let state = ckc_spec::engine::UState {
                            pairs,
                            stack: Seq::empty(),
                            sol: stack.map_values(|g: TGoal| tgoal_term(g)),
                        };
                        crate::k2_engine::clash_fails(state);
                        reveal(tunify);
                        call_some(
                            program,
                            c,
                            name@,
                            actual,
                            depth as nat,
                            prefix,
                            continuation,
                            ci as nat,
                            path_view(path@),
                            m as nat,
                        );
                    }
                    ci = m + 1;
                    continue;
                }
                let (pairs, prepared, fresh) = crate::k3_adapter::prepare(
                    &mut arena,
                    &db[m],
                    &args,
                    &rest,
                    initial.fresh,
                    depth - 1,
                    &path,
                    Ghost(initial.alts.len() as nat),
                );
                proof {
                    goals_prefix(base, arena.nodes@, pre@);
                }
                let mut stack = clone_goals(&arena, &pre, 0, Ghost(initial.alts.len() as nat));
                let ghost body = goals_view(arena.nodes@, prepared@);
                proof {
                    assert(goals_view(arena.nodes@, stack@) =~= prefix);
                    goals_concat(arena.nodes@, stack@, prepared@, initial.alts.len() as nat);
                }
                let mut tail = prepared;
                stack.append(&mut tail);
                proof {
                    assert(goals_view(arena.nodes@, stack@) =~= prefix + tbody_goals(
                        program[m as int].body,
                        c.fresh,
                        (depth - 1) as nat,
                        path_view(path@),
                    ) + continuation);
                    call_some(
                        program,
                        c,
                        name@,
                        actual,
                        depth as nat,
                        prefix,
                        continuation,
                        ci as nat,
                        path_view(path@),
                        m as nat,
                    );
                }
                let result = crate::k3_adapter::tunify_exec(
                    &mut arena,
                    pairs,
                    stack,
                    Ghost(initial.alts.len() as nat),
                );
                proof {
                    assert(base.is_prefix_of(arena.nodes@));
                    cfg_prefix(base, arena.nodes@, db.len() as nat, &initial);
                }
                match result {
                    EUni::Ok(stack) => {
                        let result = enter_clause(
                            &arena,
                            initial,
                            stack,
                            fresh,
                            m,
                            path,
                            Ghost(db.len() as nat),
                        );
                        return (arena, result);
                    },
                    EUni::Fail => {
                        crate::k2_store::truncate(&mut arena, mark);
                        proof {
                            assert(arena.nodes@ == base);
                        }
                        ci = m + 1;
                    },
                }
            },
        }
    }
    proof {
        cfg_prefix(base, arena.nodes@, db.len() as nat, &initial);
        reveal(ckc_spec::engine::next_match);
        call_none(
            program,
            c,
            name@,
            actual,
            depth as nat,
            prefix,
            continuation,
            ci as nat,
            path_view(path@),
        );
    }
    let result = fail(&arena, initial, Ghost(db.len() as nat));
    (arena, result)
}

fn predicate_parts(arena: &ETermArena, c: &ECfg, Ghost(db_len): Ghost<nat>) -> (out: Option<
    (Vec<u8>, Vec<usize>, usize, Vec<usize>),
>)
    requires
        arena_ok(arena),
        cfg_valid(arena.nodes@, db_len, c),
    ensures
        match out {
            Some((name, args, depth, path)) => {
                &&& predicate_dispatch(cfg_view(arena.nodes@, c))
                &&& roots_valid(arena.nodes@, args@)
                &&& depth > 0
                &&& cfg_view(arena.nodes@, c).stack[0] == TGoal::Lit(
                    Term::Comp(name@, root_terms(arena.nodes@, args@)),
                    depth as nat,
                    path_view(path@),
                )
            },
            None => !predicate_dispatch(cfg_view(arena.nodes@, c)),
        },
{
    if c.stack.len() == 0 {
        return None;
    }
    proof {
        assert(goal_valid(arena.nodes@, &c.stack@[0]));
    }
    match &c.stack[0] {
        EGoal::Lit { root, depth, path } => {
            if *depth == 0 {
                return None;
            }
            proof {
                assert(node_ok(arena.nodes@, *root as int));
            }
            match &arena.nodes[*root].kind {
                ENodeKind::Comp { name, child_roots, .. } => {
                    proof {
                        crate::k2_engine::node_comp_model(
                            arena.nodes@,
                            *root as int,
                            name@,
                            child_roots@,
                        );
                        assert_seqs_equal!(root_terms(arena.nodes@, child_roots@) == crate::k2_term::child_terms(arena.nodes@, child_roots@));
                    }
                    let comma: &[u8] = b",";
                    let naf: &[u8] = b"\\+";
                    proof {
                        reveal_byteslit(b",");
                        reveal_strlit(",");
                        reveal_byteslit(b"\\+");
                        reveal_strlit("\\+");
                        reveal(ckc_spec::v1text::ascii);
                        reveal(ckc_spec::engine::comma_name);
                        reveal(ckc_spec::engine::naf_name);
                        assert(comma@ == ckc_spec::engine::comma_name());
                        assert(naf@ == ckc_spec::engine::naf_name());
                    }
                    if (child_roots.len() == 2 && crate::k3_front::bytes_equal(name, comma)) || (
                    child_roots.len() == 1 && crate::k3_front::bytes_equal(name, naf)) {
                        None
                    } else {
                        proof {
                            assert(cfg_view(arena.nodes@, c).stack[0] == goal_view(
                                arena.nodes@,
                                &c.stack@[0],
                            ));
                            assert(cfg_view(arena.nodes@, c).stack[0] == TGoal::Lit(
                                Term::Comp(name@, root_terms(arena.nodes@, child_roots@)),
                                *depth as nat,
                                path_view(path@),
                            ));
                            assert(predicate_dispatch(cfg_view(arena.nodes@, c)));
                        }
                        Some((name.clone(), child_roots.clone(), *depth, path.clone()))
                    }
                },
                _ => None,
            }
        },
        EGoal::NafCut { .. } => None,
    }
}

fn simple(
    arena: &ETermArena,
    initial: ECfg,
    Ghost(db): Ghost<Seq<ckc_spec::v1text::DocClause>>,
) -> (out: EStep)
    requires
        arena_ok(arena),
        cfg_valid(arena.nodes@, db.len(), &initial),
        !predicate_dispatch(cfg_view(arena.nodes@, &initial)),
        tselected(cfg_view(arena.nodes@, &initial).stack) == 0,
    ensures
        step_valid(arena.nodes@, db.len(), &out),
        step_view(arena.nodes@, &out) == tstep(db, cfg_view(arena.nodes@, &initial)),
{
    let ghost model = cfg_view(arena.nodes@, &initial);
    proof {
        assert(model.stack.take(0) =~= Seq::<TGoal>::empty());
        assert(model.stack.skip(1) =~= model.stack.drop_first());
    }
    let mut c = initial;
    if c.stack.len() == 0 {
        return EStep::Sol(c.log);
    }
    let ghost old_stack = c.stack@;
    proof {
        assert(goal_valid(arena.nodes@, &c.stack@[0]));
        assert(goal_level(&c.stack@[0], c.alts.len() as nat));
    }
    let front = c.stack.remove(0);
    proof {
        assert(front == old_stack[0]);
        assert_seqs_equal!(goals_view(arena.nodes@, c.stack@) == model.stack.drop_first());
        assert(goals_valid(arena.nodes@, c.stack@));
        assert(goals_levels(c.stack@, c.alts.len() as nat));
    }
    match front {
        EGoal::NafCut { level } => {
            let pruned = match &c.alts[level] {
                EAlt::Naf { pruned, .. } => *pruned,
                EAlt::Cl { .. } => return EStep::Limit,
            };
            let ghost old_alts = c.alts@;
            c.alts.truncate(level);
            c.pruned = pruned;
            proof {
                assert_seqs_equal!(alts_view(arena.nodes@, c.log@, c.alts@) == model.alts.take(level as int));
                assert forall|i: int| 0 <= i < c.alts.len() implies #[trigger] alt_valid(
                    arena.nodes@,
                    c.log.len() as nat,
                    &c.alts@[i],
                    i as nat,
                ) by {
                    assert(c.alts@[i] == old_alts[i]);
                }
                assert(marks_sorted(c.alts@));
            }
            fail(arena, c, Ghost(db.len()))
        },
        EGoal::Lit { root, depth, path } => {
            if depth == 0 {
                c.pruned = true;
                return fail(arena, c, Ghost(db.len()));
            }
            proof {
                assert(node_ok(arena.nodes@, root as int));
            }
            match &arena.nodes[root].kind {
                ENodeKind::Comp { name, child_roots, .. } => {
                    let comma: &[u8] = b",";
                    let naf: &[u8] = b"\\+";
                    proof {
                        reveal_byteslit(b",");
                        reveal_strlit(",");
                        reveal_byteslit(b"\\+");
                        reveal_strlit("\\+");
                        reveal(ckc_spec::v1text::ascii);
                        reveal(ckc_spec::engine::comma_name);
                        reveal(ckc_spec::engine::naf_name);
                        assert(comma@ == ckc_spec::engine::comma_name());
                        assert(naf@ == ckc_spec::engine::naf_name());
                    }
                    if child_roots.len() == 2 && crate::k3_front::bytes_equal(name, comma) {
                        let ghost terms = crate::k2_term::child_terms(arena.nodes@, child_roots@);
                        proof {
                            assert(model.stack[0] == TGoal::Lit(
                                Term::Comp(name@, terms),
                                depth as nat,
                                path_view(path@),
                            ));
                            assert(terms[0] == arena.nodes@[child_roots@[0] as int].term@);
                            assert(terms[1] == arena.nodes@[child_roots@[1] as int].term@);
                        }
                        let mut expanded = Vec::new();
                        expanded.push(
                            EGoal::Lit { root: child_roots[0], depth, path: path.clone() },
                        );
                        expanded.push(EGoal::Lit { root: child_roots[1], depth, path });
                        proof {
                            assert(goals_valid(arena.nodes@, expanded@));
                            assert(goals_levels(expanded@, c.alts.len() as nat));
                            assert_seqs_equal!(goals_view(arena.nodes@, expanded@) == seq![
                                TGoal::Lit(terms[0], depth as nat, path_view(path@)),
                                TGoal::Lit(terms[1], depth as nat, path_view(path@))]);
                            assert(Seq::<TGoal>::empty() + goals_view(arena.nodes@, expanded@)
                                =~= goals_view(arena.nodes@, expanded@));
                            goals_concat(arena.nodes@, expanded@, c.stack@, c.alts.len() as nat);
                        }
                        expanded.append(&mut c.stack);
                        c.stack = expanded;
                        return EStep::Next(c);
                    }
                    if child_roots.len() == 1 && crate::k3_front::bytes_equal(name, naf) {
                        let ghost terms = crate::k2_term::child_terms(arena.nodes@, child_roots@);
                        proof {
                            assert(model.stack[0] == TGoal::Lit(
                                Term::Comp(name@, terms),
                                depth as nat,
                                path_view(path@),
                            ));
                            assert(terms[0] == arena.nodes@[child_roots@[0] as int].term@);
                            assert_seqs_equal!(c.log@.take(c.log.len() as int) == c.log@);
                            assert(child_roots@[0] < arena.nodes@.len());
                            assert(goals_view(arena.nodes@, c.stack@) == model.stack.drop_first());
                        }
                        if !crate::k3_safe::naf_safe_exec(arena, child_roots[0], &c.stack) {
                            c.pruned = true;
                            return fail(arena, c, Ghost(db.len()));
                        }
                        let level = c.alts.len();
                        let rest = clone_goals(arena, &c.stack, 0, Ghost(level as nat));
                        proof {
                            assert_seqs_equal!(goals_view(arena.nodes@, rest@) == model.stack.drop_first());
                        }
                        let mut expanded = Vec::new();
                        expanded.push(
                            EGoal::Lit { root: child_roots[0], depth: 1000, path: path.clone() },
                        );
                        expanded.push(EGoal::NafCut { level });
                        let mut rest = rest;
                        let ghost old_alts = c.alts@;
                        let alt = EAlt::Naf {
                            stack: c.stack,
                            fresh: c.fresh,
                            log_len: c.log.len(),
                            path,
                            inner: child_roots[0],
                            pruned: c.pruned,
                        };
                        proof {
                            push_alt(arena.nodes@, c.log@, c.alts@, alt);
                        }
                        c.alts.push(alt);
                        proof {
                            assert(goals_valid(arena.nodes@, expanded@));
                            assert(goals_levels(expanded@, c.alts.len() as nat));
                            levels_weaken(rest@, level as nat, c.alts.len() as nat);
                            reveal(ckc_spec::engine::trace_depth);
                            assert_seqs_equal!(goals_view(arena.nodes@, expanded@) == seq![
                                TGoal::Lit(terms[0], ckc_spec::engine::trace_depth(), path_view(path@)),
                                TGoal::NafCut(level as nat)]);
                            goals_concat(arena.nodes@, expanded@, rest@, c.alts.len() as nat);
                            assert(alts_view(arena.nodes@, c.log@, c.alts@) == model.alts.push(
                                TAlt::Naf {
                                    stack: model.stack.drop_first(),
                                    fresh: model.fresh,
                                    log: model.log,
                                    path: path_view(path@),
                                    inner: terms[0],
                                    pruned: model.pruned,
                                },
                            ));
                        }
                        expanded.append(&mut rest);
                        c.stack = expanded;
                        c.pruned = false;
                        return EStep::Next(c);
                    }
                    proof {
                        assert(false);
                    }
                    EStep::Limit
                },
                _ => fail(arena, c, Ghost(db.len())),
            }
        },
    }
}

fn step(input: ETermArena, db: &Vec<EClause>, initial: ECfg) -> (out: (ETermArena, EStep))
    requires
        arena_ok(&input),
        db_valid(input.nodes@, db@),
        cfg_valid(input.nodes@, db.len() as nat, &initial),
    ensures
        arena_ok(&out.0),
        input.nodes@.is_prefix_of(out.0.nodes@),
        step_valid(out.0.nodes@, db.len() as nat, &out.1),
        step_view(out.0.nodes@, &out.1) == tstep(
            db_view(input.nodes@, db@),
            cfg_view(input.nodes@, &initial),
        ),
{
    let ghost before = cfg_view(input.nodes@, &initial);
    let k = crate::k3_select::tselected_exec(&input, &initial.stack);
    if k > 0 {
        proof {
            let erased = before.stack.map_values(|g: TGoal| erase(g));
            crate::k2_select::selected_unfold(erased);
            crate::k2_select::selected_facts(erased);
            assert(erased[k as int] == erase(before.stack[k as int]));
        }
        return step_selected(input, db, initial, k);
    }
    match predicate_parts(&input, &initial, Ghost(db.len() as nat)) {
        None => {
            let result = simple(&input, initial, Ghost(db_view(input.nodes@, db@)));
            (input, result)
        },
        Some((name, args, depth, path)) => {
            let rest = clone_goals(&input, &initial.stack, 1, Ghost(initial.alts.len() as nat));
            let from = initial.ci;
            let pre: Vec<EGoal> = Vec::new();
            proof {
                assert(goals_view(input.nodes@, pre@) =~= before.stack.take(0));
            }
            if crate::compare_impl::is_arith_exec(&name, args.len()) {
                proof {
                    reveal(tstep);
                    assert(goals_view(input.nodes@, rest@) =~= before.stack.skip(1));
                }
                return arith_enter(
                    input,
                    initial,
                    &name,
                    &args,
                    pre,
                    rest,
                    path,
                    Ghost(db.len() as nat),
                );
            }
            if crate::compare_impl::is_builtin_exec(&name, args.len()) {
                if crate::compare_impl::holds_exec(&input, &name, &args) {
                    let root = match &initial.stack[0] {
                        EGoal::Lit { root, .. } => *root,
                        EGoal::NafCut { .. } => {
                            proof {
                                assert(false);
                            }
                            0
                        },
                    };
                    proof {
                        assert(goal_valid(input.nodes@, &initial.stack@[0]));
                        assert(before.stack[0] == goal_view(input.nodes@, &initial.stack@[0]));
                        assert(input@[root as int] == Term::Comp(
                            name@,
                            root_terms(input.nodes@, args@),
                        ));
                        assert(before.stack.skip(1) =~= before.stack.drop_first());
                        assert(before.stack.take(0) + before.stack.drop_first()
                            =~= before.stack.drop_first());
                    }
                    let result = enter_builtin(
                        &input,
                        initial,
                        rest,
                        root,
                        path,
                        Ghost(db.len() as nat),
                    );
                    (input, result)
                } else {
                    let result = fail(&input, initial, Ghost(db.len() as nat));
                    (input, result)
                }
            } else {
                call(input, db, initial, name, args, depth, pre, rest, from, path)
            }
        },
    }
}

// A goal selected past the front (contract eso D4): a predicate literal is
// resolved in place, a conjunction splits in place; paths ride along.
#[verifier::rlimit(5000)]
fn step_selected(input: ETermArena, db: &Vec<EClause>, initial: ECfg, k: usize) -> (out: (
    ETermArena,
    EStep,
))
    requires
        arena_ok(&input),
        db_valid(input.nodes@, db@),
        cfg_valid(input.nodes@, db.len() as nat, &initial),
        0 < k < initial.stack.len(),
        k == tselected(cfg_view(input.nodes@, &initial).stack),
        crate::k2_select::run_goal(erase(cfg_view(input.nodes@, &initial).stack[k as int])),
    ensures
        arena_ok(&out.0),
        input.nodes@.is_prefix_of(out.0.nodes@),
        step_valid(out.0.nodes@, db.len() as nat, &out.1),
        step_view(out.0.nodes@, &out.1) == tstep(
            db_view(input.nodes@, db@),
            cfg_view(input.nodes@, &initial),
        ),
{
    let ghost before = cfg_view(input.nodes@, &initial);
    let ghost nodes = input.nodes@;
    let level = initial.alts.len();
    proof {
        assert(goal_valid(nodes, &initial.stack@[k as int]));
        assert(goal_level(&initial.stack@[k as int], level as nat));
    }
    let mut pre = clone_goals(&input, &initial.stack, 0, Ghost(level as nat));
    let ghost whole = pre@;
    pre.truncate(k);
    let rest = clone_goals(&input, &initial.stack, k + 1, Ghost(level as nat));
    proof {
        assert(initial.stack@.skip(0) =~= initial.stack@);
        assert(goals_view(nodes, whole).len() == whole.len());
        assert(goals_view(nodes, initial.stack@).len() == initial.stack@.len());
        assert(whole.len() == initial.stack@.len());
        assert(pre@ =~= whole.subrange(0, k as int));
        assert forall|i: int| 0 <= i < pre@.len() implies {
            &&& goal_valid(nodes, &pre@[i])
            &&& goal_level(&pre@[i], level as nat)
        } by {
            assert(pre@[i] == whole[i]);
            assert(goal_valid(nodes, &whole[i]));
            assert(goal_level(&whole[i], level as nat));
        }
        assert(goals_view(nodes, pre@) =~= before.stack.take(k as int));
        assert(goals_view(nodes, rest@) =~= before.stack.skip(k as int + 1));
    }
    let (root, depth, path) = match &initial.stack[k] {
        EGoal::Lit { root, depth, path } => (*root, *depth, path.clone()),
        EGoal::NafCut { .. } => {
            proof {
                assert(false);
            }
            (0, 0, Vec::new())
        },
    };
    proof {
        assert(before.stack[k as int] == TGoal::Lit(
            nodes[root as int].term@,
            depth as nat,
            path_view(path@),
        ));
    }
    if depth == 0 {
        let mut c = initial;
        c.pruned = true;
        let result = fail(&input, c, Ghost(db.len() as nat));
        return (input, result);
    }
    proof {
        assert(node_ok(nodes, root as int));
    }
    let (name, args) = match &input.nodes[root].kind {
        ENodeKind::Comp { name, child_roots, .. } => {
            proof {
                crate::k2_engine::node_comp_model(nodes, root as int, name@, child_roots@);
            }
            (name.clone(), child_roots.clone())
        },
        _ => {
            proof {
                assert(false);
            }
            (Vec::new(), Vec::new())
        },
    };
    proof {
        reveal(crate::k2_term::child_terms);
        assert(root_terms(nodes, args@) == crate::k2_term::child_terms(nodes, args@));
        assert(nodes[root as int].term@ == Term::Comp(name@, root_terms(nodes, args@)));
    }
    let comma: &[u8] = b",";
    proof {
        reveal_byteslit(b",");
        reveal_strlit(",");
        reveal(ckc_spec::v1text::ascii);
        reveal(ckc_spec::engine::comma_name);
        assert(comma@ == ckc_spec::engine::comma_name());
    }
    if args.len() == 2 && crate::k3_front::bytes_equal(&name, comma) {
        let mut c = initial;
        let mut next = pre;
        next.push(EGoal::Lit { root: args[0], depth, path: path.clone() });
        next.push(EGoal::Lit { root: args[1], depth, path });
        let mut tail = rest;
        proof {
            assert(goals_valid(nodes, next@));
            assert(goals_levels(next@, level as nat));
            goals_concat(nodes, next@, tail@, level as nat);
        }
        next.append(&mut tail);
        c.stack = next;
        proof {
            assert(goals_view(nodes, c.stack@) =~= before.stack.take(k as int) + seq![
                TGoal::Lit(nodes[args@[0] as int].term@, depth as nat, path_view(path@)),
                TGoal::Lit(nodes[args@[1] as int].term@, depth as nat, path_view(path@)),
            ] + before.stack.skip(k as int + 1));
        }
        return (input, EStep::Next(c));
    }
    if crate::compare_impl::is_arith_exec(&name, args.len()) {
        return arith_enter(input, initial, &name, &args, pre, rest, path, Ghost(db.len() as nat));
    }
    if crate::compare_impl::is_builtin_exec(&name, args.len()) {
        if crate::compare_impl::holds_exec(&input, &name, &args) {
            let mut tail = rest;
            proof {
                goals_concat(nodes, pre@, tail@, level as nat);
            }
            pre.append(&mut tail);
            proof {
                assert(goals_view(nodes, pre@) =~= before.stack.take(k as int) + before.stack.skip(
                    k as int + 1,
                ));
            }
            let result = enter_builtin(&input, initial, pre, root, path, Ghost(db.len() as nat));
            return (input, result);
        }
        let result = fail(&input, initial, Ghost(db.len() as nat));
        return (input, result);
    }
    let from = initial.ci;
    call(input, db, initial, name, args, depth, pre, rest, from, path)
}

fn run_inner(input: ETermArena, db: &Vec<EClause>, initial: ECfg, fuel: usize) -> (out: (
    ETermArena,
    (EOut, usize),
))
    requires
        arena_ok(&input),
        db_valid(input.nodes@, db@),
        cfg_valid(input.nodes@, db.len() as nat, &initial),
    ensures
        arena_ok(&out.0),
        input.nodes@.is_prefix_of(out.0.nodes@),
        out_valid(out.0.nodes@, db.len() as nat, &out.1.0),
        (out_view(out.0.nodes@, &out.1.0), out.1.1 as nat) == trun(
            db_view(input.nodes@, db@),
            cfg_view(input.nodes@, &initial),
            fuel as nat,
        ),
        out.1.1 <= fuel,
{
    hide(tstep);
    hide(tcall);
    let ghost base = input.nodes@;
    let ghost program = db_view(base, db@);
    let ghost start = cfg_view(base, &initial);
    let mut arena = input;
    let mut current = initial;
    let mut left = fuel;
    while left > 0
        invariant
            base == input.nodes@,
            program == db_view(base, db@),
            start == cfg_view(base, &initial),
            arena_ok(&arena),
            base.is_prefix_of(arena.nodes@),
            db_valid(base, db@),
            db_valid(arena.nodes@, db@),
            db_view(arena.nodes@, db@) == program,
            cfg_valid(arena.nodes@, db.len() as nat, &current),
            left <= fuel,
            trun(program, start, fuel as nat) == trun(
                program,
                cfg_view(arena.nodes@, &current),
                left as nat,
            ),
        decreases left,
    {
        let ghost before_nodes = arena.nodes@;
        let ghost before = cfg_view(before_nodes, &current);
        let ghost old_left = left as nat;
        let (next_arena, next) = step(arena, db, current);
        arena = next_arena;
        proof {
            crate::k2_engine::db_models_prefix(before_nodes, arena.nodes@, db@);
            assert(base.is_prefix_of(arena.nodes@));
            reveal(trun);
        }
        left -= 1;
        match next {
            EStep::Next(c) => {
                current = c;
            },
            EStep::Sol(log) => return (arena, (EOut::Proved(log), left)),
            EStep::Done(complete) => return (arena, (EOut::Failed(complete), left)),
            EStep::Limit => return (arena, (EOut::Limit, left)),
        }
    }
    proof {
        reveal(trun);
    }
    (arena, (EOut::Limit, left))
}

pub fn run(arena: &mut ETermArena, db: &Vec<EClause>, initial: ECfg, fuel: usize) -> (out: (
    EOut,
    usize,
))
    requires
        arena_ok(old(arena)),
        db_valid(old(arena).nodes@, db@),
        cfg_valid(old(arena).nodes@, db.len() as nat, &initial),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out_valid(final(arena).nodes@, db.len() as nat, &out.0),
        (out_view(final(arena).nodes@, &out.0), out.1 as nat) == trun(
            db_view(old(arena).nodes@, db@),
            cfg_view(old(arena).nodes@, &initial),
            fuel as nat,
        ),
        out.1 <= fuel,
{
    let mut owned = crate::k2_reject::empty_arena();
    core::mem::swap(arena, &mut owned);
    let (mut owned, result) = run_inner(owned, db, initial, fuel);
    core::mem::swap(arena, &mut owned);
    result
}

pub fn roots(arena: &mut ETermArena, goals: &Vec<usize>, Ghost(db_len): Ghost<nat>) -> (out: ECfg)
    requires
        arena_ok(old(arena)),
        roots_valid(old(arena).nodes@, goals@),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        cfg_valid(final(arena).nodes@, db_len, &out),
        cfg_view(final(arena).nodes@, &out) == roots_cfg(root_terms(old(arena).nodes@, goals@)),
{
    let ghost base = arena.nodes@;
    let fresh = crate::k3_adapter::fresh_for_roots(arena, goals);
    proof {
        crate::k2_engine::roots_models_prefix(base, arena.nodes@, goals@);
    }
    let mut stack = Vec::new();
    let mut i = 0usize;
    while i < goals.len()
        invariant
            base == old(arena).nodes@,
            arena_ok(arena),
            base.is_prefix_of(arena.nodes@),
            roots_valid(arena.nodes@, goals@),
            root_terms(base, goals@) == root_terms(arena.nodes@, goals@),
            fresh <= arena.nodes.len(),
            fresh as nat == ckc_spec::engine::nvars_all(root_terms(base, goals@)),
            i <= goals.len(),
            stack.len() == i,
            goals_valid(arena.nodes@, stack@),
            goals_levels(stack@, 0),
            goals_view(arena.nodes@, stack@) == roots_cfg(root_terms(base, goals@)).stack.take(
                i as int,
            ),
        decreases goals.len() - i,
    {
        let mut path = Vec::new();
        path.push(i);
        let next = EGoal::Lit { root: goals[i], depth: 1000, path };
        proof {
            reveal(ckc_spec::engine::trace_depth);
            assert_seqs_equal!(path_view(path@) == seq![i as nat]);
            assert(goal_view(arena.nodes@, &next) == TGoal::Lit(
                root_terms(base, goals@)[i as int],
                ckc_spec::engine::trace_depth(),
                seq![i as nat],
            ));
        }
        let ghost before = stack@;
        stack.push(next);
        proof {
            reveal(ckc_spec::engine::trace_depth);
            assert forall|j: int| 0 <= j < stack.len() implies {
                &&& goal_valid(arena.nodes@, &stack@[j])
                &&& goal_level(&stack@[j], 0)
            } by {
                if j < before.len() {
                    assert(stack@[j] == before[j]);
                }
            }
            assert_seqs_equal!(goals_view(arena.nodes@, stack@) == roots_cfg(root_terms(base, goals@)).stack.take(i as int + 1), j => {
                if j < before.len() {
                    assert(stack@[j] == before[j]);
                    assert(goals_view(arena.nodes@, before)[j] == roots_cfg(root_terms(base, goals@)).stack[j]);
                }
            });
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(roots_cfg(root_terms(base, goals@)).stack.take(i as int) == roots_cfg(root_terms(base, goals@)).stack);
    }
    let out = ECfg { stack, alts: Vec::new(), fresh, ci: 0, log: Vec::new(), pruned: false };
    proof {
        assert_seqs_equal!(alts_view(arena.nodes@, out.log@, out.alts@) == Seq::empty());
        assert_seqs_equal!(log_view(arena.nodes@, out.log@) == Seq::empty());
    }
    out
}

} // verus!
