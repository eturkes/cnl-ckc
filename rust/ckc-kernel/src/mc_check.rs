use crate::k2_term::{ENode, ETermArena};
use ckc_spec::model::*;
use ckc_spec::replay::ESrc;
use ckc_spec::term::Term;
use vstd::prelude::*;

verus! {

// Contract mc (staged core): the checker behind `contract::mc_check_model` + `mc_answer`; stages
// in D9 order: decoding (Err), admission, (question,) identity, coverage, claims, closure.
// The clause ids in play: the slice's (one pass over it; ids ≥ n name no clause).
fn active_in(v: &Vec<usize>, n: usize) -> (out: Vec<bool>)
    ensures
        out@.len() == n,
        forall|ci: int| 0 <= ci < n ==> #[trigger] out@[ci] == slice_set(v@).contains(ci as nat),
{
    let mut out: Vec<bool> = Vec::new();
    let mut ci = 0usize;
    while ci < n
        invariant
            ci <= n,
            out@.len() == ci,
            forall|c2: int| 0 <= c2 < ci ==> !(#[trigger] out@[c2]),
        decreases n - ci,
    {
        out.push(false);
        ci += 1;
    }
    let mut k = 0usize;
    while k < v.len()
        invariant
            k <= v@.len(),
            out@.len() == n,
            forall|c2: int|
                0 <= c2 < n ==> (#[trigger] out@[c2]) == exists|k2: int|
                    0 <= k2 < k && v@[k2] == c2,
        decreases v.len() - k,
    {
        let x = v[k];
        if x < n {
            out.set(x, true);
        }
        proof {
            assert forall|c2: int| 0 <= c2 < n implies (#[trigger] out@[c2]) == exists|k2: int|
                0 <= k2 < k + 1 && v@[k2] == c2 by {
                if c2 == x {
                    assert(v@[k as int] == c2);
                }
            }
        }
        k += 1;
    }
    proof {
        let vm = v@.map_values(|x: usize| x as nat);
        vm.to_set_ensures();
        assert forall|ci: int| 0 <= ci < n implies #[trigger] out@[ci] == slice_set(v@).contains(
            ci as nat,
        ) by {
            if out@[ci] {
                let k2 = choose|k2: int| 0 <= k2 < k && v@[k2] == ci;
                assert(vm[k2] == ci as nat);
            }
            if vm.contains(ci as nat) {
                let k2 = choose|k2: int| 0 <= k2 < vm.len() && vm[k2] == ci as nat;
                assert(v@[k2] == ci);
            }
        }
    }
    out
}

// The clause ids in play: every id, or the slice's.
fn active_of(slice: &Option<Vec<usize>>, n: usize) -> (out: Vec<bool>)
    ensures
        out@.len() == n,
        forall|ci: int|
            0 <= ci < n ==> #[trigger] out@[ci] == match *slice {
                Option::Some(v) => slice_set(v@).contains(ci as nat),
                Option::None => true,
            },
{
    match slice {
        Some(v) => active_in(v, n),
        None => {
            let mut out: Vec<bool> = Vec::new();
            let mut ci = 0usize;
            while ci < n
                invariant
                    ci <= n,
                    out@.len() == ci,
                    forall|c2: int| 0 <= c2 < ci ==> #[trigger] out@[c2],
                decreases n - ci,
            {
                out.push(true);
                ci += 1;
            }
            out
        },
    }
}

fn none_sub(n: usize) -> (out: Vec<Option<usize>>)
    ensures
        out@.len() == n,
        forall|y: int| 0 <= y < n ==> #[trigger] out@[y] is None,
{
    let mut out: Vec<Option<usize>> = Vec::new();
    let mut y = 0usize;
    while y < n
        invariant
            y <= n,
            out@.len() == y,
            forall|y2: int| 0 <= y2 < y ==> #[trigger] out@[y2] is None,
        decreases n - y,
    {
        out.push(None);
        y += 1;
    }
    out
}

// The decoded composition: program, proposed model, certificate claims.
pub struct EMcDecoded {
    pub prog: crate::mc_load::EMcProgram,
    pub m: crate::mc_model::EMcModel,
    pub ds: Vec<crate::mc_cert::EMcDClaim>,
    pub c: Ghost<Certificate>,
}

pub open spec fn decoded_ok(nodes: Seq<ENode>, d: &EMcDecoded) -> bool {
    &&& crate::mc_load::mc_program_ok(nodes, &d.prog)
    &&& crate::mc_model::model_ok(nodes, &d.m)
    &&& crate::mc_cert::dcert_ok(nodes, d.ds@, d.c@)
}

pub proof fn decoded_prefix(before: Seq<ENode>, after: Seq<ENode>, d: &EMcDecoded)
    requires
        before.is_prefix_of(after),
        decoded_ok(before, d),
    ensures
        decoded_ok(after, d),
{
    crate::mc_load::mc_program_ok_prefix(before, after, &d.prog);
    crate::mc_model::model_ok_prefix(before, after, &d.m);
    crate::mc_cert::dcert_prefix(before, after, d.ds@, d.c@);
}

// D9 decoding (the Err stage): documents + facts, then model lines, then claims.
fn decode_all(
    arena: &mut ETermArena,
    docs: &Vec<Vec<u8>>,
    facts: &Vec<Vec<u8>>,
    model: &Vec<Vec<u8>>,
    cert: &EMcCertificate,
) -> (out: Result<EMcDecoded, EMcInput>)
    requires
        crate::k2_term::arena_ok(old(arena)),
    ensures
        crate::k2_term::arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out matches Ok(d) ==> {
            &&& decoded_ok(final(arena).nodes@, &d)
            &&& docs_decode(bytes_seq(docs@), d.prog.ds@)
            &&& terms_decode(bytes_seq(facts@), d.prog.fs@)
            &&& terms_decode(bytes_seq(model@), d.m.ms@)
            &&& cert_decodes(cert@, d.c@)
            &&& d.prog.ds@.len() == docs@.len()
            &&& d.m.ms@.len() == model@.len()
        },
{
    let ghost a0 = arena.nodes@;
    let prog = match crate::mc_load::load_program(arena, docs, facts) {
        Ok(p) => p,
        Err(e) => return Err(e),
    };
    let ghost a1 = arena.nodes@;
    let m = match crate::mc_model::decode_model(arena, model) {
        Ok(m) => m,
        Err(i) => {
            proof {
                crate::k2_load::prefix_chain(a0, a1, arena.nodes@);
            }
            return Err(EMcInput::ModelLine(i));
        },
    };
    let ghost a2 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(a0, a1, a2);
        crate::mc_load::mc_program_ok_prefix(a1, a2, &prog);
    }
    let (ds, c) = match crate::mc_cert::decode_cert(arena, cert) {
        Ok(x) => x,
        Err(k) => {
            proof {
                crate::k2_load::prefix_chain(a0, a2, arena.nodes@);
            }
            return Err(EMcInput::Claim(k));
        },
    };
    let ghost a3 = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(a0, a2, a3);
        crate::mc_load::mc_program_ok_prefix(a2, a3, &prog);
        crate::mc_model::model_ok_prefix(a2, a3, &m);
        assert(prog.ds@.len() == docs@.len());
        assert(m.ms@.len() == model@.len());
    }
    Ok(EMcDecoded { prog, m, ds, c })
}

