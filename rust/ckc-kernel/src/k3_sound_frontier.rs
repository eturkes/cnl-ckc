use super::goals::*;
use super::graph::*;
use super::log::*;
use super::*;

verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub open spec fn paths(stack: Seq<TGoal>) -> Seq<Seq<nat>> {
    stack.map_values(|g: TGoal| goal_frame(g).2)
}

pub open spec fn children(path: Seq<nat>, n: nat) -> Seq<Seq<nat>> {
    Seq::new(n, |i: int| path.push(i as nat))
}

pub open spec fn event_children(db: Seq<DocClause>, path: Seq<nat>, ev: TEv) -> Seq<Seq<nat>> {
    match ev {
        TEv::Clause(m) => if m < db.len() {
            children(path, db[m as int].body.len())
        } else {
            seq![]
        },
        TEv::Naf(_) => seq![],
    }
}

pub open spec fn unique_paths(ps: Seq<Seq<nat>>) -> bool {
    forall|i: int, j: int| 0 <= i < j < ps.len() ==> #[trigger] ps[i] != #[trigger] ps[j]
}

pub open spec fn unique_log(log: Seq<(Seq<nat>, TEv)>) -> bool {
    unique_paths(log.map_values(|e: (Seq<nat>, TEv)| e.0))
}

pub open spec fn positioned_log(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    log: Seq<(Seq<nat>, TEv)>,
) -> bool {
    forall|i: int| 0 <= i < log.len() ==> #[trigger] position(db, roots, log, log[i].0)
}

pub open spec fn open_position(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    log: Seq<(Seq<nat>, TEv)>,
    path: Seq<nat>,
) -> bool {
    position(db, roots, log, path) && path.len() <= log.len() + 1 && !logged(log, path)
}

pub open spec fn frontier_ok(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    log: Seq<(Seq<nat>, TEv)>,
    front: Seq<Seq<nat>>,
) -> bool {
    unique_paths(front) && forall|i: int|
        0 <= i < front.len() ==> #[trigger] open_position(db, roots, log, front[i])
}

pub open spec fn covered(log: Seq<(Seq<nat>, TEv)>, front: Seq<Seq<nat>>, path: Seq<nat>) -> bool {
    logged(log, path) || front.contains(path)
}

pub open spec fn cover_event(
    db: Seq<DocClause>,
    log: Seq<(Seq<nat>, TEv)>,
    front: Seq<Seq<nat>>,
    path: Seq<nat>,
    ev: TEv,
) -> bool {
    match ev {
        TEv::Clause(m) => m < db.len() && forall|i: int|
            0 <= i < db[m as int].body.len() ==> #[trigger] covered(
                log,
                front,
                path.push(i as nat),
            ),
        TEv::Naf(_) => true,
    }
}

pub open spec fn cover_log(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    log: Seq<(Seq<nat>, TEv)>,
    front: Seq<Seq<nat>>,
) -> bool {
    (forall|i: int| 0 <= i < roots.len() ==> #[trigger] covered(log, front, seq![i as nat])) && (
    forall|i: int|
        0 <= i < log.len() ==> #[trigger] cover_event(db, log, front, log[i].0, log[i].1))
}

pub proof fn paths_add(a: Seq<TGoal>, b: Seq<TGoal>)
    ensures
        paths(a + b) == paths(a) + paths(b),
{
    assert(paths(a + b) =~= paths(a) + paths(b));
}

pub proof fn paths_apply(stack: Seq<TGoal>, s: Seq<(nat, Term)>)
    ensures
        paths(tapply_all(stack, s)) == paths(stack),
{
    assert(paths(tapply_all(stack, s)) =~= paths(stack));
}

pub proof fn tbody_paths(items: Seq<BodyItem>, off: nat, d: nat, path: Seq<nat>)
    ensures
        paths(tbody_goals(items, off, d, path)) == children(path, items.len()),
{
    assert(paths(tbody_goals(items, off, d, path)) =~= children(path, items.len()));
}

pub proof fn cert_positioned(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    log: Seq<(Seq<nat>, TEv)>,
    offsets: Seq<nat>,
    th: Seq<(nat, Term)>,
)
    requires
        cert_log(db, roots, log, offsets, th),
    ensures
        positioned_log(db, roots, log),
{
    assert forall|i: int| 0 <= i < log.len() implies #[trigger] position(
        db,
        roots,
        log,
        log[i].0,
    ) by {
        assert(cert_event(db, roots, log, offsets, th, log[i].0, log[i].1));
    }
}

