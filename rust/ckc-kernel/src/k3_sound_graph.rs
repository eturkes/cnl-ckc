use super::*;

verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub open spec fn offset_at(log: Seq<(Seq<nat>, TEv)>, offsets: Seq<nat>, path: Seq<nat>) -> nat
    decreases log.len(),
{
    if log.len() == 0 || offsets.len() == 0 {
        0
    } else if log[0].0 == path {
        offsets[0]
    } else {
        offset_at(log.drop_first(), offsets.drop_first(), path)
    }
}

pub open spec fn logged(log: Seq<(Seq<nat>, TEv)>, path: Seq<nat>) -> bool {
    ev_at(log, path) is Some
}

pub open spec fn position(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    log: Seq<(Seq<nat>, TEv)>,
    path: Seq<nat>,
) -> bool {
    if path.len() == 0 {
        false
    } else if path.len() == 1 {
        path[0] < roots.len()
    } else {
        match ev_at(log, path.drop_last()) {
            Option::Some(TEv::Clause(m)) => m < db.len() && path.last() < db[m as int].body.len(),
            _ => false,
        }
    }
}

pub open spec fn raw_at(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    log: Seq<(Seq<nat>, TEv)>,
    offsets: Seq<nat>,
    path: Seq<nat>,
) -> Term {
    if path.len() == 0 {
        Term::Nil
    } else if path.len() == 1 {
        if path[0] < roots.len() {
            roots[path[0] as int]
        } else {
            Term::Nil
        }
    } else {
        match ev_at(log, path.drop_last()) {
            Option::Some(TEv::Clause(m)) => {
                if m < db.len() && path.last() < db[m as int].body.len() {
                    item_term(
                        db[m as int].body[path.last() as int],
                        offset_at(log, offsets, path.drop_last()),
                    )
                } else {
                    Term::Nil
                }
            },
            _ => Term::Nil,
        }
    }
}

// NAF finite failure is supplied at close; inner-search bindings stay outside this relation.
pub open spec fn weak_event(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    log: Seq<(Seq<nat>, TEv)>,
    offsets: Seq<nat>,
    th: Seq<(nat, Term)>,
    path: Seq<nat>,
    ev: TEv,
) -> bool {
    let g = raw_at(db, roots, log, offsets, path);
    match ev {
        TEv::Clause(m) => m < db.len() && apply(g, th) == apply(
            shift(db[m as int].head, offset_at(log, offsets, path)),
            th,
        ),
        TEv::Naf(t) => match g {
            Term::Comp(name, args) => name == naf_name() && args.len() == 1 && exists|
                s: Seq<(nat, Term)>,
            | #[trigger]
                apply(t, s) == apply(args[0], th),
            _ => false,
        },
    }
}

pub open spec fn cert_event(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    log: Seq<(Seq<nat>, TEv)>,
    offsets: Seq<nat>,
    th: Seq<(nat, Term)>,
    path: Seq<nat>,
    ev: TEv,
) -> bool {
    position(db, roots, log, path) && 1 <= path.len() <= log.len() && weak_event(
        db,
        roots,
        log,
        offsets,
        th,
        path,
        ev,
    )
}

pub open spec fn cert_log(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    log: Seq<(Seq<nat>, TEv)>,
    offsets: Seq<nat>,
    th: Seq<(nat, Term)>,
) -> bool {
    offsets.len() == log.len() && forall|i: int|
        0 <= i < log.len() ==> #[trigger] cert_event(
            db,
            roots,
            log,
            offsets,
            th,
            log[i].0,
            log[i].1,
        )
}

pub open spec fn closed_event(
    db: Seq<DocClause>,
    log: Seq<(Seq<nat>, TEv)>,
    path: Seq<nat>,
    ev: TEv,
) -> bool {
    match ev {
        TEv::Clause(m) => m < db.len() && forall|i: int|
            0 <= i < db[m as int].body.len() ==> #[trigger] logged(log, path.push(i as nat)),
        TEv::Naf(_) => true,
    }
}

