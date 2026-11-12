use crate::k2_engine::EBodyItem;
use crate::k2_term::{ENode, ENodeKind, ETermArena};
use ckc_spec::term::Term;
use ckc_spec::v1text::DocClause;
use vstd::prelude::*;

verus! {

// Contract mc D2 G3: the patient boundary. KBNamespaces = every namespace a knowledge clause
// mentions; a patient fact = a ground fact with no subterm in it.
pub open spec fn held(nodes: Seq<ENode>, ns: Seq<usize>) -> ISet<Term> {
    ISet::new(
        |n: Term| exists|i: int| 0 <= i < ns.len() && #[trigger] nodes[ns[i] as int].term@ == n,
    )
}

// kb_namespaces's per-clause disjunction.
pub open spec fn clause_names(c: DocClause, n: Term) -> bool {
    ckc_spec::model::namespaces(c.head).contains(n) || exists|j: int, k: int|
        0 <= j < c.body.len() && 0 <= k < ckc_spec::model::item_terms(c.body[j]).len()
            && #[trigger] ckc_spec::model::namespaces(
            ckc_spec::model::item_terms(c.body[j])[k],
        ).contains(n)
}

// The namespaces of the knowledge clauses among the first m.
pub open spec fn kb_upto(p: ckc_spec::model::Program, m: int) -> ISet<Term> {
    ISet::new(
        |n: Term|
            exists|ci: int|
                0 <= ci < m && #[trigger] ckc_spec::model::knowledge(p, ci) && clause_names(
                    p.clauses[ci],
                    n,
                ),
    )
}

pub proof fn subterms_all_any(ts: Seq<Term>, s: Term)
    ensures
        ckc_spec::model::subterms_all(ts).contains(s) == exists|q: int|
            0 <= q < ts.len() && #[trigger] ckc_spec::model::subterms(ts[q]).contains(s),
    decreases ts.len(),
{
    if ts.len() > 0 {
        let rest = ts.drop_first();
        subterms_all_any(rest, s);
        if ckc_spec::model::subterms_all(ts).contains(s) {
            if !ckc_spec::model::subterms(ts[0]).contains(s) {
                let q = choose|q: int|
                    0 <= q < rest.len() && #[trigger] ckc_spec::model::subterms(rest[q]).contains(
                        s,
                    );
                assert(rest[q] == ts[q + 1]);
            }
        } else {
            assert forall|q: int| 0 <= q < ts.len() implies !#[trigger] ckc_spec::model::subterms(
                ts[q],
            ).contains(s) by {
                if q > 0 {
                    assert(rest[q - 1] == ts[q]);
                }
            }
        }
    }
}

pub proof fn ground_all_at(ts: Seq<Term>, q: int)
    requires
        ckc_spec::term::ground_all(ts),
        0 <= q < ts.len(),
    ensures
        ckc_spec::term::ground(ts[q]),
    decreases ts.len(),
{
    if q > 0 {
        assert(ts.drop_first()[q - 1] == ts[q]);
        ground_all_at(ts.drop_first(), q - 1);
    }
}