pub proof fn ev_push_other(log: Seq<(Seq<nat>, TEv)>, p: Seq<nat>, ev: TEv, q: Seq<nat>)
    requires
        p != q,
    ensures
        ev_at(log.push((p, ev)), q) == ev_at(log, q),
    decreases log.len(),
{
    reveal_with_fuel(ev_at, 2);
    if log.len() > 0 && log[0].0 != q {
        assert(log.push((p, ev)).drop_first() =~= log.drop_first().push((p, ev)));
        ev_push_other(log.drop_first(), p, ev, q);
    }
}

pub proof fn log_push_unique(log: Seq<(Seq<nat>, TEv)>, path: Seq<nat>, ev: TEv)
    requires
        unique_log(log),
        !logged(log, path),
    ensures
        unique_log(log.push((path, ev))),
{
    let keys = log.map_values(|e: (Seq<nat>, TEv)| e.0);
    let next = log.push((path, ev)).map_values(|e: (Seq<nat>, TEv)| e.0);
    assert forall|i: int, j: int| 0 <= i < j < next.len() implies #[trigger] next[i]
        != #[trigger] next[j] by {
        if j < log.len() {
            assert(keys[i] != keys[j]);
        } else {
            index_logged(log, i);
            assert(log[i].0 != path);
        }
    }
}

pub proof fn child_unlogged(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    log: Seq<(Seq<nat>, TEv)>,
    p: Seq<nat>,
    ev: TEv,
    i: nat,
)
    requires
        positioned_log(db, roots, log),
        p.len() > 0,
        !logged(log, p),
    ensures
        !logged(log.push((p, ev)), p.push(i)),
{
    let child = p.push(i);
    if logged(log, child) {
        let e = ev_at(log, child).unwrap();
        lookup_member(log, child, e);
        let j = choose|j: int| 0 <= j < log.len() && log[j] == (child, e);
        fresh_position_not_child(db, roots, log, log[j].0, p, i);
    }
    assert(p != child);
    ev_push_other(log, p, ev, child);
}

pub proof fn covered_move(
    log: Seq<(Seq<nat>, TEv)>,
    p: Seq<nat>,
    ev: TEv,
    rest: Seq<Seq<nat>>,
    kids: Seq<Seq<nat>>,
    q: Seq<nat>,
)
    requires
        covered(log, seq![p] + rest, q),
    ensures
        covered(log.push((p, ev)), kids + rest, q),
{
    if logged(log, q) {
        ev_append(log, seq![(p, ev)], q);
        assert(log + seq![(p, ev)] =~= log.push((p, ev)));
    } else if q == p {
        ev_push(log, p, ev);
    } else {
        let old = seq![p] + rest;
        let i = choose|i: int| 0 <= i < old.len() && old[i] == q;
        assert(i > 0);
        assert(rest[i - 1] == q);
        assert((kids + rest)[kids.len() + i - 1] == q);
        assert((kids + rest).contains(q));
    }
}

