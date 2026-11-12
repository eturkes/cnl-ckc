use crate::k2_engine::EClause;
use crate::k2_term::{ENode, ETermArena};
use crate::v1_term_impl::EV1Class;
use ckc_spec::model::EMcInput;
use ckc_spec::term::Term;
use ckc_spec::v1text::DocFile;
#[cfg(verus_keep_ghost)]
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

// Contract mc D1: the checked composition = the documents in load order (each its two records +
// its clauses) + the direct facts (one more document); per clause its document index + ordinal.
pub struct EMcProgram {
    pub clauses: Vec<EClause>,
    pub doc: Vec<usize>,
    pub ordinal: Vec<usize>,
    pub ds: Ghost<Seq<DocFile>>,
    pub fs: Ghost<Seq<Term>>,
}

pub open spec fn nats(v: Seq<usize>) -> Seq<nat> {
    v.map_values(|x: usize| x as nat)
}

// The vectors hold `program_of`'s clause sequence + document index (neither reads the patient
// boundary or the identity).
pub open spec fn mc_program_ok(nodes: Seq<ENode>, p: &EMcProgram) -> bool {
    &&& crate::k2_engine::db_valid(nodes, p.clauses@)
    &&& crate::k2_engine::db_view(nodes, p.clauses@) == ckc_spec::model::program_of(
        p.ds@,
        p.fs@,
        0,
        Seq::empty(),
    ).clauses
    &&& nats(p.doc@) == ckc_spec::model::program_of(p.ds@, p.fs@, 0, Seq::empty()).doc
    &&& p.doc@.len() == p.clauses@.len()
    &&& p.ordinal@.len() == p.clauses@.len()
}

pub proof fn mc_program_ok_prefix(before: Seq<ENode>, after: Seq<ENode>, p: &EMcProgram)
    requires
        before.is_prefix_of(after),
        mc_program_ok(before, p),
    ensures
        mc_program_ok(after, p),
{
    crate::k2_engine::db_models_prefix(before, after, p.clauses@);
}

proof fn doc_index_push(ds: Seq<DocFile>, d: DocFile)
    ensures
        ckc_spec::model::doc_index(ds.push(d)) =~= ckc_spec::model::doc_index(ds) + Seq::new(
            ckc_spec::replay::doc_db(d).len(),
            |_k: int| ds.len(),
        ),
{
    let a = ds.push(d);
    assert(a.drop_last() =~= ds);
    assert(a.last() == d);
}