// The namespaces of t = its own + its arguments'.
proof fn namespaces_unfold(t: Term, n: Term)
    ensures
        ckc_spec::model::namespaces(t).contains(n) == (ckc_spec::model::namespace_of(t) == Some(n)
            || (t is Comp && exists|q: int|
            0 <= q < t->Comp_1.len() && #[trigger] ckc_spec::model::namespaces(
                t->Comp_1[q],
            ).contains(n))),
{
    match t {
        Term::Comp(f, args) => {
            assert(ckc_spec::model::subterms(t) == set![t].union(
                ckc_spec::model::subterms_all(args),
            ));
            if ckc_spec::model::namespaces(t).contains(n) {
                let s = choose|s: Term| #[trigger]
                    ckc_spec::model::subterms(t).contains(s) && ckc_spec::model::namespace_of(s)
                        == Some(n);
                if s != t {
                    subterms_all_any(args, s);
                    let q = choose|q: int|
                        0 <= q < args.len() && #[trigger] ckc_spec::model::subterms(
                            args[q],
                        ).contains(s);
                    assert(ckc_spec::model::namespaces(args[q]).contains(n));
                }
            }
            if ckc_spec::model::namespace_of(t) == Some(n) {
                assert(ckc_spec::model::subterms(t).contains(t));
            }
            if exists|q: int|
                0 <= q < args.len() && #[trigger] ckc_spec::model::namespaces(args[q]).contains(n) {
                let q = choose|q: int|
                    0 <= q < args.len() && #[trigger] ckc_spec::model::namespaces(args[q]).contains(
                        n,
                    );
                let s = choose|s: Term| #[trigger]
                    ckc_spec::model::subterms(args[q]).contains(s) && ckc_spec::model::namespace_of(
                        s,
                    ) == Some(n);
                subterms_all_any(args, s);
                assert(ckc_spec::model::subterms(t).contains(s));
            }
        },
        _ => {
            assert(ckc_spec::model::subterms(t) == set![t]);
            if ckc_spec::model::namespaces(t).contains(n) {
                let s = choose|s: Term| #[trigger]
                    ckc_spec::model::subterms(t).contains(s) && ckc_spec::model::namespace_of(s)
                        == Some(n);
                assert(s == t);
            }
        },
    }
}

// Push the namespace of every subterm of arena@[root] onto ns.
fn collect_ns(arena: &ETermArena, root: usize, ns: &mut Vec<usize>)
    requires
        crate::k2_term::root_ok(arena, root),
        crate::k2_engine::roots_valid(arena.nodes@, old(ns)@),
    ensures
        crate::k2_engine::roots_valid(arena.nodes@, final(ns)@),
        held(arena.nodes@, final(ns)@) == held(arena.nodes@, old(ns)@).union(
            ckc_spec::model::namespaces(arena@[root as int]),
        ),
    decreases root,
{
    let ghost t = arena@[root as int];
    let ghost ns0 = ns@;
    proof {
        assert(crate::k2_term::node_ok(arena.nodes@, root as int));
    }
    match &arena.nodes[root].kind {
        ENodeKind::Comp { name, child_roots, .. } => {
            let ghost args = crate::k2_term::child_terms(arena.nodes@, child_roots@);
            proof {
                assert(t == Term::Comp(name@, args));
            }
            let id: &[u8] = b"$guideline_id";
            proof {
                reveal_byteslit(b"$guideline_id");
                reveal_strlit("$guideline_id");
                reveal(ckc_spec::v1text::ascii);
                assert(id@ == ckc_spec::v1text::ascii("$guideline_id"@));
            }
            if child_roots.len() == 5 && crate::mc_g1::name_is(name, id, Ghost("$guideline_id"@)) {
                ns.push(child_roots[1]);
            } else if child_roots.len() == 4 && crate::mc_g1::indicator_named(name, 1) {
                ns.push(child_roots[0]);
            }
            let ghost ns1 = ns@;
            proof {
                assert forall|n: Term| #[trigger]
                    held(arena.nodes@, ns1).contains(n) == (held(arena.nodes@, ns0).contains(n)
                        || ckc_spec::model::namespace_of(t) == Some(n)) by {
                    if held(arena.nodes@, ns1).contains(n) {
                        let i = choose|i: int|
                            0 <= i < ns1.len() && #[trigger] arena.nodes@[ns1[i] as int].term@ == n;
                        if i < ns0.len() {
                            assert(ns1[i] == ns0[i]);
                        }
                    }
                    if held(arena.nodes@, ns0).contains(n) {
                        let i = choose|i: int|
                            0 <= i < ns0.len() && #[trigger] arena.nodes@[ns0[i] as int].term@ == n;
                        assert(ns1[i] == ns0[i]);
                    }
                    if ckc_spec::model::namespace_of(t) == Some(n) {
                        assert(ns1.len() == ns0.len() + 1);
                        let last = ns0.len() as int;
                        assert(arena.nodes@[ns1[last] as int].term@ == n);
                    }
                }
            }
            let mut k = 0usize;
            while k < child_roots.len()
                invariant
                    crate::k2_term::root_ok(arena, root),
                    crate::k2_term::child_roots_before(child_roots@, root as int),
                    crate::k2_term::child_roots_valid(arena.nodes@, child_roots@),
                    args == crate::k2_term::child_terms(arena.nodes@, child_roots@),
                    crate::k2_engine::roots_valid(arena.nodes@, ns@),
                    k <= child_roots@.len(),
                    forall|n: Term| #[trigger]
                        held(arena.nodes@, ns@).contains(n) == (held(arena.nodes@, ns1).contains(n)
                            || exists|q: int|
                            0 <= q < k && #[trigger] ckc_spec::model::namespaces(args[q]).contains(
                                n,
                            )),
                decreases child_roots.len() - k,
            {
                let ghost before = ns@;
                proof {
                    assert(args[k as int] == arena@[child_roots@[k as int] as int]);
                }
                collect_ns(arena, child_roots[k], ns);
                proof {
                    assert forall|n: Term| #[trigger]
                        held(arena.nodes@, ns@).contains(n) == (held(arena.nodes@, ns1).contains(n)
                            || exists|q: int|
                            0 <= q < k + 1 && #[trigger] ckc_spec::model::namespaces(
                                args[q],
                            ).contains(n)) by {
                        if ckc_spec::model::namespaces(args[k as int]).contains(n) {
                            assert(exists|q: int|
                                0 <= q < k + 1 && #[trigger] ckc_spec::model::namespaces(
                                    args[q],
                                ).contains(n));
                        }
                    }
                }
                k += 1;
            }
            proof {
                assert forall|n: Term| #[trigger]
                    held(arena.nodes@, ns@).contains(n) == held(arena.nodes@, ns0).union(
                        ckc_spec::model::namespaces(t),
                    ).contains(n) by {
                    namespaces_unfold(t, n);
                }
                assert(held(arena.nodes@, ns@) =~= held(arena.nodes@, ns0).union(
                    ckc_spec::model::namespaces(t),
                ));
            }
        },
        _ => {
            proof {
                assert forall|n: Term| #[trigger]
                    held(arena.nodes@, ns@).contains(n) == held(arena.nodes@, ns0).union(
                        ckc_spec::model::namespaces(t),
                    ).contains(n) by {
                    namespaces_unfold(t, n);
                }
                assert(held(arena.nodes@, ns@) =~= held(arena.nodes@, ns0).union(
                    ckc_spec::model::namespaces(t),
                ));
            }
        },
    }
}

