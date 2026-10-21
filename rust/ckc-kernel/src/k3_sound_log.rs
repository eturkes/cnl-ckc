use super::clear::*;
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

pub proof fn number_ground(t: Term)
    requires
        ckc_spec::arith::number(t) is Some,
    ensures
        ground(t),
        nvars(t) == 0,
{
    if let Term::Comp(_, args) = t {
        assert(args.len() == 2);
        assert(args[0] is Int && args[1] is Int);
        assert(ground_all(args)) by {
            reveal_with_fuel(ground_all, 3);
            assert(args.drop_first()[0] == args[1]);
            assert(args.drop_first().drop_first() =~= Seq::<Term>::empty());
        }
        assert forall|i: int| 0 <= i < args.len() implies nvars(#[trigger] args[i]) <= 0 by {
            assert(i == 0 || i == 1);
        }
        super::naf::nvars_all_upper(args, 0);
    }
}

pub proof fn canon_ground(p: int, q: int)
    ensures
        ground(ckc_spec::arith::canon(p, q)),
        nvars(ckc_spec::arith::canon(p, q)) == 0,
{
    let t = ckc_spec::arith::canon(p, q);
    if let Term::Comp(_, args) = t {
        assert(args.len() == 2 && args[0] is Int && args[1] is Int);
        assert(ground_all(args)) by {
            reveal_with_fuel(ground_all, 3);
            assert(args.drop_first()[0] == args[1]);
            assert(args.drop_first().drop_first() =~= Seq::<Term>::empty());
        }
        assert forall|i: int| 0 <= i < args.len() implies nvars(#[trigger] args[i]) <= 0 by {
            assert(i == 0 || i == 1);
        }
        super::naf::nvars_all_upper(args, 0);
    }
}

// The pair an arithmetic goal computes is ground.
pub proof fn arith_result_ground(t: Term)
    requires
        ckc_spec::arith::arith_result(t) is Some,
    ensures
        nvars(ckc_spec::arith::arith_result(t)->Some_0.0) == 0,
        nvars(ckc_spec::arith::arith_result(t)->Some_0.1) == 0,
        ground(ckc_spec::arith::arith_result(t)->Some_0.0),
        ground(ckc_spec::arith::arith_result(t)->Some_0.1),
{
    if let Term::Comp(name, args) = t {
        let a = ckc_spec::arith::vset(args[0], args[1]);
        let b = ckc_spec::arith::vset(args[2], args[3]);
        let r = if name == ckc_spec::arith::product_name() {
            ckc_spec::arith::vprod(a->Some_0, b->Some_0)
        } else {
            ckc_spec::arith::vquot(a->Some_0, b->Some_0)
        };
        let v = r->Some_0;
        assert(ckc_spec::arith::arith_result(t)->Some_0 == ckc_spec::arith::spell(v));
        match v {
            ckc_spec::arith::VSet::Point(n, d) => canon_ground(n, d),
            ckc_spec::arith::VSet::Lower(n, d, _) => canon_ground(n, d),
            ckc_spec::arith::VSet::Upper(n, d, _) => canon_ground(n, d),
        }
    }
}

// The evaluated instance of an arithmetic goal is ground and holds.
pub proof fn arith_head(name: Seq<u8>, args: Seq<Term>)
    requires
        ckc_spec::arith::arith_result(Term::Comp(name, args)) is Some,
    ensures
        ({
            let (sv, nv) = ckc_spec::arith::arith_result(Term::Comp(name, args))->Some_0;
            let head = Term::Comp(name, args.take(4) + seq![sv, nv]);
            &&& ground(head)
            &&& ckc_spec::compare::holds(head)
            &&& ckc_spec::engine::args_of(head) == args.take(4) + seq![sv, nv]
            &&& args.len() == 6
        }),
{
    let t = Term::Comp(name, args);
    let (sv, nv) = ckc_spec::arith::arith_result(t)->Some_0;
    arith_result_ground(t);
    number_ground(args[1]);
    number_ground(args[3]);
    let hs = args.take(4) + seq![sv, nv];
    let head = Term::Comp(name, hs);
    assert(hs[0] == args[0] && hs[1] == args[1] && hs[2] == args[2] && hs[3] == args[3]);
    assert(hs[4] == sv && hs[5] == nv);
    assert(ckc_spec::arith::arith_result(head) == ckc_spec::arith::arith_result(t));
    assert(ckc_spec::arith::arith_holds(head));
    assert forall|i: int| 0 <= i < hs.len() implies ground(#[trigger] hs[i]) by {
        assert(0 <= i < 6);
    }
    super::naf::ground_all_index(hs);
    reveal(ckc_spec::engine::args_of);
}