pub fn load_program(arena: &mut ETermArena, docs: &Vec<Vec<u8>>, facts: &Vec<Vec<u8>>) -> (r:
    Result<EMcProgram, EMcInput>)
    requires
        crate::k2_term::arena_ok(old(arena)),
    ensures
        crate::k2_term::arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        r matches Ok(p) ==> {
            &&& mc_program_ok(final(arena).nodes@, &p)
            &&& ckc_spec::model::docs_decode(ckc_spec::model::bytes_seq(docs@), p.ds@)
            &&& ckc_spec::model::terms_decode(ckc_spec::model::bytes_seq(facts@), p.fs@)
        },
{
    let ghost origin = arena.nodes@;
    let mut p = EMcProgram {
        clauses: Vec::new(),
        doc: Vec::new(),
        ordinal: Vec::new(),
        ds: Ghost(Seq::empty()),
        fs: Ghost(Seq::empty()),
    };
    proof {
        assert(origin.is_prefix_of(arena.nodes@)) by {
            assert(arena.nodes@.take(origin.len() as int) =~= origin);
        }
        let prog = ckc_spec::model::program_of(p.ds@, p.fs@, 0, Seq::empty());
        assert(ckc_spec::replay::db_of(p.ds@) =~= Seq::empty()) by {
            assert(p.ds@.map_values(|d: DocFile| ckc_spec::replay::doc_db(d)) =~= Seq::empty());
        }
        assert_seqs_equal!(crate::k2_engine::db_view(arena.nodes@, p.clauses@) == prog.clauses);
        assert_seqs_equal!(nats(p.doc@) == prog.doc);
    }
    let mut i = 0usize;
    while i < docs.len()
        invariant
            crate::k2_term::arena_ok(arena),
            origin == old(arena).nodes@,
            origin.is_prefix_of(arena.nodes@),
            i <= docs@.len(),
            p.ds@.len() == i,
            p.fs@ == Seq::<Term>::empty(),
            forall|k: int|
                0 <= k < i ==> ckc_spec::model::decode_doc(#[trigger] docs@[k]@) == Some(p.ds@[k]),
            mc_program_ok(arena.nodes@, &p),
        decreases docs.len() - i,
    {
        let ghost before = arena.nodes@;
        let parsed = crate::v1_impl::v1_parse(&docs[i], arena);
        proof {
            crate::k2_load::prefix_chain(origin, before, arena.nodes@);
            crate::k2_engine::db_models_prefix(before, arena.nodes@, p.clauses@);
        }
        let parsed = match parsed {
            Some(parsed) => parsed,
            None => return Err(EMcInput::Document(i)),
        };
        proof {
            reveal(crate::v1_term_impl::parsed_metadata_ok);
            reveal(ckc_spec::v1text::accepts);
            reveal(ckc_spec::replay::the_v1);
            assert(ckc_spec::v1text::accepts(docs@[i as int]@));
            assert(ckc_spec::v1text::wf_v1(parsed@));
        }
        match parsed.class {
            EV1Class::Doc => {},
            _ => return Err(EMcInput::Document(i)),
        }
        let ghost d = choose|d: DocFile| parsed@ == ckc_spec::v1text::V1File::Doc(d);
        proof {
            assert(parsed@ is Doc);
            assert(ckc_spec::v1text::wf_doc(d));
        }
        let ghost mid = arena.nodes@;
        let part = crate::k2_bridge::document_db(arena, &parsed, Ghost(d));
        proof {
            crate::k2_load::prefix_chain(origin, mid, arena.nodes@);
            crate::k2_engine::db_models_prefix(mid, arena.nodes@, p.clauses@);
        }
        let n = part.len();
        let ghost ds0 = p.ds@;
        let ghost view0 = crate::k2_engine::db_view(arena.nodes@, p.clauses@);
        let ghost doc0 = p.doc@;
        let ghost ord0 = p.ordinal@;
        crate::k2_bridge::append_db(arena, &mut p.clauses, part);
        let mut k = 0usize;
        while k < n
            invariant
                k <= n,
                p.doc@.len() == doc0.len() + k,
                forall|j: int| 0 <= j < doc0.len() ==> p.doc@[j] == doc0[j],
                forall|j: int| doc0.len() <= j < p.doc@.len() ==> p.doc@[j] == i,
                p.ordinal@.len() == ord0.len() + k,
            decreases n - k,
        {
            p.doc.push(i);
            p.ordinal.push(k);
            k += 1;
        }
        proof {
            crate::k2_load::db_of_push(ds0, d);
            doc_index_push(ds0, d);
            let prog0 = ckc_spec::model::program_of(ds0, Seq::empty(), 0, Seq::empty());
            let prog1 = ckc_spec::model::program_of(ds0.push(d), Seq::empty(), 0, Seq::empty());
            assert(view0 == prog0.clauses);
            assert(nats(doc0) == prog0.doc);
            assert(n == ckc_spec::replay::doc_db(d).len());
            assert_seqs_equal!(crate::k2_engine::db_view(arena.nodes@, p.clauses@) == prog1.clauses, j => {
                if j < view0.len() {
                    assert(crate::k2_engine::db_view(arena.nodes@, p.clauses@)[j] == view0[j]);
                }
            });
            let di0 = ckc_spec::model::doc_index(ds0);
            let di1 = ckc_spec::model::doc_index(ds0.push(d));
            assert(prog0.doc.len() == di0.len());
            assert(di1.len() == di0.len() + n);
            assert_seqs_equal!(nats(p.doc@) == prog1.doc, j => {
                assert(prog1.doc[j] == di1[j]);
                if j < doc0.len() {
                    assert(nats(p.doc@)[j] == nats(doc0)[j]);
                    assert(nats(doc0)[j] == prog0.doc[j]);
                    assert(prog0.doc[j] == di0[j]);
                } else {
                    assert(nats(p.doc@)[j] == i as nat);
                }
            });
        }
        p.ds = Ghost(ds0.push(d));
        proof {
            assert forall|k2: int| 0 <= k2 < i + 1 implies ckc_spec::model::decode_doc(
                #[trigger] docs@[k2]@,
            ) == Some(p.ds@[k2]) by {
                if k2 < i {
                    assert(p.ds@[k2] == ds0[k2]);
                } else {
                    assert(p.ds@[k2] == d);
                }
            }
        }
        i += 1;
    }
    let mut j = 0usize;
    while j < facts.len()
        invariant
            crate::k2_term::arena_ok(arena),
            origin == old(arena).nodes@,
            origin.is_prefix_of(arena.nodes@),
            j <= facts@.len(),
            p.ds@.len() == docs@.len(),
            forall|k: int|
                0 <= k < docs@.len() ==> ckc_spec::model::decode_doc(#[trigger] docs@[k]@) == Some(
                    p.ds@[k],
                ),
            p.fs@.len() == j,
            forall|k: int|
                0 <= k < j ==> ckc_spec::model::term_decodes(#[trigger] facts@[k]@, p.fs@[k]),
            mc_program_ok(arena.nodes@, &p),
        decreases facts.len() - j,
    {
        let ghost before = arena.nodes@;
        let root = crate::mc_term::decode_term(arena, &facts[j]);
        proof {
            crate::k2_load::prefix_chain(origin, before, arena.nodes@);
            crate::k2_engine::db_models_prefix(before, arena.nodes@, p.clauses@);
        }
        let root = match root {
            Some(root) => root,
            None => return Err(EMcInput::Fact(j)),
        };
        let ghost t = arena@[root as int];
        let ghost fs0 = p.fs@;
        let ghost view0 = crate::k2_engine::db_view(arena.nodes@, p.clauses@);
        let ghost doc0 = p.doc@;
        p.clauses.push(EClause { head: root, body: Vec::new() });
        p.doc.push(docs.len());
        p.ordinal.push(j);
        proof {
            let prog0 = ckc_spec::model::program_of(p.ds@, fs0, 0, Seq::empty());
            let prog1 = ckc_spec::model::program_of(p.ds@, fs0.push(t), 0, Seq::empty());
            let last = p.clauses@.len() - 1;
            assert(crate::k2_engine::body_items_view(arena.nodes@, p.clauses@[last].body@)
                =~= Seq::<ckc_spec::v1text::BodyItem>::empty());
            assert(crate::k2_engine::clause_valid(arena.nodes@, &p.clauses@[last]));
            assert(crate::k2_engine::db_valid(arena.nodes@, p.clauses@)) by {
                assert forall|q: int|
                    0 <= q < p.clauses@.len() implies #[trigger] crate::k2_engine::clause_valid(
                    arena.nodes@,
                    &p.clauses@[q],
                ) by {
                    if q < last {
                        assert(crate::k2_engine::clause_valid(arena.nodes@, &p.clauses@[q]));
                    }
                }
            }
            assert(view0 == prog0.clauses);
            assert(nats(doc0) == prog0.doc);
            assert_seqs_equal!(crate::k2_engine::db_view(arena.nodes@, p.clauses@) == prog1.clauses, q => {
                if q < view0.len() {
                    assert(crate::k2_engine::db_view(arena.nodes@, p.clauses@)[q] == view0[q]);
                }
            });
            let di = ckc_spec::model::doc_index(p.ds@);
            assert(prog0.doc.len() == di.len() + fs0.len());
            assert(prog1.doc.len() == di.len() + fs0.len() + 1);
            assert_seqs_equal!(nats(p.doc@) == prog1.doc, q => {
                if q < doc0.len() {
                    assert(nats(p.doc@)[q] == nats(doc0)[q]);
                    assert(nats(doc0)[q] == prog0.doc[q]);
                    if q < di.len() {
                        assert(prog0.doc[q] == di[q]);
                        assert(prog1.doc[q] == di[q]);
                    } else {
                        assert(prog0.doc[q] == p.ds@.len());
                        assert(prog1.doc[q] == p.ds@.len());
                    }
                } else {
                    assert(q == doc0.len());
                    assert(nats(p.doc@)[q] == docs@.len());
                    assert(prog1.doc[q] == p.ds@.len());
                }
            });
        }
        p.fs = Ghost(fs0.push(t));
        proof {
            assert forall|k: int| 0 <= k < j + 1 implies ckc_spec::model::term_decodes(
                #[trigger] facts@[k]@,
                p.fs@[k],
            ) by {
                if k < j {
                    assert(p.fs@[k] == fs0[k]);
                }
            }
        }
        j += 1;
    }
    proof {
        assert forall|k: int| 0 <= k < docs@.len() implies ckc_spec::model::decode_doc(
            #[trigger] ckc_spec::model::bytes_seq(docs@)[k],
        ) == Some(p.ds@[k]) by {
            assert(ckc_spec::model::bytes_seq(docs@)[k] == docs@[k]@);
        }
        assert forall|k: int| 0 <= k < facts@.len() implies ckc_spec::model::term_decodes(
            #[trigger] ckc_spec::model::bytes_seq(facts@)[k],
            p.fs@[k],
        ) by {
            assert(ckc_spec::model::bytes_seq(facts@)[k] == facts@[k]@);
        }
    }
    Ok(p)
}

} // verus!
