use crate::k2_term::{ENodeKind, ETermArena};
use crate::mc_model::EMcModel;
use ckc_spec::term::Term;
use vstd::prelude::*;

verus! {

// Contract mc D6: goals evaluated over M by joins. A built-in admits at most one extension of
// the valuation (its inputs bound by left-to-right safety); a schema literal one per model fact.
proof fn arith_not_compare(name: Seq<u8>)
    requires
        ckc_spec::arith::is_arith(name, 6),
    ensures
        name != ckc_spec::compare::satisfies_name(),
{
    reveal_strlit("guideline_product");
    reveal_strlit("guideline_quotient");
    reveal_strlit("guideline_satisfies");
    assert(ckc_spec::compare::satisfies_name()[10] == 0x73u8);
    if name == ckc_spec::arith::product_name() {
        assert(name[10] == 0x70u8);
    } else {
        assert(name[10] == 0x71u8);
    }
}

// Arithmetic results read the four inputs alone.
proof fn arith_inputs(f: Seq<u8>, a: Seq<Term>, b: Seq<Term>)
    requires
        a.len() == 6,
        b.len() == 6,
        a[0] == b[0],
        a[1] == b[1],
        a[2] == b[2],
        a[3] == b[3],
    ensures
        ckc_spec::arith::arith_result(Term::Comp(f, a)) == ckc_spec::arith::arith_result(
            Term::Comp(f, b),
        ),
{
    reveal(ckc_spec::arith::arith_result);
}

// The instance of a built-in g under th: its arguments, instantiated where every variable is bound.
pub open spec fn bound_in(t: Term, th: Map<nat, Term>) -> bool {
    forall|x: nat| #[trigger] ckc_spec::engine::occurs(x, t) ==> th.contains_key(x)
}

// The unique extension of th under which built-in goal arena@[r] holds, if any (most general).
pub fn builtin_step(
    arena: &mut ETermArena,
    r: usize,
    sub: &Vec<Option<usize>>,
    Ghost(th): Ghost<Map<nat, Term>>,
    Ghost(m): Ghost<ISet<Term>>,
) -> (out: Option<(Vec<Option<usize>>, Ghost<Map<nat, Term>>)>)
    requires
        crate::k2_term::arena_ok(old(arena)),
        r < old(arena).nodes@.len(),
        ckc_spec::model::builtin(old(arena)@[r as int]),
        ckc_spec::model::lit_safe(old(arena)@[r as int], th.dom()),
        crate::mc_subst::sub_ok(old(arena).nodes@, sub@, th),
        forall|x: nat| #[trigger]
            ckc_spec::engine::occurs(x, old(arena)@[r as int]) ==> x < sub@.len(),
    ensures
        crate::k2_term::arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Some((s2, th2)) ==> {
            &&& crate::mc_subst::sub_ok(final(arena).nodes@, s2@, th2@)
            &&& s2@.len() == sub@.len()
            &&& crate::mc_subst::extends(th2@, th)
            &&& th2@.dom() == th.dom().union(ckc_spec::model::binds(old(arena)@[r as int]))
            &&& crate::mc_model::goal_ok(m, old(arena)@[r as int], th2@)
        },
        forall|t3: Map<nat, Term>|
            crate::mc_subst::extends(t3, th) && crate::mc_model::goal_ok(
                m,
                old(arena)@[r as int],
                t3,
            ) ==> (out matches Some((s2, th2)) && crate::mc_subst::extends(t3, th2@)),
{
    let ghost g = arena@[r as int];
    let ghost origin = arena.nodes@;
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, r as int));
        assert(origin.is_prefix_of(origin)) by {
            assert(origin.take(origin.len() as int) =~= origin);
        }
        crate::mc_model::sub_vals_wf(arena, sub@, th);
    }
    let (name, kids) = match &arena.nodes[r].kind {
        ENodeKind::Comp { name, child_roots, .. } => (name.clone(), child_roots.clone()),
        _ => {
            proof {
                assert(false);
            }
            return None;
        },
    };
    let ghost args = crate::k2_term::child_terms(origin, kids@);
    proof {
        assert(g == Term::Comp(name@, args));
        assert(args.len() == kids@.len());
    }
    let arith = crate::compare_impl::is_arith_exec(&name, kids.len());
    let k_in = if arith {
        4
    } else {
        kids.len()
    };
    proof {
        // the inputs' variables are bound
        let ins = ckc_spec::model::builtin_inputs(g);
        assert(ins =~= args.subrange(0, k_in as int)) by {
            if arith {
                assert(args.take(4) =~= args.subrange(0, 4));
            } else {
                assert(args =~= args.subrange(0, args.len() as int));
            }
        }
        assert forall|q: int| 0 <= q < k_in implies #[trigger] bound_in(args[q], th) by {
            assert forall|x: nat| #[trigger]
                ckc_spec::engine::occurs(x, args[q]) implies th.contains_key(x) by {
                crate::mc_safe::vars_all_any(args.subrange(0, k_in as int), x);
                assert(args.subrange(0, k_in as int)[q] == args[q]);
                assert(ckc_spec::model::vars_all(ins).contains(x));
            }
        }
    }
    let mut inst: Vec<usize> = Vec::new();
    let mut q = 0usize;
    while q < k_in
        invariant
            crate::k2_term::arena_ok(arena),
            origin.is_prefix_of(arena.nodes@),
            crate::mc_subst::sub_ok(arena.nodes@, sub@, th),
            k_in <= kids@.len(),
            crate::k2_term::child_roots_valid(origin, kids@),
            args == crate::k2_term::child_terms(origin, kids@),
            forall|q2: int| 0 <= q2 < k_in ==> #[trigger] bound_in(args[q2], th),
            q <= k_in,
            inst@.len() == q,
            forall|q2: int|
                0 <= q2 < q ==> #[trigger] inst@[q2] < arena.nodes@.len()
                    && arena@[inst@[q2] as int] == ckc_spec::model::apply(args[q2], th),
        decreases k_in - q,
    {
        let ghost before = arena.nodes@;
        proof {
            assert(origin[kids@[q as int] as int] == before[kids@[q as int] as int]);
            assert(bound_in(args[q as int], th));
        }
        let b = crate::mc_subst::instantiate(arena, kids[q], sub, Ghost(th));
        proof {
            crate::k2_load::prefix_chain(origin, before, arena.nodes@);
            crate::mc_subst::sub_ok_prefix(before, arena.nodes@, sub@, th);
            assert forall|q2: int| 0 <= q2 < q implies #[trigger] inst@[q2] < arena.nodes@.len()
                && arena@[inst@[q2] as int] == ckc_spec::model::apply(args[q2], th) by {
                assert(before[inst@[q2] as int] == arena.nodes@[inst@[q2] as int]);
            }
        }
        inst.push(b);
        q += 1;
    }
    let ghost after_inst = arena.nodes@;
    proof {
        crate::mc_subst::apply_all_index(args, th);
    }
    if !arith {
        let ok = crate::compare_impl::holds_exec(arena, &name, &inst);
        proof {
            let ia = crate::k2_engine::root_terms(arena.nodes@, inst@);
            assert(ia =~= ckc_spec::model::apply_all(args, th));
            assert(ckc_spec::model::apply(g, th) == Term::Comp(name@, ia));
            crate::mc_safe::vars_comp(g, 0);
            assert(bound_in(g, th)) by {
                assert forall|x: nat| #[trigger]
                    ckc_spec::engine::occurs(x, g) implies th.contains_key(x) by {
                    crate::mc_safe::occurs_all_any(x, args);
                    let q2 = choose|q2: int|
                        0 <= q2 < args.len() && #[trigger] ckc_spec::engine::occurs(x, args[q2]);
                    assert(bound_in(args[q2], th));
                }
            }
            crate::mc_subst::apply_ground(g, th);
            crate::mc_model::apply_wf(g, th);
            crate::mc_proof::lemma_builtin_apply(g, th);
            assert(ckc_spec::model::binds(g) =~= Set::<nat>::empty()) by {
                assert(ckc_spec::model::builtin_outputs(g) =~= Seq::<Term>::empty());
            }
            assert(th.dom().union(ckc_spec::model::binds(g)) =~= th.dom());
            assert forall|t3: Map<nat, Term>|
                crate::mc_subst::extends(t3, th) implies ckc_spec::model::apply(g, t3)
                == ckc_spec::model::apply(g, th) by {
                crate::mc_subst::apply_agree(g, t3, th);
            }
        }
        if ok {
            proof {
                crate::mc_subst::sub_ok_prefix(after_inst, arena.nodes@, sub@, th);
            }
            return Some((sub.clone(), Ghost(th)));
        } else {
            return None;
        }
    }
    let ghost before_out = arena.nodes@;
    inst.push(kids[4]);
    inst.push(kids[5]);
    proof {
        assert(arena.nodes@[kids@[4] as int] == origin[kids@[4] as int]);
        assert(arena.nodes@[kids@[5] as int] == origin[kids@[5] as int]);
    }
    let res = crate::arith_impl::arith_result_exec(arena, &name, &inst);
    let ghost after_res = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(origin, before_out, after_res);
        crate::mc_subst::sub_ok_prefix(before_out, after_res, sub@, th);
        let ia = crate::k2_engine::root_terms(before_out, inst@);
        arith_not_compare(name@);
        assert forall|t3: Map<nat, Term>|
            crate::mc_subst::extends(t3, th) implies ckc_spec::arith::arith_result(
            ckc_spec::model::apply(g, t3),
        ) == ckc_spec::arith::arith_result(Term::Comp(name@, ia)) by {
            crate::mc_subst::apply_all_index(args, t3);
            assert forall|q2: int| 0 <= q2 < 4 implies ckc_spec::model::apply(args[q2], t3)
                == ia[q2] by {
                assert(bound_in(args[q2], th));
                crate::mc_subst::apply_agree(args[q2], t3, th);
            }
            arith_inputs(name@, ckc_spec::model::apply_all(args, t3), ia);
        }
    }
    let (s, n) = match res {
        Some(pair) => pair,
        None => {
            proof {
                assert forall|t3: Map<nat, Term>|
                    crate::mc_subst::extends(t3, th) && crate::mc_model::goal_ok(
                        m,
                        g,
                        t3,
                    ) implies false by {
                    crate::mc_proof::lemma_builtin_apply(g, t3);
                    reveal(ckc_spec::arith::arith_holds);
                }
            }
            return None;
        },
    };
    let ghost ia = crate::k2_engine::root_terms(before_out, inst@);
    let ghost sv = arena@[s as int];
    let ghost nv = arena@[n as int];
    proof {
        assert(ckc_spec::arith::arith_result(Term::Comp(name@, ia)) == Some((sv, nv)));
        assert(arena.nodes@[kids@[4] as int] == origin[kids@[4] as int]);
        assert(arena.nodes@[kids@[5] as int] == origin[kids@[5] as int]);
        assert(arena@[kids@[4] as int] == args[4]);
        assert(arena@[kids@[5] as int] == args[5]);
    }
    // a spelled result is ground; a non-ground one equals no ground instance
    if !crate::k2_walk::ground_root(arena, s) || !crate::k2_walk::ground_root(arena, n) {
        proof {
            assert forall|t3: Map<nat, Term>|
                crate::mc_subst::extends(t3, th) && crate::mc_model::goal_ok(
                    m,
                    g,
                    t3,
                ) implies false by {
                crate::mc_proof::lemma_builtin_apply(g, t3);
                reveal(ckc_spec::arith::arith_holds);
                crate::mc_subst::apply_all_index(args, t3);
                crate::mc_patient::ground_all_at(ckc_spec::model::apply_all(args, t3), 4);
                crate::mc_patient::ground_all_at(ckc_spec::model::apply_all(args, t3), 5);
            }
        }
        return None;
    }
    let mut s2 = sub.clone();
    proof {
        assert forall|x: nat| #[trigger]
            ckc_spec::engine::occurs(x, arena@[kids@[4] as int]) implies x < s2@.len() by {
            crate::mc_subst::occurs_in_arg(name@, args, 4, x);
        }
    }
    let (ok1, th1) = crate::mc_subst::match_term(arena, kids[4], s, &mut s2, Ghost(th));
    if !ok1 {
        proof {
            assert forall|t3: Map<nat, Term>|
                crate::mc_subst::extends(t3, th) && crate::mc_model::goal_ok(
                    m,
                    g,
                    t3,
                ) implies false by {
                crate::mc_proof::lemma_builtin_apply(g, t3);
                reveal(ckc_spec::arith::arith_holds);
                crate::mc_subst::apply_all_index(args, t3);
                assert(ckc_spec::model::apply(args[4], t3) == sv);
            }
        }
        return None;
    }
    proof {
        assert forall|x: nat| #[trigger]
            ckc_spec::engine::occurs(x, arena@[kids@[5] as int]) implies x < s2@.len() by {
            crate::mc_subst::occurs_in_arg(name@, args, 5, x);
        }
    }
    let (ok2, th2) = crate::mc_subst::match_term(arena, kids[5], n, &mut s2, th1);
    if !ok2 {
        proof {
            assert forall|t3: Map<nat, Term>|
                crate::mc_subst::extends(t3, th) && crate::mc_model::goal_ok(
                    m,
                    g,
                    t3,
                ) implies false by {
                crate::mc_proof::lemma_builtin_apply(g, t3);
                reveal(ckc_spec::arith::arith_holds);
                crate::mc_subst::apply_all_index(args, t3);
                assert(ckc_spec::model::apply(args[4], t3) == sv);
                assert(crate::mc_subst::extends(t3, th1@));
                assert(ckc_spec::model::apply(args[5], t3) == nv);
            }
        }
        return None;
    }
    proof {
        let t1 = th1@;
        let t2 = th2@;
        assert(crate::mc_subst::extends(t2, th)) by {
            assert forall|k: nat| #[trigger] th.contains_key(k) implies t2.contains_key(k) && t2[k]
                == th[k] by {
                assert(t1.contains_key(k));
            }
        }
        crate::mc_subst::apply_all_index(args, t2);
        assert forall|x: nat| #[trigger] ckc_spec::engine::occurs(x, g) implies t2.contains_key(
            x,
        ) by {
            crate::mc_safe::occurs_all_any(x, args);
            let q2 = choose|q2: int|
                0 <= q2 < args.len() && #[trigger] ckc_spec::engine::occurs(x, args[q2]);
            if q2 < 4 {
                assert(bound_in(args[q2], th));
                assert(th.contains_key(x));
            } else if q2 == 4 {
                assert(t1.contains_key(x));
            }
        }
        crate::mc_model::sub_vals_wf(arena, s2@, t2);
        assert forall|x: nat| #[trigger] ckc_spec::engine::occurs(x, g) implies t2.contains_key(x)
            && ckc_spec::term::ground(t2[x]) by {}
        crate::mc_subst::apply_ground(g, t2);
        crate::mc_model::apply_wf(g, t2);
        crate::mc_proof::lemma_builtin_apply(g, t2);
        reveal(ckc_spec::arith::arith_holds);
        assert(ckc_spec::model::apply(args[4], t2) == sv) by {
            assert forall|x: nat| #[trigger] ckc_spec::engine::occurs(x, args[4]) implies (
            t2.contains_key(x) == t1.contains_key(x) && (t2.contains_key(x) ==> t2[x]
                == t1[x])) by {
                assert(t1.contains_key(x));
            }
            crate::mc_subst::apply_agree(args[4], t2, t1);
        }
        assert(ckc_spec::model::apply(args[5], t2) == nv);
        assert(ckc_spec::arith::arith_result(ckc_spec::model::apply(g, t2)) == Some((sv, nv)));
        assert(ckc_spec::model::apply_all(args, t2)[4] == sv);
        assert(ckc_spec::model::apply_all(args, t2)[5] == nv);
        assert(ckc_spec::arith::arith_holds(ckc_spec::model::apply(g, t2)));
        assert(crate::mc_model::goal_ok(m, g, t2));
        assert(ckc_spec::model::binds(g) =~= ckc_spec::model::vars_all(args.skip(4))) by {
            assert(ckc_spec::model::builtin_outputs(g) == args.skip(4));
        }
        assert forall|x: nat| #[trigger]
            t2.dom().contains(x) == th.dom().union(ckc_spec::model::binds(g)).contains(x) by {
            crate::mc_safe::vars_all_any(args.skip(4), x);
            assert(args.skip(4).len() == 2);
            assert(args.skip(4)[0] == args[4]);
            assert(args.skip(4)[1] == args[5]);
            crate::mc_safe::vars_occurs(args[4], x);
            crate::mc_safe::vars_occurs(args[5], x);
        }
        assert(t2.dom() =~= th.dom().union(ckc_spec::model::binds(g)));
        assert forall|t3: Map<nat, Term>|
            crate::mc_subst::extends(t3, th) && crate::mc_model::goal_ok(
                m,
                g,
                t3,
            ) implies crate::mc_subst::extends(t3, t2) by {
            crate::mc_proof::lemma_builtin_apply(g, t3);
            crate::mc_subst::apply_all_index(args, t3);
            assert(ckc_spec::model::apply(args[4], t3) == sv);
            assert(crate::mc_subst::extends(t3, t1));
            assert(ckc_spec::model::apply(args[5], t3) == nv);
        }
    }
    Some((s2, th2))
}

