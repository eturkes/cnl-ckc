use super::freshness::*;
use super::frontier::*;
use super::goals::*;
use super::graph::*;
use super::shapes::*;
use super::*;

verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};

pub ghost struct Cert {
    pub theta: Seq<(nat, Term)>,
    pub offsets: Seq<nat>,
}

pub ghost struct Snapshot {
    pub stack: Seq<TGoal>,
    pub fresh: nat,
    pub log: Seq<(Seq<nat>, TEv)>,
}

pub ghost struct Aux {
    pub current: Option<Cert>,
    pub saved: Seq<Option<Cert>>,
}

pub open spec fn snapshot(c: TCfg) -> Snapshot {
    Snapshot { stack: c.stack, fresh: c.fresh, log: c.log }
}

pub open spec fn alt_snapshot(a: TAlt) -> Snapshot {
    match a {
        TAlt::Cl { stack, fresh, log, .. } => Snapshot { stack, fresh, log },
        TAlt::Naf { stack, fresh, log, path, inner, .. } => Snapshot {
            stack: seq![TGoal::Lit(Term::Comp(naf_name(), seq![inner]), 1, path)] + stack,
            fresh,
            log,
        },
    }
}

pub open spec fn slot_valid(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    v: Snapshot,
    cert: Cert,
    g: TGoal,
) -> bool {
    match g {
        TGoal::Lit(t, _, path) => trace_literal(raw_at(db, roots, v.log, cert.offsets, path)) && t
            == apply(raw_at(db, roots, v.log, cert.offsets, path), cert.theta) && nvars(t)
            <= v.fresh,
        TGoal::NafCut(_) => false,
    }
}

pub open spec fn outer_valid(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    v: Snapshot,
    cert: Cert,
) -> bool {
    cert_log(db, roots, v.log, cert.offsets, cert.theta) && unique_log(v.log) && frontier_ok(
        db,
        roots,
        v.log,
        paths(v.stack),
    ) && cover_log(db, roots, v.log, paths(v.stack)) && binds_below(cert.theta, v.fresh) && forall|
        i: int,
    |
        0 <= i < v.stack.len() ==> #[trigger] slot_valid(db, roots, v, cert, v.stack[i])
}

// Inner searches erase certificate state. Their outer NAF alternative retains the frozen snapshot.
pub open spec fn state_valid(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    v: Snapshot,
    state: Option<Cert>,
) -> bool {
    match state {
        Option::Some(cert) => outer_valid(db, roots, v, cert),
        Option::None => in_naf(v.stack),
    }
}

pub open spec fn alt_valid(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    a: TAlt,
    state: Option<Cert>,
    level: nat,
) -> bool {
    state_valid(db, roots, alt_snapshot(a), state) && cuts_below(alt_snapshot(a).stack, level)
}

pub open spec fn saved_valid(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    alts: Seq<TAlt>,
    saved: Seq<Option<Cert>>,
) -> bool {
    alts.len() == saved.len() && forall|i: int|
        0 <= i < alts.len() ==> #[trigger] alt_valid(db, roots, alts[i], saved[i], i as nat)
}

pub open spec fn aux_valid(db: Seq<DocClause>, roots: Seq<Term>, c: TCfg, aux: Aux) -> bool {
    state_valid(db, roots, snapshot(c), aux.current) && cuts_below(c.stack, c.alts.len())
        && saved_valid(db, roots, c.alts, aux.saved)
}

pub open spec fn initial_aux() -> Aux {
    Aux { current: Option::Some(Cert { theta: seq![], offsets: seq![] }), saved: seq![] }
}

pub proof fn outer_no_naf(db: Seq<DocClause>, roots: Seq<Term>, v: Snapshot, cert: Cert)
    requires
        outer_valid(db, roots, v, cert),
    ensures
        !in_naf(v.stack),
{
    if in_naf(v.stack) {
        let i = choose|i: int| 0 <= i < v.stack.len() && v.stack[i] is NafCut;
        assert(slot_valid(db, roots, v, cert, v.stack[i]));
    }
}