// D9 admission: the whole composition, its patient boundary clamped as program_of does.
fn admission_stage(
    arena: &ETermArena,
    prog: &crate::mc_load::EMcProgram,
    patient: usize,
    n_docs: usize,
    Ghost(identity): Ghost<Seq<u8>>,
) -> (out: Option<EMcOutcome>)
    requires
        crate::k2_term::arena_ok(arena),
        crate::mc_load::mc_program_ok(arena.nodes@, prog),
        prog.ds@.len() == n_docs,
    ensures
        out is None ==> admitted(program_of(prog.ds@, prog.fs@, patient as nat, identity)),
        out matches Some(o) ==> !(o is Verified) && !(o is VerifiedResult),
{
    let pb = if patient <= n_docs {
        patient
    } else {
        n_docs
    };
    match crate::mc_admit::admit_exec(arena, prog, pb, Ghost(identity)) {
        crate::mc_admit::EAdmit::Refused(d, o) => Some(
            EMcOutcome::Refused(EMcStage::Admission, EMcItem::Clause { doc: d, ordinal: o }),
        ),
        crate::mc_admit::EAdmit::TermSize => Some(EMcOutcome::Indeterminate(EMcReason::TermSize)),
        crate::mc_admit::EAdmit::Admitted => {
            proof {
                assert(program_of(prog.ds@, prog.fs@, patient as nat, identity) == program_of(
                    prog.ds@,
                    prog.fs@,
                    pb as nat,
                    identity,
                ));
            }
            None
        },
    }
}

