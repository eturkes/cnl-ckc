use super::body::*;
use super::bounded::*;
use super::freshness::*;
use super::frontier::*;
use super::goals::*;
use super::graph::*;
use super::log::*;
use super::shapes::*;
use super::state::*;
use super::unification::*;
use super::*;

verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub open spec fn naf_image(g: Term, th: Seq<(nat, Term)>, inner: Term) -> bool {
    match g {
        Term::Comp(name, args) => name == naf_name() && args.len() == 1 && apply(args[0], th)
            == inner,
        _ => false,
    }
}

pub proof fn naf_slot_image(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    v: Snapshot,
    cert: Cert,
    inner: Term,
    depth: nat,
    path: Seq<nat>,
)
    requires
        slot_valid(
            db,
            roots,
            v,
            cert,
            TGoal::Lit(Term::Comp(naf_name(), seq![inner]), depth, path),
        ),
    ensures
        naf_image(raw_at(db, roots, v.log, cert.offsets, path), cert.theta, inner),
{
    let g = raw_at(db, roots, v.log, cert.offsets, path);
    if let Term::Comp(name, args) = g {
        apply_comp(name, args, cert.theta);
        assert(Term::Comp(name, apply_terms(args, cert.theta)) == Term::Comp(
            naf_name(),
            seq![inner],
        ));
        assert(apply_terms(args, cert.theta)[0] == inner);
    }
}

// This step records only the payload correspondence; finite failure joins at forest close.
pub proof fn record_naf(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    v: Snapshot,
    cert: Cert,
    inner: Term,
    depth: nat,
    path: Seq<nat>,
    rest: Seq<TGoal>,
)
    requires
        outer_valid(db, roots, v, cert),
        v.stack == seq![TGoal::Lit(Term::Comp(naf_name(), seq![inner]), depth, path)] + rest,
    ensures
        outer_valid(
            db,
            roots,
            Snapshot { stack: rest, fresh: v.fresh, log: v.log.push((path, TEv::Naf(inner))) },
            Cert { theta: cert.theta, offsets: cert.offsets.push(0) },
        ),
{
    let ev = TEv::Naf(inner);
    let extra = seq![(path, ev)];
    let more = seq![0nat];
    let next = Snapshot { stack: rest, fresh: v.fresh, log: v.log.push((path, ev)) };
    let nc = Cert { theta: cert.theta, offsets: cert.offsets.push(0) };
    let oldfront = paths(v.stack);
    let tail = paths(rest);
    paths_add(seq![TGoal::Lit(Term::Comp(naf_name(), seq![inner]), depth, path)], rest);
    assert(paths(seq![TGoal::Lit(Term::Comp(naf_name(), seq![inner]), depth, path)]) =~= seq![
        path,
    ]);
    assert(oldfront == seq![path] + tail);
    assert(open_position(db, roots, v.log, oldfront[0]));
    assert(oldfront[0] == path);
    assert(slot_valid(db, roots, v, cert, v.stack[0]));
    naf_slot_image(db, roots, v, cert, inner, depth, path);
    cert_positioned(db, roots, v.log, cert.offsets, cert.theta);
    frontier_replace(db, roots, v.log, path, ev, tail);
    cover_replace(db, roots, v.log, path, ev, tail);
    log_push_unique(v.log, path, ev);
    assert(event_children(db, path, ev) + tail =~= tail);
    assert(v.log + extra =~= next.log);
    assert(cert.offsets + more =~= nc.offsets);
    assert(cert.theta + seq![] =~= cert.theta);
    cert_old_extend(db, roots, v.log, cert.offsets, cert.theta, extra, more, seq![]);
    raw_append(db, roots, v.log, cert.offsets, extra, more, path);
    position_append(db, roots, v.log, extra, path);
    let raw = raw_at(db, roots, v.log, cert.offsets, path);
    if let Term::Comp(_, args) = raw {
        assert(apply(inner, seq![]) == apply(args[0], cert.theta));
    }
    assert(weak_event(db, roots, next.log, nc.offsets, nc.theta, path, ev));
    assert(cert_event(db, roots, next.log, nc.offsets, nc.theta, path, ev));
    assert forall|i: int| 0 <= i < next.log.len() implies #[trigger] cert_event(
        db,
        roots,
        next.log,
        nc.offsets,
        nc.theta,
        next.log[i].0,
        next.log[i].1,
    ) by {
        if i < v.log.len() {
            assert(cert_event(
                db,
                roots,
                v.log + extra,
                cert.offsets + more,
                cert.theta + seq![],
                v.log[i].0,
                v.log[i].1,
            ));
        }
    }
    assert(cert_log(db, roots, next.log, nc.offsets, nc.theta));
    assert forall|i: int| 0 <= i < rest.len() implies #[trigger] slot_valid(
        db,
        roots,
        next,
        nc,
        rest[i],
    ) by {
        assert(v.stack[i + 1] == rest[i]);
        assert(slot_valid(db, roots, v, cert, v.stack[i + 1]));
        assert(open_position(db, roots, v.log, oldfront[i + 1]));
        if let TGoal::Lit(_, _, p) = rest[i] {
            assert(oldfront[i + 1] == p);
            raw_append(db, roots, v.log, cert.offsets, extra, more, p);
        }
    }
    assert(outer_valid(db, roots, next, nc));
}