pub proof fn saved_prefix(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    alts: Seq<TAlt>,
    saved: Seq<Option<Cert>>,
    n: nat,
)
    requires
        saved_valid(db, roots, alts, saved),
        n <= alts.len(),
    ensures
        saved_valid(db, roots, alts.take(n as int), saved.take(n as int)),
{
    assert forall|i: int| 0 <= i < n implies #[trigger] alt_valid(
        db,
        roots,
        alts.take(n as int)[i],
        saved.take(n as int)[i],
        i as nat,
    ) by {
        assert(alt_valid(db, roots, alts[i], saved[i], i as nat));
    }
}

pub proof fn saved_push(
    db: Seq<DocClause>,
    roots: Seq<Term>,
    alts: Seq<TAlt>,
    saved: Seq<Option<Cert>>,
    a: TAlt,
    state: Option<Cert>,
)
    requires
        saved_valid(db, roots, alts, saved),
        alt_valid(db, roots, a, state, alts.len()),
    ensures
        saved_valid(db, roots, alts.push(a), saved.push(state)),
{
    assert forall|i: int| 0 <= i < alts.push(a).len() implies #[trigger] alt_valid(
        db,
        roots,
        alts.push(a)[i],
        saved.push(state)[i],
        i as nat,
    ) by {
        if i < alts.len() {
            assert(alt_valid(db, roots, alts[i], saved[i], i as nat));
        }
    }
}

pub proof fn initial_valid(db: Seq<DocClause>, roots: Seq<Term>)
    requires
        forall|i: int| 0 <= i < roots.len() ==> #[trigger] positive(roots[i]),
    ensures
        aux_valid(db, roots, roots_cfg(roots), initial_aux()),
{
    let c = roots_cfg(roots);
    let v = snapshot(c);
    let cert = initial_aux().current.unwrap();
    let front = paths(c.stack);
    assert(front =~= Seq::new(roots.len(), |i: int| seq![i as nat]));
    assert forall|i: int, j: int| 0 <= i < j < front.len() implies #[trigger] front[i]
        != #[trigger] front[j] by {
        assert(front[i][0] == i as nat);
        assert(front[j][0] == j as nat);
    }
    assert forall|i: int| 0 <= i < front.len() implies #[trigger] open_position(
        db,
        roots,
        seq![],
        front[i],
    ) by {
        assert(front[i] == seq![i as nat]);
    }
    assert(frontier_ok(db, roots, seq![], front));
    assert forall|i: int| 0 <= i < roots.len() implies #[trigger] covered(
        seq![],
        front,
        seq![i as nat],
    ) by {
        assert(front[i] == seq![i as nat]);
        assert(front.contains(seq![i as nat]));
    }
    assert(cover_log(db, roots, seq![], front));
    assert forall|i: int| 0 <= i < c.stack.len() implies #[trigger] slot_valid(
        db,
        roots,
        v,
        cert,
        c.stack[i],
    ) by {
        nvars_all_index(roots, i);
        assert(raw_at(db, roots, seq![], seq![], seq![i as nat]) == roots[i]);
    }
    assert(outer_valid(db, roots, v, cert));
    assert(cuts_below(c.stack, 0));
}

pub proof fn terminal_certificate(db: Seq<DocClause>, roots: Seq<Term>, c: TCfg, aux: Aux)
    requires
        aux_valid(db, roots, c, aux),
        c.stack.len() == 0,
    ensures
        aux.current is Some,
        cert_log(db, roots, c.log, aux.current.unwrap().offsets, aux.current.unwrap().theta),
        complete_log(db, roots, c.log),
{
    assert(!in_naf(c.stack));
    assert(aux.current is Some);
    let cert = aux.current.unwrap();
    assert(outer_valid(db, roots, snapshot(c), cert));
    assert(paths(c.stack) =~= Seq::<Seq<nat>>::empty());
    cover_empty_complete(db, roots, c.log);
}

} // verus!
