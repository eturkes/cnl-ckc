use super::graph::*;
use super::*;

verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub proof fn index_logged(log: Seq<(Seq<nat>, TEv)>, i: int)
    requires
        0 <= i < log.len(),
    ensures
        logged(log, log[i].0),
    decreases log.len(),
{
    if log[0].0 != log[i].0 {
        index_logged(log.drop_first(), i - 1);
    }
}

pub proof fn ev_append(log: Seq<(Seq<nat>, TEv)>, extra: Seq<(Seq<nat>, TEv)>, path: Seq<nat>)
    requires
        logged(log, path),
    ensures
        ev_at(log + extra, path) == ev_at(log, path),
    decreases log.len(),
{
    if log.len() > 0 && log[0].0 != path {
        assert((log + extra).drop_first() =~= log.drop_first() + extra);
        ev_append(log.drop_first(), extra, path);
    }
}

pub proof fn ev_push(log: Seq<(Seq<nat>, TEv)>, path: Seq<nat>, ev: TEv)
    ensures
        logged(log.push((path, ev)), path),
        !logged(log, path) ==> ev_at(log.push((path, ev)), path) == Option::Some(ev),
    decreases log.len(),
{
    if log.len() > 0 && log[0].0 != path {
        assert(log.push((path, ev)).drop_first() =~= log.drop_first().push((path, ev)));
        ev_push(log.drop_first(), path, ev);
    }
}

pub proof fn offset_append(
    log: Seq<(Seq<nat>, TEv)>,
    offsets: Seq<nat>,
    extra: Seq<(Seq<nat>, TEv)>,
    more: Seq<nat>,
    path: Seq<nat>,
)
    requires
        offsets.len() == log.len(),
        logged(log, path),
    ensures
        offset_at(log + extra, offsets + more, path) == offset_at(log, offsets, path),
    decreases log.len(),
{
    if log.len() > 0 && log[0].0 != path {
        assert((log + extra).drop_first() =~= log.drop_first() + extra);
        assert((offsets + more).drop_first() =~= offsets.drop_first() + more);
        offset_append(log.drop_first(), offsets.drop_first(), extra, more, path);
    }
}

pub proof fn offset_push(
    log: Seq<(Seq<nat>, TEv)>,
    offsets: Seq<nat>,
    path: Seq<nat>,
    ev: TEv,
    off: nat,
)
    requires
        offsets.len() == log.len(),
        !logged(log, path),
    ensures
        offset_at(log.push((path, ev)), offsets.push(off), path) == off,
    decreases log.len(),
{
    if log.len() > 0 {
        assert(log[0].0 != path);
        assert(log.push((path, ev)).drop_first() =~= log.drop_first().push((path, ev)));
        assert(offsets.push(off).drop_first() =~= offsets.drop_first().push(off));
        offset_push(log.drop_first(), offsets.drop_first(), path, ev, off);
    }
}

pub proof fn position_append(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    log: Seq<(Seq<nat>, TEv)>,
    extra: Seq<(Seq<nat>, TEv)>,
    path: Seq<nat>,
)
    requires
        position(db, roots, log, path),
    ensures
        position(db, roots, log + extra, path),
{
    if path.len() > 1 {
        ev_append(log, extra, path.drop_last());
    }
}

pub proof fn raw_append(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    log: Seq<(Seq<nat>, TEv)>,
    offsets: Seq<nat>,
    extra: Seq<(Seq<nat>, TEv)>,
    more: Seq<nat>,
    path: Seq<nat>,
)
    requires
        position(db, roots, log, path),
        offsets.len() == log.len(),
    ensures
        raw_at(db, roots, log + extra, offsets + more, path) == raw_at(
            db,
            roots,
            log,
            offsets,
            path,
        ),
{
    if path.len() > 1 {
        ev_append(log, extra, path.drop_last());
        offset_append(log, offsets, extra, more, path.drop_last());
    }
}

pub proof fn weak_extend(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    log: Seq<(Seq<nat>, TEv)>,
    offsets: Seq<nat>,
    th: Seq<(nat, Term)>,
    extra: Seq<(Seq<nat>, TEv)>,
    more: Seq<nat>,
    s: Seq<(nat, Term)>,
    path: Seq<nat>,
    ev: TEv,
)
    requires
        offsets.len() == log.len(),
        position(db, roots, log, path),
        logged(log, path),
        weak_event(db, roots, log, offsets, th, path, ev),
    ensures
        weak_event(db, roots, log + extra, offsets + more, th + s, path, ev),
{
    raw_append(db, roots, log, offsets, extra, more, path);
    offset_append(log, offsets, extra, more, path);
    let g = raw_at(db, roots, log, offsets, path);
    match ev {
        TEv::Clause(m) => {
            apply_extension(g, shift(db[m as int].head, offset_at(log, offsets, path)), th, s);
        },
        TEv::Naf(t) => {
            if let Term::Comp(_, args) = g {
                let w = choose|w: Seq<(nat, Term)>| apply(t, w) == apply(args[0], th);
                apply_append(t, w, s);
                apply_append(args[0], th, s);
                assert(apply(t, w + s) == apply(args[0], th + s));
            }
        },
    }
}

pub proof fn cert_old_extend(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    log: Seq<(Seq<nat>, TEv)>,
    offsets: Seq<nat>,
    th: Seq<(nat, Term)>,
    extra: Seq<(Seq<nat>, TEv)>,
    more: Seq<nat>,
    s: Seq<(nat, Term)>,
)
    requires
        cert_log(db, roots, log, offsets, th),
    ensures
        forall|i: int|
            0 <= i < log.len() ==> #[trigger] cert_event(
                db,
                roots,
                log + extra,
                offsets + more,
                th + s,
                log[i].0,
                log[i].1,
            ),
{
    assert forall|i: int| 0 <= i < log.len() implies #[trigger] cert_event(
        db,
        roots,
        log + extra,
        offsets + more,
        th + s,
        log[i].0,
        log[i].1,
    ) by {
        assert(cert_event(db, roots, log, offsets, th, log[i].0, log[i].1));
        index_logged(log, i);
        position_append(db, roots, log, extra, log[i].0);
        weak_extend(db, roots, log, offsets, th, extra, more, s, log[i].0, log[i].1);
    }
}

pub proof fn closed_extend(
    db: Seq<DocClause>,
    log: Seq<(Seq<nat>, TEv)>,
    extra: Seq<(Seq<nat>, TEv)>,
    path: Seq<nat>,
    ev: TEv,
)
    requires
        closed_event(db, log, path, ev),
    ensures
        closed_event(db, log + extra, path, ev),
{
    if let TEv::Clause(m) = ev {
        assert forall|i: int| 0 <= i < db[m as int].body.len() implies #[trigger] logged(
            log + extra,
            path.push(i as nat),
        ) by {
            ev_append(log, extra, path.push(i as nat));
        }
    }
}

pub proof fn fresh_position_not_child(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    log: Seq<(Seq<nat>, TEv)>,
    q: Seq<nat>,
    p: Seq<nat>,
    i: nat,
)
    requires
        position(db, roots, log, q),
        p.len() > 0,
        !logged(log, p),
    ensures
        q != p.push(i),
{
    if q == p.push(i) {
        assert(q.drop_last() =~= p);
        assert(logged(log, p));
    }
}

} // verus!