pub proof fn cert_push(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    log: Seq<(Seq<nat>, TEv)>,
    offsets: Seq<nat>,
    th: Seq<(nat, Term)>,
    path: Seq<nat>,
    ev: TEv,
    off: nat,
    s: Seq<(nat, Term)>,
)
    requires
        cert_log(db, roots, log, offsets, th),
        cert_event(db, roots, log.push((path, ev)), offsets.push(off), th + s, path, ev),
    ensures
        cert_log(db, roots, log.push((path, ev)), offsets.push(off), th + s),
{
    let next = log.push((path, ev));
    let more = offsets.push(off);
    cert_old_extend(db, roots, log, offsets, th, seq![(path, ev)], seq![off], s);
    assert(log + seq![(path, ev)] =~= next);
    assert(offsets + seq![off] =~= more);
    assert forall|i: int| 0 <= i < next.len() implies #[trigger] cert_event(
        db,
        roots,
        next,
        more,
        th + s,
        next[i].0,
        next[i].1,
    ) by {
        if i < log.len() {
            assert(cert_event(
                db,
                roots,
                log + seq![(path, ev)],
                offsets + seq![off],
                th + s,
                log[i].0,
                log[i].1,
            ));
        }
    }
}

pub proof fn slot_extend(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    v: Snapshot,
    cert: Cert,
    next: Snapshot,
    nc: Cert,
    extra: Seq<(Seq<nat>, TEv)>,
    more: Seq<nat>,
    s: Seq<(nat, Term)>,
    g: TGoal,
)
    requires
        slot_valid(db, roots, v, cert, g),
        position(db, roots, v.log, goal_frame(g).2),
        cert.offsets.len() == v.log.len(),
        next.log == v.log + extra,
        nc.offsets == cert.offsets + more,
        nc.theta == cert.theta + s,
        v.fresh <= next.fresh,
        binds_below(s, next.fresh),
    ensures
        slot_valid(db, roots, next, nc, tapply(g, s)),
{
    if let TGoal::Lit(t, _, path) = g {
        raw_append(db, roots, v.log, cert.offsets, extra, more, path);
        apply_append(raw_at(db, roots, v.log, cert.offsets, path), cert.theta, s);
        apply_below(t, s, next.fresh);
    }
}

pub proof fn child_slot(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    next: Snapshot,
    nc: Cert,
    path: Seq<nat>,
    m: nat,
    i: nat,
    off: nat,
    th: Seq<(nat, Term)>,
    s: Seq<(nat, Term)>,
    depth: nat,
)
    requires
        bodies_wf(db),
        m < db.len(),
        i < db[m as int].body.len(),
        path.len() > 0,
        ev_at(next.log, path) == Option::Some(TEv::Clause(m)),
        offset_at(next.log, nc.offsets, path) == off,
        nc.theta == th + s,
        binds_below(th, off),
        binds_below(s, next.fresh),
        off + clause_nvars(db[m as int]) <= next.fresh,
    ensures
        slot_valid(
            db,
            roots,
            next,
            nc,
            TGoal::Lit(apply(item_term(db[m as int].body[i as int], off), s), depth, path.push(i)),
        ),
{
    let it = db[m as int].body[i as int];
    assert(wf_body_item(it));
    child_raw(db, roots, next.log, nc.offsets, path, m, i);
    item_literal(it, off);
    item_fresh(it, off, th);
    apply_append(item_term(it, off), th, s);
    body_bound(db, m, off);
    assert(nvars(item_term(it, off)) <= off + clause_nvars(db[m as int]));
    apply_below(item_term(it, off), s, next.fresh);
}