// D9 identity → coverage → claims → closure over p (ids in play = `active`); None = accepted.
fn accept_stages(
    arena: &mut ETermArena,
    d: &EMcDecoded,
    active: &Vec<bool>,
    Ghost(p): Ghost<Program>,
    cert: &EMcCertificate,
    identity: &[u8],
    model: &Vec<Vec<u8>>,
) -> (out: Option<EMcOutcome>)
    requires
        crate::k2_term::arena_ok(old(arena)),
        decoded_ok(old(arena).nodes@, d),
        cert_decodes(cert@, d.c@),
        d.m.ms@.len() == model@.len(),
        p.identity == identity@,
        p.clauses == crate::k2_engine::db_view(old(arena).nodes@, d.prog.clauses@),
        active@.len() == d.prog.clauses@.len(),
        forall|ci: int|
            0 <= ci < active@.len() ==> (#[trigger] active@[ci]) == p.active.contains(ci as nat),
        forall|ci: int|
            0 <= ci < p.clauses.len() ==> ckc_spec::model::g1_clause(#[trigger] p.clauses[ci])
                && ckc_spec::model::safe_clause(p.clauses[ci]),
    ensures
        crate::k2_term::arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out is None ==> accepted(p, d.m.ms@.to_iset(), d.c@),
        out matches Some(o) ==> !(o is Verified) && !(o is VerifiedResult),
{
    let ghost a3 = arena.nodes@;
    let ghost mm = d.m.ms@.to_iset();
    proof {
        assert(a3.is_prefix_of(a3)) by {
            assert(a3.take(a3.len() as int) =~= a3);
        }
    }
    if !crate::k3_front::bytes_equal(&cert.identity, identity) {
        return Some(EMcOutcome::Refused(EMcStage::Identity, EMcItem::Composition));
    }
    match crate::mc_cert::coverage_check(arena, &d.m, &d.ds, d.c) {
        Err(crate::mc_cert::ECov::Unclaimed(i)) => {
            return Some(EMcOutcome::Refused(EMcStage::Coverage, EMcItem::Fact(model[i].clone())));
        },
        Err(crate::mc_cert::ECov::ClaimAt(k)) => {
            return Some(EMcOutcome::Refused(EMcStage::Coverage, EMcItem::ClaimAt(k)));
        },
        Ok(()) => {},
    }
    let mut k = 0usize;
    while k < d.ds.len()
        invariant
            crate::k2_term::arena_ok(arena),
            a3 == old(arena).nodes@,
            a3.is_prefix_of(arena.nodes@),
            decoded_ok(arena.nodes@, d),
            mm == d.m.ms@.to_iset(),
            p.clauses == crate::k2_engine::db_view(arena.nodes@, d.prog.clauses@),
            active@.len() == d.prog.clauses@.len(),
            forall|ci: int|
                0 <= ci < active@.len() ==> (#[trigger] active@[ci]) == p.active.contains(
                    ci as nat,
                ),
            forall|ci: int|
                0 <= ci < p.clauses.len() ==> ckc_spec::model::g1_clause(#[trigger] p.clauses[ci])
                    && ckc_spec::model::safe_clause(p.clauses[ci]),
            k <= d.ds@.len(),
            forall|q: int| 0 <= q < k ==> #[trigger] claim_valid(p, mm, d.c@, q),
        decreases d.ds.len() - k,
    {
        let ghost before = arena.nodes@;
        if !crate::mc_cert::claim_ok(arena, &d.prog, active, &d.m, &d.ds, d.c, Ghost(p), k) {
            proof {
                crate::k2_load::prefix_chain(a3, before, arena.nodes@);
            }
            return Some(EMcOutcome::Refused(EMcStage::Claim, EMcItem::ClaimAt(k)));
        }
        proof {
            crate::k2_load::prefix_chain(a3, before, arena.nodes@);
            decoded_prefix(before, arena.nodes@, d);
            crate::k2_engine::db_models_prefix(before, arena.nodes@, d.prog.clauses@);
        }
        k += 1;
    }
    let mut ci = 0usize;
    while ci < d.prog.clauses.len()
        invariant
            crate::k2_term::arena_ok(arena),
            a3 == old(arena).nodes@,
            a3.is_prefix_of(arena.nodes@),
            decoded_ok(arena.nodes@, d),
            mm == d.m.ms@.to_iset(),
            p.clauses == crate::k2_engine::db_view(arena.nodes@, d.prog.clauses@),
            active@.len() == d.prog.clauses@.len(),
            forall|c2: int|
                0 <= c2 < active@.len() ==> (#[trigger] active@[c2]) == p.active.contains(
                    c2 as nat,
                ),
            forall|c2: int|
                0 <= c2 < p.clauses.len() ==> ckc_spec::model::g1_clause(#[trigger] p.clauses[c2])
                    && ckc_spec::model::safe_clause(p.clauses[c2]),
            ci <= d.prog.clauses@.len(),
            forall|c2: int|
                0 <= c2 < ci && active@[c2] ==> #[trigger] crate::mc_closure::heads_in(
                    mm,
                    p.clauses[c2],
                    0,
                    Map::<nat, Term>::empty(),
                ),
        decreases d.prog.clauses.len() - ci,
    {
        if active[ci] {
            let clause = &d.prog.clauses[ci];
            let ghost cc = p.clauses[ci as int];
            proof {
                assert(crate::k2_engine::clause_valid(arena.nodes@, &d.prog.clauses@[ci as int]));
                assert(cc == crate::k2_engine::clause_view(arena.nodes@, clause));
            }
            let n = match crate::mc_safe::clause_bound(arena, clause) {
                Some(n) => n,
                None => return Some(EMcOutcome::Indeterminate(EMcReason::TermSize)),
            };
            let sub = none_sub(n);
            let ghost before = arena.nodes@;
            proof {
                assert(crate::mc_subst::sub_ok(arena.nodes@, sub@, Map::<nat, Term>::empty()));
                assert(Map::<nat, Term>::empty().dom() =~= crate::mc_lemma::binds_upto(cc, 0));
            }
            match crate::mc_closure::close_items(
                arena,
                clause,
                Ghost(cc),
                n,
                0,
                &sub,
                Ghost(Map::<nat, Term>::empty()),
                &d.m,
            ) {
                Err(root) => {
                    proof {
                        crate::k2_load::prefix_chain(a3, before, arena.nodes@);
                    }
                    let mut text = crate::k2_term::term_line(arena, root);
                    if text.len() >= 2 {
                        text.pop();
                        text.pop();
                    }
                    return Some(
                        EMcOutcome::Refused(
                            EMcStage::Closure,
                            EMcItem::Derivable { clause: ci, head: text },
                        ),
                    );
                },
                Ok(()) => {},
            }
            proof {
                crate::k2_load::prefix_chain(a3, before, arena.nodes@);
                decoded_prefix(before, arena.nodes@, d);
                crate::k2_engine::db_models_prefix(before, arena.nodes@, d.prog.clauses@);
            }
        }
        ci += 1;
    }
    proof {
        assert forall|h: Term| #[trigger] immediate(p, mm, mm).contains(h) implies mm.contains(
            h,
        ) by {
            let (c2, th) = choose|c2: int, th: Map<nat, Term>| #[trigger]
                instance_of(p, c2, th) && body_holds(mm, mm, p.clauses[c2], th) && h == apply(
                    p.clauses[c2].head,
                    th,
                );
            assert(active@[c2]);
            assert(crate::mc_closure::heads_in(mm, p.clauses[c2], 0, Map::<nat, Term>::empty()));
            assert(crate::mc_subst::extends(th, Map::<nat, Term>::empty()));
            assert(crate::mc_closure::items_from(mm, p.clauses[c2], 0, th));
        }
        assert(closed(p, mm, mm));
        assert(d.c@.identity == p.identity);
        assert(accepted(p, mm, d.c@));
    }
    None
}

pub fn check_model_impl(
    docs: &Vec<Vec<u8>>,
    facts: &Vec<Vec<u8>>,
    patient: usize,
    identity: &[u8],
    slice: &Option<Vec<usize>>,
    model: &Vec<Vec<u8>>,
    cert: &EMcCertificate,
) -> (r: Result<EMcOutcome, EMcInput>)
    ensures
        r matches Result::Ok(EMcOutcome::Verified) ==> check_verified(
            bytes_seq(docs@),
            bytes_seq(facts@),
            patient as nat,
            identity@,
            slice_view(*slice),
            bytes_seq(model@),
            cert@,
        ),
{
    let mut arena = crate::k2_reject::empty_arena();
    let d = match decode_all(&mut arena, docs, facts, model, cert) {
        Ok(d) => d,
        Err(e) => return Err(e),
    };
    if let Some(o) = admission_stage(&arena, &d.prog, patient, docs.len(), Ghost(identity@)) {
        return Ok(o);
    }
    let ghost pfull = program_of(d.prog.ds@, d.prog.fs@, patient as nat, identity@);
    let ghost p = under(pfull, slice_view(*slice));
    let active = active_of(slice, d.prog.clauses.len());
    proof {
        let p0 = program_of(d.prog.ds@, d.prog.fs@, 0, Seq::empty());
        assert(p.clauses == p0.clauses);
        assert(p.clauses == crate::k2_engine::db_view(arena.nodes@, d.prog.clauses@));
        assert forall|ci: int| 0 <= ci < active@.len() implies (#[trigger] active@[ci])
            == p.active.contains(ci as nat) by {}
        assert forall|ci: int| 0 <= ci < p.clauses.len() implies ckc_spec::model::g1_clause(
            #[trigger] p.clauses[ci],
        ) && ckc_spec::model::safe_clause(p.clauses[ci]) by {
            assert(admitted_clause(pfull, ci));
        }
    }
    if let Some(o) = accept_stages(&mut arena, &d, &active, Ghost(p), cert, identity, model) {
        return Ok(o);
    }
    proof {
        assert(docs_decode(bytes_seq(docs@), d.prog.ds@));
        assert(terms_decode(bytes_seq(facts@), d.prog.fs@));
        assert(terms_decode(bytes_seq(model@), d.m.ms@));
        assert(cert_decodes(cert@, d.c@));
    }
    Ok(EMcOutcome::Verified)
}

// answer_verified from its witnesses, in a context small enough for the solver to reach both
// existentials through custody's match (the driver's own context left them unproved).
proof fn answer_verified_intro(
    docs: Seq<Seq<u8>>,
    facts: Seq<Seq<u8>>,
    patient: nat,
    identity: Seq<u8>,
    slice: ISet<nat>,
    model: Seq<Seq<u8>>,
    cert: CertText,
    query: ckc_spec::replay::Src,
    result: Seq<u8>,
    qf: ckc_spec::v1text::QueryFile,
    rows: Seq<Term>,
    ds: Seq<ckc_spec::v1text::DocFile>,
    fs: Seq<Term>,
    ms: Seq<Term>,
    c: Certificate,
    t: Term,
)
    requires
        ckc_spec::answers::custody(query) == Result::<
            (ckc_spec::v1text::QueryFile, Seq<Term>),
            ckc_spec::replay::Out,
        >::Ok((qf, rows)),
        safe_question(question_of(qf, rows)),
        docs_decode(docs, ds),
        terms_decode(facts, fs),
        terms_decode(model, ms),
        cert_decodes(cert, c),
        admitted(program_of(ds, fs, patient, identity)),
        accepted(select(program_of(ds, fs, patient, identity), slice), ms.to_iset(), c),
        result_ok(ms.to_iset(), question_of(qf, rows), t),
        ckc_spec::v1text::term_bytes(t) == result,
    ensures
        answer_verified(docs, facts, patient, identity, slice, model, cert, query, result),
{
    assert(exists|t2: Term| #[trigger]
        result_ok(ms.to_iset(), question_of(qf, rows), t2) && ckc_spec::v1text::term_bytes(t2)
            == result);
}

pub fn answer_impl(
    docs: &Vec<Vec<u8>>,
    facts: &Vec<Vec<u8>>,
    patient: usize,
    identity: &[u8],
    slice: &Vec<usize>,
    model: &Vec<Vec<u8>>,
    cert: &EMcCertificate,
    query: &ESrc,
) -> (r: Result<EMcOutcome, EMcInput>)
    ensures
        r matches Result::Ok(EMcOutcome::VerifiedResult(b)) ==> answer_verified(
            bytes_seq(docs@),
            bytes_seq(facts@),
            patient as nat,
            identity@,
            slice_set(slice@),
            bytes_seq(model@),
            cert@,
            query@,
            b@,
        ),
{
    let mut arena = crate::k2_reject::empty_arena();
    let qy = match crate::k2_answers::custody_exec(&mut arena, query) {
        Ok(qy) => qy,
        Err(out) => return Err(EMcInput::Query(out)),
    };
    let ghost a0 = arena.nodes@;
    let ghost rows = crate::k2_engine::root_terms(a0, qy.rows@);
    let ghost q = question_of(qy.file@, rows);
    let ghost qc = question_clause(q);
    proof {
        assert(ckc_spec::answers::custody(query@) == Result::<
            (ckc_spec::v1text::QueryFile, Seq<Term>),
            ckc_spec::replay::Out,
        >::Ok((qy.file@, rows)));
        crate::mc_answer::custody_walk(query@, qy.file@, rows);
        crate::mc_answer::walk_comp(qy.file@.goal);
    }
    let eq = crate::mc_answer::build_question(&mut arena, &qy);
    let ghost a1 = arena.nodes@;
    let d = match decode_all(&mut arena, docs, facts, model, cert) {
        Ok(d) => d,
        Err(e) => return Err(e),
    };
    let ghost a2 = arena.nodes@;
    proof {
        crate::mc_answer::question_ok_prefix(a1, a2, &eq, q);
    }
    if let Some(o) = admission_stage(&arena, &d.prog, patient, docs.len(), Ghost(identity@)) {
        return Ok(o);
    }
    let n = match crate::mc_safe::clause_bound(&arena, &eq.clause) {
        Some(n) => n,
        None => return Ok(EMcOutcome::Indeterminate(EMcReason::TermSize)),
    };
    if let Err(item) = crate::mc_answer::question_check(&arena, &eq, Ghost(q), n) {
        return Ok(EMcOutcome::Refused(EMcStage::Question, item));
    }
    let ghost pfull = program_of(d.prog.ds@, d.prog.fs@, patient as nat, identity@);
    let ghost p = select(pfull, slice_set(slice@));
    let active = active_in(slice, d.prog.clauses.len());
    proof {
        let p0 = program_of(d.prog.ds@, d.prog.fs@, 0, Seq::empty());
        assert(p.clauses == p0.clauses);
        assert(p.clauses == crate::k2_engine::db_view(arena.nodes@, d.prog.clauses@));
        assert forall|ci: int| 0 <= ci < active@.len() implies (#[trigger] active@[ci])
            == p.active.contains(ci as nat) by {}
        assert forall|ci: int| 0 <= ci < p.clauses.len() implies ckc_spec::model::g1_clause(
            #[trigger] p.clauses[ci],
        ) && ckc_spec::model::safe_clause(p.clauses[ci]) by {
            assert(admitted_clause(pfull, ci));
        }
    }
    if let Some(o) = accept_stages(&mut arena, &d, &active, Ghost(p), cert, identity, model) {
        return Ok(o);
    }
    let ghost a3 = arena.nodes@;
    let ghost mm = d.m.ms@.to_iset();
    let sub = none_sub(n);
    let mut out: Vec<usize> = Vec::new();
    proof {
        crate::mc_answer::question_ok_prefix(a2, a3, &eq, q);
        decoded_prefix(a2, a3, &d);
        assert(crate::mc_safe::vars_below(qc, n as nat));
        assert(crate::mc_subst::sub_ok(a3, sub@, Map::<nat, Term>::empty()));
        assert(Map::<nat, Term>::empty().dom() =~= crate::mc_lemma::binds_upto(qc, 0));
        assert forall|k: int| 0 <= k < qc.body.len() implies (#[trigger] qc.body[k]) is Pos
            && qc.body[k]->Pos_0 is Comp by {
            assert(qc.body[k] == ckc_spec::v1text::BodyItem::Pos(q.goals[k]));
        }
        assert(crate::mc_answer::pos_comp(qc));
        assert(crate::mc_answer::outs_ok(a3, out@, mm, qc));
    }
    crate::mc_answer::collect_items(
        &mut arena,
        &eq.clause,
        Ghost(qc),
        n,
        0,
        &sub,
        Ghost(Map::<nat, Term>::empty()),
        &d.m,
        &mut out,
    );
    let ghost a4 = arena.nodes@;
    let ghost s = crate::k2_engine::root_terms(a4, out@);
    proof {
        assert(crate::k2_sort::roots_ok(&arena, out@));
        assert(crate::k2_sort::roots_ground(&arena, out@)) by {
            assert forall|i: int| 0 <= i < out@.len() implies ckc_spec::term::ground(
                arena@[out@[i] as int],
            ) by {
                assert(arena@[out@[i] as int] == a4[out@[i] as int].term@);
            }
        }
    }
    let sorted = crate::k2_sort::sort_unique(&arena, &out);
    let ghost rr = crate::k2_engine::root_terms(a4, sorted@);
    proof {
        crate::k2_aggregate::sort_views(&arena, sorted@);
        crate::k2_aggregate::sort_views(&arena, out@);
        assert(rr == ckc_spec::engine::sort_unique(s));
        crate::mc_order::sort_unique_canonical(s);
        assert forall|t: Term| #[trigger]
            rr.to_iset().contains(t) == answer_terms(mm, q).contains(t) by {
            crate::mc_model::iset_of_seq(rr, t);
            if s.contains(t) {
                let i = choose|i: int| 0 <= i < s.len() && s[i] == t;
                assert(s[i] == a4[out@[i] as int].term@);
                assert(crate::mc_answer::answer_of(mm, qc, a4[out@[i] as int].term@));
            }
            if answer_terms(mm, q).contains(t) {
                let th = choose|th: Map<nat, Term>| #[trigger]
                    valuation(question_clause(q), th) && body_holds(mm, mm, question_clause(q), th)
                        && t == apply(question_clause(q).head, th);
                assert(crate::mc_subst::extends(th, Map::<nat, Term>::empty()));
                assert(crate::mc_closure::items_from(mm, qc, 0, th));
                assert(crate::mc_answer::collected(a4, out@, t));
                let i = choose|i: int|
                    0 <= i < out@.len() && #[trigger] a4[out@[i] as int].term@ == t;
                assert(s[i] == t);
            }
        }
        assert(rr.to_iset() =~= answer_terms(mm, q));
        assert(canonical_answers(rr, mm, q));
    }
    let result = crate::mc_answer::result_root(&mut arena, &sorted, eq.proj.len() > 0);
    let ghost t = arena@[result as int];
    proof {
        assert(q.projection.len() == eq.proj@.len());
        if eq.proj@.len() == 0 {
            if rr.len() > 0 {
                assert(rr.contains(rr[0]));
                crate::mc_model::iset_of_seq(rr, rr[0]);
                assert(answer_terms(mm, q).contains(rr[0]));
            } else {
                assert forall|t2: Term| !#[trigger] answer_terms(mm, q).contains(t2) by {
                    crate::mc_model::iset_of_seq(rr, t2);
                }
            }
        }
        assert(result_ok(mm, q, t));
    }
    let mut bytes = crate::k2_term::term_line(&arena, result);
    proof {
        reveal_strlit(".\n");
        assert(ckc_spec::v1text::ascii(".\n"@).len() == 2);
    }
    bytes.pop();
    bytes.pop();
    proof {
        assert(bytes@ =~= ckc_spec::v1text::term_bytes(t));
        answer_verified_intro(
            bytes_seq(docs@),
            bytes_seq(facts@),
            patient as nat,
            identity@,
            slice_set(slice@),
            bytes_seq(model@),
            cert@,
            query@,
            bytes@,
            qy.file@,
            rows,
            d.prog.ds@,
            d.prog.fs@,
            d.m.ms@,
            d.c@,
            t,
        );
    }
    Ok(EMcOutcome::VerifiedResult(bytes))
}

} // verus!