pub open spec fn complete_log(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    log: Seq<(Seq<nat>, TEv)>,
) -> bool {
    (forall|i: int| 0 <= i < roots.len() ==> #[trigger] logged(log, seq![i as nat])) && (forall|
        i: int,
    |
        0 <= i < log.len() ==> #[trigger] closed_event(db, log, log[i].0, log[i].1))
}

pub open spec fn naf_event_ok(db: Seq<DocClause>, ev: TEv) -> bool {
    match ev {
        TEv::Naf(t) => naf_fails(db, t),
        TEv::Clause(_) => true,
    }
}

pub open spec fn naf_log_ok(db: Seq<DocClause>, log: Seq<(Seq<nat>, TEv)>) -> bool {
    forall|i: int| 0 <= i < log.len() ==> #[trigger] naf_event_ok(db, log[i].1)
}

pub proof fn lookup_member(log: Seq<(Seq<nat>, TEv)>, path: Seq<nat>, ev: TEv)
    requires
        ev_at(log, path) == Option::Some(ev),
    ensures
        exists|i: int| 0 <= i < log.len() && #[trigger] log[i] == (path, ev),
    decreases log.len(),
{
    if log.len() > 0 {
        if log[0].0 == path {
            assert(log[0] == (path, ev));
        } else {
            lookup_member(log.drop_first(), path, ev);
            let i = choose|i: int|
                0 <= i < log.drop_first().len() && log.drop_first()[i] == (path, ev);
            assert(log[i + 1] == (path, ev));
        }
    }
}

pub proof fn event_properties(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    log: Seq<(Seq<nat>, TEv)>,
    offsets: Seq<nat>,
    th: Seq<(nat, Term)>,
    path: Seq<nat>,
    ev: TEv,
)
    requires
        cert_log(db, roots, log, offsets, th),
        complete_log(db, roots, log),
        naf_log_ok(db, log),
        ev_at(log, path) == Option::Some(ev),
    ensures
        cert_event(db, roots, log, offsets, th, path, ev),
        closed_event(db, log, path, ev),
        naf_event_ok(db, ev),
{
    lookup_member(log, path, ev);
    let i = choose|i: int| 0 <= i < log.len() && log[i] == (path, ev);
    assert(cert_event(db, roots, log, offsets, th, log[i].0, log[i].1));
    assert(closed_event(db, log, log[i].0, log[i].1));
    assert(naf_event_ok(db, log[i].1));
}

pub proof fn build_all_map(
    db: Seq<DocClause>,
    log: Seq<(Seq<nat>, TEv)>,
    path: Seq<nat>,
    i: nat,
    n: nat,
)
    requires
        i <= n,
        path.len() <= log.len(),
    ensures
        build_all(db, log, path, i, n) == Seq::new(
            (n - i) as nat,
            |j: int| build(db, log, path.push(i + j as nat)),
        ),
    decreases n - i,
{
    if i < n {
        build_all_map(db, log, path, i + 1, n);
    }
    assert(build_all(db, log, path, i, n) =~= Seq::new(
        (n - i) as nat,
        |j: int| build(db, log, path.push(i + j as nat)),
    ));
}

pub proof fn kids_pointwise(
    db: Seq<DocClause>,
    th: Seq<(nat, Term)>,
    gs: Seq<Term>,
    kids: Seq<PNode>,
)
    requires
        gs.len() == kids.len(),
        forall|i: int| 0 <= i < gs.len() ==> #[trigger] node_valid(db, th, gs[i], kids[i]),
    ensures
        kids_valid(db, th, gs, kids),
    decreases kids.len(),
{
    if kids.len() > 0 {
        assert forall|i: int| 0 <= i < gs.drop_first().len() implies #[trigger] node_valid(
            db,
            th,
            gs.drop_first()[i],
            kids.drop_first()[i],
        ) by {
            assert(gs.drop_first()[i] == gs[i + 1]);
            assert(kids.drop_first()[i] == kids[i + 1]);
        }
        kids_pointwise(db, th, gs.drop_first(), kids.drop_first());
        assert(node_valid(db, th, gs[0], kids[0]));
    }
}

pub proof fn clause_node_valid(
    db: Seq<DocClause>,
    th: Seq<(nat, Term)>,
    g: Term,
    m: nat,
    k: nat,
    kids: Seq<PNode>,
)
    requires
        m < db.len(),
        resolves(db, th, g, m, k, kids),
    ensures
        node_valid(db, th, g, PNode::Clause(m, kids)),
{
    assert(false);
}