pub proof fn record_clause(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    v: Snapshot,
    cert: Cert,
    name: Seq<u8>,
    args: Seq<Term>,
    depth: nat,
    path: Seq<nat>,
    rest: Seq<TGoal>,
    m: nat,
    out: Seq<TGoal>,
)
    requires
        bodies_wf(db),
        outer_valid(db, roots, v, cert),
        v.stack == seq![TGoal::Lit(Term::Comp(name, args), depth, path)] + rest,
        m < db.len(),
        lit_fa(db[m as int].head) == Option::Some((name, args.len())),
        tunify(
            zip(args, args_of(shift(db[m as int].head, v.fresh))),
            tbody_goals(db[m as int].body, v.fresh, (depth - 1) as nat, path) + rest,
        ) == TUni::Ok(out),
    ensures
        exists|nc: Cert| #[trigger]
            outer_valid(
                db,
                roots,
                Snapshot {
                    stack: out,
                    fresh: v.fresh + clause_nvars(db[m as int]),
                    log: v.log.push((path, TEv::Clause(m))),
                },
                nc,
            ),
{
    let cl = db[m as int];
    let fresh = v.fresh + clause_nvars(cl);
    let head = shift(cl.head, v.fresh);
    let pairs = zip(args, args_of(head));
    let body = tbody_goals(cl.body, v.fresh, (depth - 1) as nat, path);
    let stack2 = body + rest;
    let oldfront = paths(v.stack);
    let tail = paths(rest);
    paths_add(seq![TGoal::Lit(Term::Comp(name, args), depth, path)], rest);
    assert(paths(seq![TGoal::Lit(Term::Comp(name, args), depth, path)]) =~= seq![path]);
    assert(oldfront == seq![path] + tail);
    assert(open_position(db, roots, v.log, oldfront[0]));
    assert(oldfront[0] == path);
    assert(slot_valid(db, roots, v, cert, v.stack[0]));
    shift_head_fa(cl.head, name, args.len(), v.fresh);
    shift_bound(cl.head, v.fresh);
    assert(head == Term::Comp(name, args_of(head)));
    assert(nvars_all(args) <= fresh);
    assert(nvars_all(args_of(head)) <= fresh);
    pairs_decomp(args, args_of(head), seq![], fresh);
    assert(pairs + seq![] =~= pairs);
    tunify_bounded(pairs, stack2, fresh);
    let s = choose|s: Seq<(nat, Term)>|
        tunifier_witness(pairs, stack2, tunify(pairs, stack2), fresh, s);
    assert(out == tapply_all(stack2, s));
    let ev = TEv::Clause(m);
    let extra = seq![(path, ev)];
    let more = seq![v.fresh];
    let next = Snapshot { stack: out, fresh, log: v.log.push((path, ev)) };
    let nc = Cert { theta: cert.theta + s, offsets: cert.offsets.push(v.fresh) };
    assert(v.log + extra =~= next.log);
    assert(cert.offsets + more =~= nc.offsets);
    cert_positioned(db, roots, v.log, cert.offsets, cert.theta);
    frontier_replace(db, roots, v.log, path, ev, tail);
    cover_replace(db, roots, v.log, path, ev, tail);
    log_push_unique(v.log, path, ev);
    paths_apply(stack2, s);
    paths_add(body, rest);
    tbody_paths(cl.body, v.fresh, (depth - 1) as nat, path);
    assert(paths(next.stack) == event_children(db, path, ev) + tail);
    raw_append(db, roots, v.log, cert.offsets, extra, more, path);
    position_append(db, roots, v.log, extra, path);
    offset_push(v.log, cert.offsets, path, ev, v.fresh);
    ev_push(v.log, path, ev);
    solves_comp(name, args, args_of(head), seq![], s);
    let raw = raw_at(db, roots, v.log, cert.offsets, path);
    apply_fresh(cl.head, v.fresh, cert.theta);
    apply_append(raw, cert.theta, s);
    apply_append(head, cert.theta, s);
    assert(weak_event(db, roots, next.log, nc.offsets, nc.theta, path, ev));
    assert(cert_event(db, roots, next.log, nc.offsets, nc.theta, path, ev));
    cert_push(db, roots, v.log, cert.offsets, cert.theta, path, ev, v.fresh, s);
    binds_mono(cert.theta, v.fresh, fresh);
    binds_append(cert.theta, s, fresh);
    assert forall|i: int| 0 <= i < out.len() implies #[trigger] slot_valid(
        db,
        roots,
        next,
        nc,
        out[i],
    ) by {
        if i < cl.body.len() {
            child_slot(
                db,
                roots,
                next,
                nc,
                path,
                m,
                i as nat,
                v.fresh,
                cert.theta,
                s,
                (depth - 1) as nat,
            );
            assert(out[i] == TGoal::Lit(
                apply(item_term(cl.body[i], v.fresh), s),
                (depth - 1) as nat,
                path.push(i as nat),
            ));
        } else {
            let j = i - cl.body.len();
            assert(v.stack[j + 1] == rest[j]);
            assert(slot_valid(db, roots, v, cert, v.stack[j + 1]));
            assert(open_position(db, roots, v.log, oldfront[j + 1]));
            assert(goal_frame(rest[j]).2 == oldfront[j + 1]);
            slot_extend(db, roots, v, cert, next, nc, extra, more, s, rest[j]);
            assert(out[i] == tapply(rest[j], s));
        }
    }
    assert(outer_valid(db, roots, next, nc));
}

} // verus!