// KBNamespaces of the program `prog` holds, the patient boundary at pb.
pub fn kb_names(arena: &ETermArena, prog: &crate::mc_load::EMcProgram, pb: usize) -> (ns: Vec<
    usize,
>)
    requires
        crate::k2_term::arena_ok(arena),
        crate::mc_load::mc_program_ok(arena.nodes@, prog),
        pb <= prog.ds@.len(),
    ensures
        crate::k2_engine::roots_valid(arena.nodes@, ns@),
        held(arena.nodes@, ns@) == ckc_spec::model::kb_namespaces(
            ckc_spec::model::program_of(prog.ds@, prog.fs@, pb as nat, Seq::empty()),
        ),
{
    let ghost p = ckc_spec::model::program_of(prog.ds@, prog.fs@, pb as nat, Seq::empty());
    let ghost p0 = ckc_spec::model::program_of(prog.ds@, prog.fs@, 0, Seq::empty());
    proof {
        assert(p.clauses == p0.clauses);
        assert(p.doc == p0.doc);
        assert(p.patient == pb as nat);
    }
    let mut ns: Vec<usize> = Vec::new();
    let mut ci = 0usize;
    while ci < prog.clauses.len()
        invariant
            crate::k2_term::arena_ok(arena),
            crate::mc_load::mc_program_ok(arena.nodes@, prog),
            p == ckc_spec::model::program_of(prog.ds@, prog.fs@, pb as nat, Seq::empty()),
            p.clauses == crate::k2_engine::db_view(arena.nodes@, prog.clauses@),
            crate::mc_load::nats(prog.doc@) == p.doc,
            p.patient == pb as nat,
            ci <= prog.clauses@.len(),
            crate::k2_engine::roots_valid(arena.nodes@, ns@),
            held(arena.nodes@, ns@) == kb_upto(p, ci as int),
        decreases prog.clauses.len() - ci,
    {
        let ghost ns0 = ns@;
        let ghost c = p.clauses[ci as int];
        proof {
            assert(crate::k2_engine::clause_valid(arena.nodes@, &prog.clauses@[ci as int]));
            assert(c == crate::k2_engine::clause_view(arena.nodes@, &prog.clauses@[ci as int]));
            assert(prog.doc@.len() == prog.clauses@.len());
            assert(prog.clauses@.len() == p.clauses.len());
            assert(p.doc[ci as int] == prog.doc@[ci as int] as nat);
        }
        if prog.doc[ci] < pb {
            let clause = &prog.clauses[ci];
            collect_ns(arena, clause.head, &mut ns);
            let ghost nh = ns@;
            let mut j = 0usize;
            while j < clause.body.len()
                invariant
                    crate::k2_term::arena_ok(arena),
                    crate::k2_engine::clause_valid(arena.nodes@, clause),
                    c == crate::k2_engine::clause_view(arena.nodes@, clause),
                    crate::k2_engine::roots_valid(arena.nodes@, ns@),
                    j <= clause.body@.len(),
                    forall|n: Term| #[trigger]
                        held(arena.nodes@, ns@).contains(n) == (held(arena.nodes@, nh).contains(n)
                            || exists|j2: int, k: int|
                            0 <= j2 < j && 0 <= k < ckc_spec::model::item_terms(c.body[j2]).len()
                                && #[trigger] ckc_spec::model::namespaces(
                                ckc_spec::model::item_terms(c.body[j2])[k],
                            ).contains(n)),
                decreases clause.body.len() - j,
            {
                let ghost nj = ns@;
                proof {
                    assert(crate::k2_engine::body_item_valid(
                        arena.nodes@,
                        &clause.body@[j as int],
                    ));
                    assert(c.body[j as int] == crate::k2_engine::body_item_view(
                        arena.nodes@,
                        &clause.body@[j as int],
                    ));
                }
                match &clause.body[j] {
                    EBodyItem::Pos { root } => {
                        collect_ns(arena, *root, &mut ns);
                        proof {
                            let its = ckc_spec::model::item_terms(c.body[j as int]);
                            assert(its == seq![arena@[*root as int]]);
                            assert forall|n: Term| #[trigger]
                                held(arena.nodes@, ns@).contains(n) == (held(
                                    arena.nodes@,
                                    nh,
                                ).contains(n) || exists|j2: int, k: int|
                                    0 <= j2 < j + 1 && 0 <= k < ckc_spec::model::item_terms(
                                        c.body[j2],
                                    ).len() && #[trigger] ckc_spec::model::namespaces(
                                        ckc_spec::model::item_terms(c.body[j2])[k],
                                    ).contains(n)) by {
                                if ckc_spec::model::namespaces(its[0]).contains(n) {
                                    assert(ckc_spec::model::namespaces(
                                        ckc_spec::model::item_terms(c.body[j as int])[0],
                                    ).contains(n));
                                }
                            }
                        }
                    },
                    EBodyItem::Naf { roots } => {
                        let ghost gs = crate::k2_engine::root_terms(arena.nodes@, roots@);
                        let mut k = 0usize;
                        while k < roots.len()
                            invariant
                                crate::k2_term::arena_ok(arena),
                                crate::k2_engine::roots_valid(arena.nodes@, roots@),
                                gs == crate::k2_engine::root_terms(arena.nodes@, roots@),
                                crate::k2_engine::roots_valid(arena.nodes@, ns@),
                                k <= roots@.len(),
                                forall|n: Term| #[trigger]
                                    held(arena.nodes@, ns@).contains(n) == (held(
                                        arena.nodes@,
                                        nj,
                                    ).contains(n) || exists|q: int|
                                        0 <= q < k && #[trigger] ckc_spec::model::namespaces(
                                            gs[q],
                                        ).contains(n)),
                            decreases roots.len() - k,
                        {
                            proof {
                                assert(gs[k as int] == arena@[roots@[k as int] as int]);
                            }
                            collect_ns(arena, roots[k], &mut ns);
                            proof {
                                assert forall|n: Term| #[trigger]
                                    held(arena.nodes@, ns@).contains(n) == (held(
                                        arena.nodes@,
                                        nj,
                                    ).contains(n) || exists|q: int|
                                        0 <= q < k + 1 && #[trigger] ckc_spec::model::namespaces(
                                            gs[q],
                                        ).contains(n)) by {
                                    if ckc_spec::model::namespaces(gs[k as int]).contains(n) {
                                        assert(exists|q: int|
                                            0 <= q < k + 1
                                                && #[trigger] ckc_spec::model::namespaces(
                                                gs[q],
                                            ).contains(n));
                                    }
                                }
                            }
                            k += 1;
                        }
                        proof {
                            assert(ckc_spec::model::item_terms(c.body[j as int]) == gs);
                            assert forall|n: Term| #[trigger]
                                held(arena.nodes@, ns@).contains(n) == (held(
                                    arena.nodes@,
                                    nh,
                                ).contains(n) || exists|j2: int, k2: int|
                                    0 <= j2 < j + 1 && 0 <= k2 < ckc_spec::model::item_terms(
                                        c.body[j2],
                                    ).len() && #[trigger] ckc_spec::model::namespaces(
                                        ckc_spec::model::item_terms(c.body[j2])[k2],
                                    ).contains(n)) by {
                                if exists|q: int|
                                    0 <= q < roots@.len() && #[trigger] ckc_spec::model::namespaces(
                                        gs[q],
                                    ).contains(n) {
                                    let q = choose|q: int|
                                        0 <= q < roots@.len()
                                            && #[trigger] ckc_spec::model::namespaces(
                                            gs[q],
                                        ).contains(n);
                                    assert(ckc_spec::model::namespaces(
                                        ckc_spec::model::item_terms(c.body[j as int])[q],
                                    ).contains(n));
                                }
                            }
                        }
                    },
                }
                j += 1;
            }
            proof {
                assert forall|n: Term| #[trigger]
                    held(arena.nodes@, ns@).contains(n) == kb_upto(p, ci as int + 1).contains(
                        n,
                    ) by {
                    if clause_names(c, n) {
                        assert(ckc_spec::model::knowledge(p, ci as int));
                    }
                    if kb_upto(p, ci as int + 1).contains(n) && !kb_upto(p, ci as int).contains(n) {
                        let cj = choose|cj: int|
                            0 <= cj < ci + 1 && #[trigger] ckc_spec::model::knowledge(p, cj)
                                && clause_names(p.clauses[cj], n);
                        assert(cj == ci);
                    }
                }
                assert(held(arena.nodes@, ns@) =~= kb_upto(p, ci as int + 1));
            }
        } else {
            proof {
                assert(!ckc_spec::model::knowledge(p, ci as int));
                assert forall|n: Term| #[trigger]
                    kb_upto(p, ci as int).contains(n) == kb_upto(p, ci as int + 1).contains(n) by {
                    if kb_upto(p, ci as int + 1).contains(n) {
                        let cj = choose|cj: int|
                            0 <= cj < ci + 1 && #[trigger] ckc_spec::model::knowledge(p, cj)
                                && clause_names(p.clauses[cj], n);
                        assert(cj != ci);
                    }
                }
                assert(kb_upto(p, ci as int) =~= kb_upto(p, ci as int + 1));
            }
        }
        ci += 1;
    }
    proof {
        assert forall|n: Term| #[trigger]
            kb_upto(p, ci as int).contains(n) == ckc_spec::model::kb_namespaces(p).contains(n) by {
            if ckc_spec::model::kb_namespaces(p).contains(n) {
                let cj = choose|cj: int| #[trigger]
                    ckc_spec::model::knowledge(p, cj) && clause_names(p.clauses[cj], n);
                assert(0 <= cj < ci);
            }
        }
        assert(kb_upto(p, ci as int) =~= ckc_spec::model::kb_namespaces(p));
    }
    ns
}