// Some extension of th makes every goal of gs from k hold in M (sound + complete).
pub fn witness_from(
    arena: &mut ETermArena,
    roots: &Vec<usize>,
    k: usize,
    sub: &Vec<Option<usize>>,
    Ghost(th): Ghost<Map<nat, Term>>,
    m: &EMcModel,
) -> (out: bool)
    requires
        crate::k2_term::arena_ok(old(arena)),
        crate::k2_engine::roots_valid(old(arena).nodes@, roots@),
        k <= roots@.len(),
        crate::mc_model::model_ok(old(arena).nodes@, m),
        crate::mc_subst::sub_ok(old(arena).nodes@, sub@, th),
        ckc_spec::model::conj_safe(
            crate::k2_engine::root_terms(old(arena).nodes@, roots@).skip(k as int),
            th.dom(),
        ),
        forall|q: int, x: nat|
            0 <= q < roots@.len() && #[trigger] ckc_spec::engine::occurs(
                x,
                crate::k2_engine::root_terms(old(arena).nodes@, roots@)[q],
            ) ==> x < sub@.len(),
        forall|q: int|
            0 <= q < roots@.len() ==> (#[trigger] crate::k2_engine::root_terms(
                old(arena).nodes@,
                roots@,
            )[q]) is Comp,
    ensures
        crate::k2_term::arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out == exists|t3: Map<nat, Term>|
            crate::mc_subst::extends(t3, th) && crate::mc_model::goals_sat(
                m.ms@.to_iset(),
                crate::k2_engine::root_terms(old(arena).nodes@, roots@),
                k as int,
                t3,
            ),
    decreases roots@.len() - k,
{
    let ghost gs = crate::k2_engine::root_terms(arena.nodes@, roots@);
    let ghost mm = m.ms@.to_iset();
    let ghost origin = arena.nodes@;
    proof {
        assert(origin.is_prefix_of(origin)) by {
            assert(origin.take(origin.len() as int) =~= origin);
        }
    }
    if k == roots.len() {
        proof {
            assert(crate::mc_subst::extends(th, th));
            assert(crate::mc_model::goals_sat(mm, gs, k as int, th));
        }
        return true;
    }
    let k1 = k + 1;
    let r = roots[k];
    let ghost g = gs[k as int];
    proof {
        assert(g == arena@[r as int]);
        assert(g is Comp);
        assert(gs.skip(k as int)[0] == g);
        assert(gs.skip(k as int).drop_first() =~= gs.skip(k as int + 1));
        assert(ckc_spec::model::lit_safe(g, th.dom()));
        assert(ckc_spec::model::conj_safe(
            gs.skip(k as int + 1),
            th.dom().union(ckc_spec::model::binds(g)),
        ));
        assert forall|x: nat| #[trigger] ckc_spec::engine::occurs(x, arena@[r as int]) implies x
            < sub@.len() by {
            assert(ckc_spec::engine::occurs(x, gs[k as int]));
        }
    }
    let (builtin, _kids, _k_in) = crate::mc_safe::literal_parts(arena, r);
    if builtin {
        match builtin_step(arena, r, sub, Ghost(th), Ghost(mm)) {
            None => {
                proof {
                    assert forall|t3: Map<nat, Term>|
                        crate::mc_subst::extends(t3, th) && crate::mc_model::goals_sat(
                            mm,
                            gs,
                            k as int,
                            t3,
                        ) implies false by {
                        assert(crate::mc_model::goal_ok(mm, gs[k as int], t3));
                    }
                }
                return false;
            },
            Some((s2, th2)) => {
                let ghost before = arena.nodes@;
                proof {
                    crate::mc_model::model_ok_prefix(origin, before, m);
                    assert(crate::k2_engine::root_terms(before, roots@) =~= gs) by {
                        assert forall|q: int| 0 <= q < roots@.len() implies before[roots@[q] as int]
                            == origin[roots@[q] as int] by {}
                    }
                }
                let found = witness_from(arena, roots, k1, &s2, th2, m);
                proof {
                    crate::k2_load::prefix_chain(origin, before, arena.nodes@);
                    if found {
                        let t3 = choose|t3: Map<nat, Term>|
                            crate::mc_subst::extends(t3, th2@) && crate::mc_model::goals_sat(
                                mm,
                                gs,
                                k as int + 1,
                                t3,
                            );
                        assert(crate::mc_subst::extends(t3, th)) by {
                            assert forall|kk: nat| #[trigger]
                                th.contains_key(kk) implies t3.contains_key(kk) && t3[kk]
                                == th[kk] by {
                                assert(th2@.contains_key(kk));
                            }
                        }
                        assert(crate::mc_model::goal_ok(mm, g, t3)) by {
                            assert(crate::mc_model::goal_ok(mm, g, th2@));
                            assert forall|x: nat| #[trigger]
                                ckc_spec::engine::occurs(x, g) implies (t3.contains_key(x)
                                == th2@.contains_key(x) && (t3.contains_key(x) ==> t3[x]
                                == th2@[x])) by {
                                crate::mc_subst::ground_apply_binds(g, th2@, x);
                            }
                            crate::mc_subst::apply_agree(g, t3, th2@);
                        }
                        assert forall|q: int|
                            k <= q < gs.len() implies #[trigger] crate::mc_model::goal_ok(
                            mm,
                            gs[q],
                            t3,
                        ) by {
                            if q > k {
                                assert(crate::mc_model::goal_ok(mm, gs[q], t3));
                            }
                        }
                    } else {
                        assert forall|t3: Map<nat, Term>|
                            crate::mc_subst::extends(t3, th) && crate::mc_model::goals_sat(
                                mm,
                                gs,
                                k as int,
                                t3,
                            ) implies false by {
                            assert(crate::mc_model::goal_ok(mm, gs[k as int], t3));
                            assert(crate::mc_subst::extends(t3, th2@));
                            assert(crate::mc_model::goals_sat(mm, gs, k as int + 1, t3));
                        }
                    }
                }
                return found;
            },
        }
    }
    // A schema literal: one extension per model fact it matches.

    let mut i = 0usize;
    while i < m.facts.len()
        invariant
            crate::k2_term::arena_ok(arena),
            origin == old(arena).nodes@,
            origin.is_prefix_of(arena.nodes@),
            crate::mc_model::model_ok(arena.nodes@, m),
            crate::mc_model::model_ok(origin, m),
            crate::mc_subst::sub_ok(arena.nodes@, sub@, th),
            gs == crate::k2_engine::root_terms(origin, roots@),
            crate::k2_engine::roots_valid(origin, roots@),
            k < roots@.len(),
            k1 == k + 1,
            r == roots@[k as int],
            g == gs[k as int],
            g == origin[r as int].term@,
            g is Comp,
            !ckc_spec::model::builtin(g),
            mm == m.ms@.to_iset(),
            ckc_spec::model::conj_safe(
                gs.skip(k as int + 1),
                th.dom().union(ckc_spec::model::binds(g)),
            ),
            forall|q: int, x: nat|
                0 <= q < roots@.len() && #[trigger] ckc_spec::engine::occurs(x, gs[q]) ==> x
                    < sub@.len(),
            forall|q: int| 0 <= q < roots@.len() ==> (#[trigger] gs[q]) is Comp,
            i <= m.facts@.len(),
            forall|t3: Map<nat, Term>|
                crate::mc_subst::extends(t3, th) && crate::mc_model::goals_sat(mm, gs, k as int, t3)
                    ==> !exists|i2: int|
                    0 <= i2 < i && #[trigger] m.ms@[i2] == ckc_spec::model::apply(g, t3),
        decreases m.facts.len() - i,
    {
        let ghost before = arena.nodes@;
        let mut s2 = sub.clone();
        proof {
            assert(before[r as int] == origin[r as int]);
            assert(m.facts@[i as int] < before.len());
            assert forall|x: nat| #[trigger]
                ckc_spec::engine::occurs(x, before[r as int].term@) implies x < s2@.len() by {
                assert(ckc_spec::engine::occurs(x, gs[k as int]));
            }
        }
        let (ok, th2) = crate::mc_subst::match_term(arena, r, m.facts[i], &mut s2, Ghost(th));
        if ok {
            proof {
                assert(crate::k2_engine::root_terms(before, roots@) =~= gs) by {
                    assert forall|q: int| 0 <= q < roots@.len() implies before[roots@[q] as int]
                        == origin[roots@[q] as int] by {}
                }
                assert(th2@.dom() =~= th.dom().union(ckc_spec::model::binds(g))) by {
                    assert forall|x: nat|
                        th2@.dom().contains(x) == th.dom().union(
                            ckc_spec::model::binds(g),
                        ).contains(x) by {
                        crate::mc_safe::vars_occurs(g, x);
                    }
                }
            }
            let found = witness_from(arena, roots, k1, &s2, th2, m);
            proof {
                crate::k2_load::prefix_chain(origin, before, arena.nodes@);
                crate::mc_subst::sub_ok_prefix(before, arena.nodes@, sub@, th);
                crate::mc_model::model_ok_prefix(before, arena.nodes@, m);
            }
            if found {
                proof {
                    let t3 = choose|t3: Map<nat, Term>|
                        crate::mc_subst::extends(t3, th2@) && crate::mc_model::goals_sat(
                            mm,
                            gs,
                            k as int + 1,
                            t3,
                        );
                    let f = m.ms@[i as int];
                    assert(origin[m.facts@[i as int] as int].term@ == f);
                    assert(before[m.facts@[i as int] as int] == origin[m.facts@[i as int] as int]);
                    assert(ckc_spec::model::apply(g, th2@) == f);
                    assert forall|x: nat| #[trigger] ckc_spec::engine::occurs(x, g) implies (
                    t3.contains_key(x) == th2@.contains_key(x) && (t3.contains_key(x) ==> t3[x]
                        == th2@[x])) by {}
                    crate::mc_subst::apply_agree(g, t3, th2@);
                    crate::mc_model::iset_of_seq(m.ms@, f);
                    crate::mc_subst::builtin_apply_iff(g, t3);
                    assert(crate::k2_term::node_ok(arena.nodes@, m.facts@[i as int] as int));
                    assert(arena.nodes@[m.facts@[i as int] as int]
                        == origin[m.facts@[i as int] as int]);
                    assert(crate::mc_model::goal_ok(mm, g, t3));
                    assert(crate::mc_subst::extends(t3, th)) by {
                        assert forall|kk: nat| #[trigger]
                            th.contains_key(kk) implies t3.contains_key(kk) && t3[kk] == th[kk] by {
                            assert(th2@.contains_key(kk));
                        }
                    }
                    assert forall|q: int|
                        k <= q < gs.len() implies #[trigger] crate::mc_model::goal_ok(
                        mm,
                        gs[q],
                        t3,
                    ) by {
                        if q > k {
                            assert(crate::mc_model::goal_ok(mm, gs[q], t3));
                        }
                    }
                }
                return true;
            }
            proof {
                assert forall|t3: Map<nat, Term>|
                    crate::mc_subst::extends(t3, th) && crate::mc_model::goals_sat(
                        mm,
                        gs,
                        k as int,
                        t3,
                    ) implies !exists|i2: int|
                    0 <= i2 < i + 1 && #[trigger] m.ms@[i2] == ckc_spec::model::apply(g, t3) by {
                    if m.ms@[i as int] == ckc_spec::model::apply(g, t3) {
                        assert(before[m.facts@[i as int] as int].term@ == m.ms@[i as int]);
                        assert(crate::mc_subst::extends(t3, th2@));
                        assert(crate::mc_model::goals_sat(mm, gs, k as int + 1, t3));
                    }
                }
            }
        } else {
            proof {
                assert forall|t3: Map<nat, Term>|
                    crate::mc_subst::extends(t3, th) && crate::mc_model::goals_sat(
                        mm,
                        gs,
                        k as int,
                        t3,
                    ) implies !exists|i2: int|
                    0 <= i2 < i + 1 && #[trigger] m.ms@[i2] == ckc_spec::model::apply(g, t3) by {
                    assert(before[m.facts@[i as int] as int].term@ == m.ms@[i as int]);
                }
            }
        }
        i += 1;
    }
    proof {
        assert forall|t3: Map<nat, Term>|
            crate::mc_subst::extends(t3, th) && crate::mc_model::goals_sat(
                mm,
                gs,
                k as int,
                t3,
            ) implies false by {
            assert(crate::mc_model::goal_ok(mm, g, t3));
            crate::mc_subst::builtin_apply_iff(g, t3);
            crate::mc_model::iset_of_seq(m.ms@, ckc_spec::model::apply(g, t3));
            let i2 = choose|i2: int|
                0 <= i2 < m.ms@.len() && m.ms@[i2] == ckc_spec::model::apply(g, t3);
        }
    }
    false
}

} // verus!
