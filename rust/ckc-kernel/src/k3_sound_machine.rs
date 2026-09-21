use super::body::*;
use super::control::*;
use super::goals::*;
use super::graph::*;
use super::record::*;
use super::shapes::*;
use super::state::*;
use super::*;

verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub open spec fn continued(db: Seq<DocClause>, roots: Seq<Term>, step: TStep) -> bool {
    match step {
        TStep::Next(c) => exists|aux: Aux| #[trigger] aux_valid(db, roots, c, aux),
        _ => true,
    }
}

pub proof fn continued_next(db: Seq<DocClause>, roots: Seq<Term>, c: TCfg, aux: Aux)
    requires
        aux_valid(db, roots, c, aux),
    ensures
        continued(db, roots, TStep::Next(c)),
{
    assert(exists|a: Aux| #[trigger] aux_valid(db, roots, c, a));
    assert(TStep::Next(c) is Next);
    assert(TStep::Next(c)->Next_0 == c);
    reveal(continued);
}

pub proof fn tfail_preserves(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    c: TCfg,
    saved: Seq<Option<Cert>>,
)
    requires
        saved_valid(db, roots, c.alts, saved),
    ensures
        continued(db, roots, tfail(c)),
        !(tfail(c) is Sol),
{
    if c.alts.len() > 0 {
        let level = (c.alts.len() - 1) as nat;
        let a = c.alts.last();
        let state = saved.last();
        let alts = c.alts.drop_last();
        let tail = saved.drop_last();
        assert(alt_valid(db, roots, a, state, level));
        saved_prefix(db, roots, c.alts, saved, level);
        assert(c.alts.take(level as int) =~= alts);
        assert(saved.take(level as int) =~= tail);
        match a {
            TAlt::Cl { stack, fresh, ci, log } => {
                let nc = TCfg { stack, fresh, ci, log, alts, pruned: c.pruned };
                let na = Aux { current: state, saved: tail };
                assert(snapshot(nc) == alt_snapshot(a));
                assert(aux_valid(db, roots, nc, na));
                continued_next(db, roots, nc, na);
                assert(tfail(c) == TStep::Next(nc));
            },
            TAlt::Naf { stack, fresh, log, path, inner, pruned } => {
                if !c.pruned {
                    let v = alt_snapshot(a);
                    assert(v.stack.drop_first() =~= stack);
                    cuts_tail(v.stack, level);
                    in_naf_tail(v.stack);
                    let log2 = if in_naf(stack) {
                        log
                    } else {
                        log.push((path, TEv::Naf(inner)))
                    };
                    let nc = TCfg { stack, fresh, ci: 0, log: log2, alts, pruned };
                    match state {
                        Option::Some(cert) => {
                            outer_no_naf(db, roots, v, cert);
                            assert(!in_naf(stack));
                            record_naf(db, roots, v, cert, inner, 1, path, stack);
                            let after = Cert { theta: cert.theta, offsets: cert.offsets.push(0) };
                            let na = Aux { current: Option::Some(after), saved: tail };
                            assert(aux_valid(db, roots, nc, na));
                            continued_next(db, roots, nc, na);
                        },
                        Option::None => {
                            assert(in_naf(stack));
                            let na = Aux { current: Option::None, saved: tail };
                            assert(aux_valid(db, roots, nc, na));
                            continued_next(db, roots, nc, na);
                        },
                    }
                    assert(tfail(c) == TStep::Next(nc));
                }
            },
        }
    }
}

pub proof fn tcall_preserves(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    c: TCfg,
    aux: Aux,
    name: Seq<u8>,
    args: Seq<Term>,
    depth: nat,
    rest: Seq<TGoal>,
    ci: nat,
    path: Seq<nat>,
)
    requires
        bodies_wf(db),
        aux_valid(db, roots, c, aux),
        c.stack == seq![TGoal::Lit(Term::Comp(name, args), depth, path)] + rest,
    ensures
        continued(db, roots, tcall(db, c, name, args, depth, rest, ci, path)),
        !(tcall(db, c, name, args, depth, rest, ci, path) is Sol),
    decreases db.len() - ci,
{
    hide(tunify);
    next_match_bound(db, name, args.len(), ci);
    head_match(db, name, args.len(), ci);
    match next_match(db, name, args.len(), ci) {
        Option::None => {
            tfail_preserves(db, roots, c, aux.saved);
        },
        Option::Some(m) => {
            let cl = db[m as int];
            let body = tbody_goals(cl.body, c.fresh, (depth - 1) as nat, path);
            let stack2 = body + rest;
            let pairs = zip(args, args_of(shift(cl.head, c.fresh)));
            match tunify(pairs, stack2) {
                TUni::Fail => {
                    tcall_preserves(db, roots, c, aux, name, args, depth, rest, m + 1, path);
                },
                TUni::Out => {},
                TUni::Ok(out) => {
                    let a = TAlt::Cl { stack: c.stack, fresh: c.fresh, ci: m + 1, log: c.log };
                    assert(alt_snapshot(a) == snapshot(c));
                    assert(alt_valid(db, roots, a, aux.current, c.alts.len()));
                    saved_push(db, roots, c.alts, aux.saved, a, aux.current);
                    let saved = aux.saved.push(aux.current);
                    let alts = c.alts.push(a);
                    let log = if in_naf(c.stack) {
                        c.log
                    } else {
                        c.log.push((path, TEv::Clause(m)))
                    };
                    let nc = TCfg {
                        stack: out,
                        fresh: c.fresh + clause_nvars(cl),
                        ci: 0,
                        log,
                        alts,
                        pruned: c.pruned,
                    };
                    assert(c.stack.drop_first() =~= rest);
                    cuts_tail(c.stack, c.alts.len());
                    cuts_mono(rest, c.alts.len(), alts.len());
                    tbody_cuts(cl.body, c.fresh, (depth - 1) as nat, path, alts.len());
                    cuts_add(body, rest, alts.len());
                    tunify_frames(pairs, stack2);
                    cuts_frames(stack2, out, alts.len());
                    match aux.current {
                        Option::Some(cert) => {
                            outer_no_naf(db, roots, snapshot(c), cert);
                            record_clause(
                                db,
                                roots,
                                snapshot(c),
                                cert,
                                name,
                                args,
                                depth,
                                path,
                                rest,
                                m,
                                out,
                            );
                            let after = choose|after: Cert| #[trigger]
                                outer_valid(
                                    db,
                                    roots,
                                    Snapshot {
                                        stack: out,
                                        fresh: c.fresh + clause_nvars(cl),
                                        log: c.log.push((path, TEv::Clause(m))),
                                    },
                                    after,
                                );
                            let na = Aux { current: Option::Some(after), saved };
                            assert(aux_valid(db, roots, nc, na));
                            continued_next(db, roots, nc, na);
                        },
                        Option::None => {
                            in_naf_tail(c.stack);
                            in_naf_suffix(body, rest);
                            in_naf_frames(stack2, out);
                            let na = Aux { current: Option::None, saved };
                            assert(aux_valid(db, roots, nc, na));
                            continued_next(db, roots, nc, na);
                        },
                    }
                    assert(tcall(db, c, name, args, depth, rest, ci, path) == TStep::Next(nc));
                },
            }
        },
    }
}

pub proof fn tstep_preserves(db: Seq<DocClause>, roots: Seq<Term>, c: TCfg, aux: Aux)
    requires
        bodies_wf(db),
        aux_valid(db, roots, c, aux),
    ensures
        continued(db, roots, tstep(db, c)),
        tstep(db, c) matches TStep::Sol(log) ==> c.stack.len() == 0 && log == c.log,
{
    if c.stack.len() > 0 {
        let rest = c.stack.drop_first();
        assert(seq![c.stack[0]] + rest =~= c.stack);
        match c.stack[0] {
            TGoal::NafCut(lvl) => {
                assert(goal_frame(c.stack[0]).0);
                assert(lvl < c.alts.len());
                match c.alts[lvl as int] {
                    TAlt::Naf { pruned, .. } => {
                        saved_prefix(db, roots, c.alts, aux.saved, lvl);
                        let cut = TCfg { alts: c.alts.take(lvl as int), pruned, ..c };
                        tfail_preserves(db, roots, cut, aux.saved.take(lvl as int));
                    },
                    _ => {},
                }
            },
            TGoal::Lit(g, depth, path) => {
                if depth == 0 {
                    tfail_preserves(db, roots, TCfg { pruned: true, ..c }, aux.saved);
                } else {
                    match g {
                        Term::Comp(name, args) => {
                            if name == comma_name() && args.len() == 2 {
                                let prefix = seq![
                                    TGoal::Lit(args[0], depth, path),
                                    TGoal::Lit(args[1], depth, path),
                                ];
                                let nc = TCfg { stack: prefix + rest, ..c };
                                match aux.current {
                                    Option::Some(cert) => {
                                        outer_literal_shape(db, roots, snapshot(c), cert, 0);
                                        assert(args.len() != 2);
                                    },
                                    Option::None => {
                                        in_naf_tail(c.stack);
                                        in_naf_suffix(prefix, rest);
                                        cuts_tail(c.stack, c.alts.len());
                                        assert(cuts_below(prefix, c.alts.len()));
                                        cuts_add(prefix, rest, c.alts.len());
                                        let na = Aux { current: Option::None, saved: aux.saved };
                                        assert(aux_valid(db, roots, nc, na));
                                        continued_next(db, roots, nc, na);
                                    },
                                }
                                assert(tstep(db, c) == TStep::Next(nc));
                            } else if name == naf_name() && args.len() == 1 {
                                let inner = args[0];
                                assert(args =~= seq![inner]);
                                let a = TAlt::Naf {
                                    stack: rest,
                                    fresh: c.fresh,
                                    log: c.log,
                                    path,
                                    inner,
                                    pruned: c.pruned,
                                };
                                naf_alternative(db, roots, c, aux, inner, depth, path, rest);
                                saved_push(db, roots, c.alts, aux.saved, a, aux.current);
                                let alts = c.alts.push(a);
                                let prefix = seq![
                                    TGoal::Lit(inner, trace_depth(), path),
                                    TGoal::NafCut(c.alts.len()),
                                ];
                                let nc = TCfg { stack: prefix + rest, alts, pruned: false, ..c };
                                cuts_tail(c.stack, c.alts.len());
                                cuts_mono(rest, c.alts.len(), alts.len());
                                assert(cuts_below(prefix, alts.len()));
                                cuts_add(prefix, rest, alts.len());
                                assert(nc.stack[1] is NafCut);
                                assert(in_naf(nc.stack));
                                let na = Aux {
                                    current: Option::None,
                                    saved: aux.saved.push(aux.current),
                                };
                                assert(aux_valid(db, roots, nc, na));
                                continued_next(db, roots, nc, na);
                                assert(tstep(db, c) == TStep::Next(nc));
                            } else {
                                tcall_preserves(
                                    db,
                                    roots,
                                    c,
                                    aux,
                                    name,
                                    args,
                                    depth,
                                    rest,
                                    c.ci,
                                    path,
                                );
                            }
                        },
                        _ => {
                            tfail_preserves(db, roots, c, aux.saved);
                        },
                    }
                }
            },
        }
    }
}

pub open spec fn complete_cert(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    log: Seq<(Seq<nat>, TEv)>,
    cert: Cert,
) -> bool {
    cert_log(db, roots, log, cert.offsets, cert.theta) && complete_log(db, roots, log)
}

pub open spec fn has_weak_certificate(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    log: Seq<(Seq<nat>, TEv)>,
) -> bool {
    exists|cert: Cert| #[trigger] complete_cert(db, roots, log, cert)
}

pub proof fn trun_weak(db: Seq<DocClause>, roots: Seq<Term>, c: TCfg, aux: Aux, fuel: nat)
    requires
        bodies_wf(db),
        aux_valid(db, roots, c, aux),
    ensures
        trun(db, c, fuel).0 matches TOut::Proved(log) ==> has_weak_certificate(db, roots, log),
    decreases fuel,
{
    if fuel > 0 {
        tstep_preserves(db, roots, c, aux);
        match tstep(db, c) {
            TStep::Next(nc) => {
                assert(continued(db, roots, TStep::Next(nc)));
                let na = choose|na: Aux| #[trigger] aux_valid(db, roots, nc, na);
                trun_weak(db, roots, nc, na, (fuel - 1) as nat);
            },
            TStep::Sol(log) => {
                assert(c.stack.len() == 0 && log == c.log);
                terminal_certificate(db, roots, c, aux);
                let cert = aux.current.unwrap();
                assert(complete_cert(db, roots, log, cert));
                assert(has_weak_certificate(db, roots, log));
            },
            _ => {},
        }
    }
}

pub proof fn roots_weak(db: Seq<DocClause>, goal: Term)
    requires
        bodies_wf(db),
        ckc_spec::answers::goal_walk(goal) is None,
    ensures
        trun(db, roots_cfg(conj_leaves(goal)), trace_inf()).0 matches TOut::Proved(log)
            ==> has_weak_certificate(db, conj_leaves(goal), log),
{
    let roots = conj_leaves(goal);
    root_shapes(goal);
    initial_valid(db, roots);
    trun_weak(db, roots, roots_cfg(roots), initial_aux(), trace_inf());
}

} // verus!