// No subterm of arena@[root] (ground) is a term ns holds.
pub fn clear_of(arena: &ETermArena, root: usize, ns: &Vec<usize>) -> (out: bool)
    requires
        crate::k2_term::root_ok(arena, root),
        crate::k2_engine::roots_valid(arena.nodes@, ns@),
        ckc_spec::term::ground(arena@[root as int]),
    ensures
        out == forall|s: Term| #[trigger]
            ckc_spec::model::subterms(arena@[root as int]).contains(s) ==> !held(
                arena.nodes@,
                ns@,
            ).contains(s),
    decreases root,
{
    let ghost t = arena@[root as int];
    let mut i = 0usize;
    while i < ns.len()
        invariant
            crate::k2_term::root_ok(arena, root),
            crate::k2_engine::roots_valid(arena.nodes@, ns@),
            ckc_spec::term::ground(t),
            t == arena@[root as int],
            i <= ns@.len(),
            forall|q: int| 0 <= q < i ==> #[trigger] arena.nodes@[ns@[q] as int].term@ != t,
        decreases ns.len() - i,
    {
        if crate::k2_walk::ground_root(arena, ns[i]) && crate::k2_sort::term_equal(
            arena,
            root,
            ns[i],
        ) {
            proof {
                assert(arena.nodes@[ns@[i as int] as int].term@ == t);
                assert(held(arena.nodes@, ns@).contains(t));
                if let Term::Comp(_, args) = t {
                    assert(ckc_spec::model::subterms(t) == set![t].union(
                        ckc_spec::model::subterms_all(args),
                    ));
                } else {
                    assert(ckc_spec::model::subterms(t) == set![t]);
                }
                assert(ckc_spec::model::subterms(t).contains(t));
            }
            return false;
        }
        i += 1;
    }
    proof {
        assert(!held(arena.nodes@, ns@).contains(t));
        assert(crate::k2_term::node_ok(arena.nodes@, root as int));
    }
    match &arena.nodes[root].kind {
        ENodeKind::Comp { name, child_roots, .. } => {
            let ghost args = crate::k2_term::child_terms(arena.nodes@, child_roots@);
            proof {
                assert(t == Term::Comp(name@, args));
            }
            let mut k = 0usize;
            while k < child_roots.len()
                invariant
                    crate::k2_term::root_ok(arena, root),
                    crate::k2_term::child_roots_before(child_roots@, root as int),
                    crate::k2_term::child_roots_valid(arena.nodes@, child_roots@),
                    args == crate::k2_term::child_terms(arena.nodes@, child_roots@),
                    crate::k2_engine::roots_valid(arena.nodes@, ns@),
                    t == Term::Comp(name@, args),
                    t == arena@[root as int],
                    ckc_spec::term::ground(t),
                    k <= child_roots@.len(),
                    forall|q: int, s: Term|
                        0 <= q < k && #[trigger] ckc_spec::model::subterms(args[q]).contains(s)
                            ==> !held(arena.nodes@, ns@).contains(s),
                decreases child_roots.len() - k,
            {
                proof {
                    assert(args[k as int] == arena@[child_roots@[k as int] as int]);
                    ground_all_at(args, k as int);
                }
                if !clear_of(arena, child_roots[k], ns) {
                    proof {
                        let s = choose|s: Term| #[trigger]
                            ckc_spec::model::subterms(args[k as int]).contains(s) && held(
                                arena.nodes@,
                                ns@,
                            ).contains(s);
                        subterms_all_any(args, s);
                        assert(ckc_spec::model::subterms(t) == set![t].union(
                            ckc_spec::model::subterms_all(args),
                        ));
                        assert(ckc_spec::model::subterms(t).contains(s));
                    }
                    return false;
                }
                k += 1;
            }
            proof {
                assert(ckc_spec::model::subterms(t) == set![t].union(
                    ckc_spec::model::subterms_all(args),
                ));
                assert forall|s: Term| #[trigger]
                    ckc_spec::model::subterms(t).contains(s) implies !held(
                    arena.nodes@,
                    ns@,
                ).contains(s) by {
                    if s != t {
                        subterms_all_any(args, s);
                        let q = choose|q: int|
                            0 <= q < args.len() && #[trigger] ckc_spec::model::subterms(
                                args[q],
                            ).contains(s);
                    }
                }
            }
            true
        },
        _ => {
            proof {
                assert(ckc_spec::model::subterms(t) == set![t]);
            }
            true
        },
    }
}

} // verus!