pub proof fn cover_replace(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    log: Seq<(Seq<nat>, TEv)>,
    p: Seq<nat>,
    ev: TEv,
    rest: Seq<Seq<nat>>,
)
    requires
        cover_log(db, roots, log, seq![p] + rest),
        ev matches TEv::Clause(m) ==> m < db.len(),
    ensures
        cover_log(db, roots, log.push((p, ev)), event_children(db, p, ev) + rest),
{
    let kids = event_children(db, p, ev);
    let next = log.push((p, ev));
    let front = kids + rest;
    assert forall|i: int| 0 <= i < roots.len() implies #[trigger] covered(
        next,
        front,
        seq![i as nat],
    ) by {
        covered_move(log, p, ev, rest, kids, seq![i as nat]);
    }
    assert forall|j: int| 0 <= j < next.len() implies #[trigger] cover_event(
        db,
        next,
        front,
        next[j].0,
        next[j].1,
    ) by {
        if j < log.len() {
            assert(cover_event(db, log, seq![p] + rest, log[j].0, log[j].1));
            if let TEv::Clause(m) = log[j].1 {
                assert forall|i: int| 0 <= i < db[m as int].body.len() implies #[trigger] covered(
                    next,
                    front,
                    log[j].0.push(i as nat),
                ) by {
                    covered_move(log, p, ev, rest, kids, log[j].0.push(i as nat));
                }
            }
        } else {
            if let TEv::Clause(m) = ev {
                assert forall|i: int| 0 <= i < db[m as int].body.len() implies #[trigger] covered(
                    next,
                    front,
                    p.push(i as nat),
                ) by {
                    assert(kids[i] == p.push(i as nat));
                    assert(front[i] == p.push(i as nat));
                    assert(front.contains(p.push(i as nat)));
                }
            }
        }
    }
}

pub proof fn frontier_replace(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    log: Seq<(Seq<nat>, TEv)>,
    p: Seq<nat>,
    ev: TEv,
    rest: Seq<Seq<nat>>,
)
    requires
        positioned_log(db, roots, log),
        frontier_ok(db, roots, log, seq![p] + rest),
        ev matches TEv::Clause(m) ==> m < db.len(),
    ensures
        frontier_ok(db, roots, log.push((p, ev)), event_children(db, p, ev) + rest),
{
    let old = seq![p] + rest;
    let kids = event_children(db, p, ev);
    let front = kids + rest;
    let next = log.push((p, ev));
    assert(open_position(db, roots, log, old[0]));
    assert(old[0] == p);
    ev_push(log, p, ev);
    assert forall|i: int, j: int| 0 <= i < j < front.len() implies #[trigger] front[i]
        != #[trigger] front[j] by {
        if j < kids.len() {
            assert(front[i].last() == i as nat);
            assert(front[j].last() == j as nat);
        } else if i < kids.len() {
            let q = rest[j - kids.len()];
            assert(open_position(db, roots, log, old[j - kids.len() + 1]));
            fresh_position_not_child(db, roots, log, q, p, i as nat);
        } else {
            assert(old[i - kids.len() + 1] != old[j - kids.len() + 1]);
        }
    }
    assert(unique_paths(front));
    assert forall|i: int| 0 <= i < front.len() implies #[trigger] open_position(
        db,
        roots,
        next,
        front[i],
    ) by {
        if i < kids.len() {
            assert(front[i] == p.push(i as nat));
            assert(front[i].drop_last() =~= p);
            assert(front[i].last() == i as nat);
            child_unlogged(db, roots, log, p, ev, i as nat);
        } else {
            let j = i - kids.len();
            assert(open_position(db, roots, log, old[j + 1]));
            assert(old[j + 1] == front[i]);
            position_append(db, roots, log, seq![(p, ev)], front[i]);
            assert(log + seq![(p, ev)] =~= next);
            assert(old[0] != old[j + 1]);
            ev_push_other(log, p, ev, front[i]);
        }
    }
    assert(frontier_ok(db, roots, next, front));
}

pub proof fn cover_empty_complete(db: Seq<DocClause>, roots: Seq<Term>, log: Seq<(Seq<nat>, TEv)>)
    requires
        cover_log(db, roots, log, seq![]),
    ensures
        complete_log(db, roots, log),
{
    assert forall|i: int| 0 <= i < roots.len() implies #[trigger] logged(log, seq![i as nat]) by {
        assert(covered(log, seq![], seq![i as nat]));
    }
    assert forall|i: int| 0 <= i < log.len() implies #[trigger] closed_event(
        db,
        log,
        log[i].0,
        log[i].1,
    ) by {
        assert(cover_event(db, log, seq![], log[i].0, log[i].1));
        if let TEv::Clause(m) = log[i].1 {
            assert forall|j: int| 0 <= j < db[m as int].body.len() implies #[trigger] logged(
                log,
                log[i].0.push(j as nat),
            ) by {
                assert(covered(log, seq![], log[i].0.push(j as nat)));
            }
        }
    }
}

} // verus!