// Every argument of a built-in that holds is an atom or a number.
pub proof fn holds_ground(t: Term)
    requires
        ckc_spec::compare::holds(t),
    ensures
        ground(t),
        nvars(t) == 0,
{
    if let Term::Comp(name, args) = t {
        assert(args.len() == 4 || args.len() == 6);
        if ckc_spec::compare::compare_holds(t) {
            if args.len() == 4 {
                number_ground(args[1]);
                number_ground(args[3]);
            } else {
                number_ground(args[1]);
                number_ground(args[4]);
            }
        } else {
            assert(ckc_spec::arith::arith_holds(t));
            let a = ckc_spec::arith::vset(args[0], args[1]);
            let b = ckc_spec::arith::vset(args[2], args[3]);
            assert(a is Some && b is Some);
            number_ground(args[1]);
            number_ground(args[3]);
            let r = if name == ckc_spec::arith::product_name() {
                ckc_spec::arith::vprod(a->Some_0, b->Some_0)
            } else {
                ckc_spec::arith::vquot(a->Some_0, b->Some_0)
            };
            let v = r->Some_0;
            assert((args[4], args[5]) == ckc_spec::arith::spell(v));
            match v {
                ckc_spec::arith::VSet::Point(n, d) => canon_ground(n, d),
                ckc_spec::arith::VSet::Lower(n, d, _) => canon_ground(n, d),
                ckc_spec::arith::VSet::Upper(n, d, _) => canon_ground(n, d),
            }
        }
        assert forall|i: int| 0 <= i < args.len() implies nvars(#[trigger] args[i]) <= 0 by {
            if args.len() == 4 {
                assert(i == 0 || i == 1 || i == 2 || i == 3);
            } else {
                assert(i == 0 || i == 1 || i == 2 || i == 3 || i == 4 || i == 5);
            }
        }
        super::naf::nvars_all_upper(args, 0);
        assert forall|i: int| 0 <= i < args.len() implies ground(#[trigger] args[i]) by {
            if args.len() == 4 {
                assert(i == 0 || i == 1 || i == 2 || i == 3);
            } else {
                assert(i == 0 || i == 1 || i == 2 || i == 3 || i == 4 || i == 5);
            }
        }
        super::naf::ground_all_index(args);
    }
}

pub proof fn builtin_fixed(t: Term, s: Seq<(nat, Term)>)
    requires
        ckc_spec::compare::holds(t),
    ensures
        apply(t, s) == t,
{
    holds_ground(t);
    assert(nvars(t) == 0);
    assert forall|i: int| 0 <= i < s.len() implies !occurs((#[trigger] s[i]).0, t) by {
        if occurs(s[i].0, t) {
            occurs_below(s[i].0, t);
        }
    }
    apply_avoid(t, s);
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
        naf_fixed(ev, s),
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
        TEv::Builtin(t) => {
            builtin_fixed(t, s);
            apply_append(g, th, s);
        },
        TEv::Naf(t) => {
            if let Term::Comp(_, args) = g {
                apply_append(args[0], th, s);
                assert(apply(args[0], th + s) == apply(t, s));
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
        forall|i: int| 0 <= i < log.len() ==> #[trigger] naf_fixed(log[i].1, s),
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
        assert(naf_fixed(log[i].1, s));
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
