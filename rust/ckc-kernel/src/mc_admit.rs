use crate::k2_engine::EBodyItem;
use crate::k2_term::ETermArena;
use crate::mc_load::EMcProgram;
use vstd::prelude::*;

verus! {

// Contract mc D3 (MC half): admission = G1 shapes + left-to-right safety + the patient boundary,
// per clause; the first failing clause's (document, ordinal) on refusal.
pub enum EAdmit {
    Admitted,
    Refused(usize, usize),
    TermSize,
}

fn g1_clause_exec(arena: &ETermArena, clause: &crate::k2_engine::EClause) -> (out: bool)
    requires
        crate::k2_term::arena_ok(arena),
        crate::k2_engine::clause_valid(arena.nodes@, clause),
    ensures
        out == ckc_spec::model::g1_clause(crate::k2_engine::clause_view(arena.nodes@, clause)),
{
    let ghost c = crate::k2_engine::clause_view(arena.nodes@, clause);
    if !crate::mc_g1::schema_literal_exec(arena, clause.head) || !crate::mc_g1::g1_literal_exec(
        arena,
        clause.head,
    ) {
        return false;
    }
    let mut j = 0usize;
    while j < clause.body.len()
        invariant
            crate::k2_term::arena_ok(arena),
            crate::k2_engine::clause_valid(arena.nodes@, clause),
            c == crate::k2_engine::clause_view(arena.nodes@, clause),
            ckc_spec::model::schema_literal(c.head),
            ckc_spec::model::g1_literal(c.head),
            j <= clause.body@.len(),
            forall|q: int| 0 <= q < j ==> #[trigger] ckc_spec::model::g1_item(c.body[q]),
        decreases clause.body.len() - j,
    {
        proof {
            assert(crate::k2_engine::body_item_valid(arena.nodes@, &clause.body@[j as int]));
            assert(c.body[j as int] == crate::k2_engine::body_item_view(
                arena.nodes@,
                &clause.body@[j as int],
            ));
        }
        match &clause.body[j] {
            EBodyItem::Pos { root } => {
                if !crate::mc_g1::g1_literal_exec(arena, *root) {
                    proof {
                        assert(c.body[j as int] == ckc_spec::v1text::BodyItem::Pos(
                            arena@[*root as int],
                        ));
                        assert(!ckc_spec::model::g1_item(c.body[j as int]));
                    }
                    return false;
                }
            },
            EBodyItem::Naf { roots } => {
                let ghost gs = crate::k2_engine::root_terms(arena.nodes@, roots@);
                proof {
                    assert(c.body[j as int] == ckc_spec::v1text::BodyItem::Naf(gs));
                }
                let mut k = 0usize;
                while k < roots.len()
                    invariant
                        crate::k2_term::arena_ok(arena),
                        crate::k2_engine::clause_valid(arena.nodes@, clause),
                        c == crate::k2_engine::clause_view(arena.nodes@, clause),
                        j < clause.body@.len(),
                        c.body[j as int] == ckc_spec::v1text::BodyItem::Naf(gs),
                        crate::k2_engine::roots_valid(arena.nodes@, roots@),
                        gs == crate::k2_engine::root_terms(arena.nodes@, roots@),
                        k <= roots@.len(),
                        forall|q: int| 0 <= q < k ==> ckc_spec::model::g1_literal(#[trigger] gs[q]),
                    decreases roots.len() - k,
                {
                    proof {
                        assert(gs[k as int] == arena@[roots@[k as int] as int]);
                    }
                    if !crate::mc_g1::g1_literal_exec(arena, roots[k]) {
                        proof {
                            assert(!ckc_spec::model::g1_literal(gs[k as int]));
                            assert(c.body[j as int]->Naf_0 == gs);
                            assert(k < gs.len());
                            assert(!ckc_spec::model::g1_item(c.body[j as int]));
                        }
                        return false;
                    }
                    k += 1;
                }
            },
        }
        j += 1;
    }
    true
}

// Admission of the whole program: patient boundary pb (≤ the document count).
pub fn admit_exec(
    arena: &ETermArena,
    prog: &EMcProgram,
    pb: usize,
    Ghost(identity): Ghost<Seq<u8>>,
) -> (out: EAdmit)
    requires
        crate::k2_term::arena_ok(arena),
        crate::mc_load::mc_program_ok(arena.nodes@, prog),
        pb <= prog.ds@.len(),
    ensures
        out is Admitted ==> ckc_spec::model::admitted(
            ckc_spec::model::program_of(prog.ds@, prog.fs@, pb as nat, identity),
        ),
{
    let ghost p = ckc_spec::model::program_of(prog.ds@, prog.fs@, pb as nat, identity);
    let ghost p0 = ckc_spec::model::program_of(prog.ds@, prog.fs@, 0, Seq::empty());
    let ns = crate::mc_patient::kb_names(arena, prog, pb);
    proof {
        let pn = ckc_spec::model::program_of(prog.ds@, prog.fs@, pb as nat, Seq::empty());
        assert(p.clauses == p0.clauses && p.doc == p0.doc && p.patient == pb as nat);
        assert(pn.clauses == p.clauses && pn.doc == p.doc && pn.patient == p.patient);
        assert(ckc_spec::model::kb_namespaces(pn) =~= ckc_spec::model::kb_namespaces(p)) by {
            assert forall|n: ckc_spec::term::Term| #[trigger]
                ckc_spec::model::kb_namespaces(pn).contains(n) == ckc_spec::model::kb_namespaces(
                    p,
                ).contains(n) by {
                if ckc_spec::model::kb_namespaces(pn).contains(n) {
                    let ci = choose|ci: int| #[trigger]
                        ckc_spec::model::knowledge(pn, ci) && crate::mc_patient::clause_names(
                            pn.clauses[ci],
                            n,
                        );
                    assert(ckc_spec::model::knowledge(p, ci));
                }
                if ckc_spec::model::kb_namespaces(p).contains(n) {
                    let ci = choose|ci: int| #[trigger]
                        ckc_spec::model::knowledge(p, ci) && crate::mc_patient::clause_names(
                            p.clauses[ci],
                            n,
                        );
                    assert(ckc_spec::model::knowledge(pn, ci));
                }
            }
        }
    }
    let mut ci = 0usize;
    while ci < prog.clauses.len()
        invariant
            crate::k2_term::arena_ok(arena),
            crate::mc_load::mc_program_ok(arena.nodes@, prog),
            p == ckc_spec::model::program_of(prog.ds@, prog.fs@, pb as nat, identity),
            p.clauses == crate::k2_engine::db_view(arena.nodes@, prog.clauses@),
            crate::mc_load::nats(prog.doc@) == p.doc,
            p.patient == pb as nat,
            crate::k2_engine::roots_valid(arena.nodes@, ns@),
            crate::mc_patient::held(arena.nodes@, ns@) == ckc_spec::model::kb_namespaces(p),
            ci <= prog.clauses@.len(),
            forall|q: int| 0 <= q < ci ==> #[trigger] ckc_spec::model::admitted_clause(p, q),
        decreases prog.clauses.len() - ci,
    {
        let clause = &prog.clauses[ci];
        let ghost c = p.clauses[ci as int];
        proof {
            assert(crate::k2_engine::clause_valid(arena.nodes@, &prog.clauses@[ci as int]));
            assert(c == crate::k2_engine::clause_view(arena.nodes@, &prog.clauses@[ci as int]));
            assert(p.doc[ci as int] == prog.doc@[ci as int] as nat);
        }
        if !g1_clause_exec(arena, clause) {
            return EAdmit::Refused(prog.doc[ci], prog.ordinal[ci]);
        }
        match crate::mc_safe::safe_clause_exec(arena, clause) {
            None => return EAdmit::TermSize,
            Some(false) => return EAdmit::Refused(prog.doc[ci], prog.ordinal[ci]),
            Some(true) => {},
        }
        if prog.doc[ci] >= pb {
            if clause.body.len() != 0 || !crate::k2_walk::ground_root(arena, clause.head)
                || !crate::mc_patient::clear_of(arena, clause.head, &ns) {
                return EAdmit::Refused(prog.doc[ci], prog.ordinal[ci]);
            }
            proof {
                assert(ckc_spec::model::patient_ok(p, ci as int));
            }
        }
        proof {
            assert(ckc_spec::model::admitted_clause(p, ci as int));
        }
        ci += 1;
    }
    proof {
        assert(prog.doc@.len() == prog.clauses@.len());
    }
    EAdmit::Admitted
}

} // verus!