pub proof fn naf_build_valid(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    log: Seq<(Seq<nat>, TEv)>,
    offsets: Seq<nat>,
    th: Seq<(nat, Term)>,
    path: Seq<nat>,
    t: Term,
)
    requires
        cert_log(db, roots, log, offsets, th),
        complete_log(db, roots, log),
        naf_log_ok(db, log),
        ev_at(log, path) == Option::Some(TEv::Naf(t)),
    ensures
        node_valid(db, th, raw_at(db, roots, log, offsets, path), build(db, log, path)),
{
    event_properties(db, roots, log, offsets, th, path, TEv::Naf(t));
    let g = raw_at(db, roots, log, offsets, path);
    assert(weak_event(db, roots, log, offsets, th, path, TEv::Naf(t)));
    assert(naf_fails(db, t));
    if let Term::Comp(name, args) = g {
        let s = choose|s: Seq<(nat, Term)>| apply(t, s) == apply(args[0], th);
        assert(apply(t, s) == apply(args[0], th));
    }
    assert(node_valid(db, th, g, PNode::Naf(t)));
    assert(path.len() <= log.len());
    assert(build(db, log, path) == PNode::Naf(t));
}

pub proof fn build_valid(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    log: Seq<(Seq<nat>, TEv)>,
    offsets: Seq<nat>,
    th: Seq<(nat, Term)>,
    path: Seq<nat>,
)
    requires
        cert_log(db, roots, log, offsets, th),
        complete_log(db, roots, log),
        naf_log_ok(db, log),
        logged(log, path),
    ensures
        node_valid(db, th, raw_at(db, roots, log, offsets, path), build(db, log, path)),
    decreases log.len() + 1 - path.len(),
{
    hide(node_valid);
    let ev = ev_at(log, path).unwrap();
    event_properties(db, roots, log, offsets, th, path, ev);
    match ev {
        TEv::Naf(t) => {
            naf_build_valid(db, roots, log, offsets, th, path, t);
        },
        TEv::Clause(m) => {
            let k = offset_at(log, offsets, path);
            let gs = body_terms(db[m as int].body, k);
            let kids = build_all(db, log, path, 0, gs.len());
            build_all_map(db, log, path, 0, gs.len());
            assert forall|i: int| 0 <= i < gs.len() implies #[trigger] node_valid(
                db,
                th,
                gs[i],
                kids[i],
            ) by {
                let child = path.push(i as nat);
                assert(logged(log, child));
                assert(child.drop_last() =~= path);
                assert(child.last() == i as nat);
                build_valid(db, roots, log, offsets, th, child);
                assert(raw_at(db, roots, log, offsets, child) == gs[i]);
            }
            kids_pointwise(db, th, gs, kids);
            assert(resolves(db, th, raw_at(db, roots, log, offsets, path), m, k, kids));
            assert(build(db, log, path) == PNode::Clause(m, kids));
            clause_node_valid(db, th, raw_at(db, roots, log, offsets, path), m, k, kids);
        },
    }
}

pub proof fn certified_forest(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    log: Seq<(Seq<nat>, TEv)>,
    offsets: Seq<nat>,
    th: Seq<(nat, Term)>,
)
    requires
        cert_log(db, roots, log, offsets, th),
        complete_log(db, roots, log),
        naf_log_ok(db, log),
    ensures
        kids_valid(db, th, roots, forest_of(db, roots, log)),
{
    let forest = forest_of(db, roots, log);
    assert forall|i: int| 0 <= i < roots.len() implies #[trigger] node_valid(
        db,
        th,
        roots[i],
        forest[i],
    ) by {
        assert(logged(log, seq![i as nat]));
        build_valid(db, roots, log, offsets, th, seq![i as nat]);
        assert(raw_at(db, roots, log, offsets, seq![i as nat]) == roots[i]);
    }
    kids_pointwise(db, th, roots, forest);
}

pub proof fn forest_of_sound(
    db: Seq<DocClause>,
    goal: Term,
    log: Seq<(Seq<nat>, TEv)>,
    offsets: Seq<nat>,
    th: Seq<(nat, Term)>,
)
    requires
        cert_log(db, conj_leaves(goal), log, offsets, th),
        complete_log(db, conj_leaves(goal), log),
        naf_log_ok(db, log),
    ensures
        forest_valid(db, goal, forest_of(db, conj_leaves(goal), log)),
{
    certified_forest(db, conj_leaves(goal), log, offsets, th);
    assert(kids_valid(db, th, conj_leaves(goal), forest_of(db, conj_leaves(goal), log)));
}

} // verus!
