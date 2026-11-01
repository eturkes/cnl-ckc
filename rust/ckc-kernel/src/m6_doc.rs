use crate::k2_term::ETermArena;
#[cfg(verus_keep_ghost)]
use crate::k2_term::arena_ok;
use crate::m6_custody::{first_mismatch, nonempty_lines, read_clauses, ulex_matches};
use crate::m6_dump::{EDump, parse_dump};
use crate::m6_flat::slot;
use crate::m6_model::*;
use crate::m6_payload::{first_nonground, obligations, print_payload, reject, reject_sym};
use crate::m6_project::project;
use crate::m6_symbols::Sym;
use crate::m6_term::*;
#[cfg(verus_keep_ghost)]
use crate::v1_term_impl::*;
use crate::v1_term_impl::{EBundleMeta, EV1Class};
use ckc_spec::emit as spec;
use ckc_spec::replay::{self, EOut};
use ckc_spec::term::Term;
use ckc_spec::v1text::{self, DocFile, V1File};
use vstd::assert_seqs_equal;
use vstd::prelude::*;

verus! {

pub fn sentence_count(arena: &ETermArena, d: &EDump) -> (out: Option<usize>)
    requires
        arena_ok(arena),
        crate::m6_dump::dump_roots(arena.nodes@, d),
    ensures
        crate::m6_custody::option_nat(out) == spec::sentence_count(d@),
{
    let ss = from_root(arena, d.sentences);
    match items(arena, &ss) {
        Some(ts) => Some(ts.len()),
        None => None,
    }
}

// The scalar fields of a well-formed document (revealing wf_doc here instantiates
// its ordering quantifier along every bundle chain).
proof fn wf_doc_fields(d: DocFile)
    requires
        v1text::wf_doc(d),
    ensures
        v1text::name_ok(d.docid),
        v1text::hex64(d.ace),
        v1text::digest_ok(d.ulex),
        v1text::digest_ok(d.vocabulary),
        d.bundles.len() >= 1,
{
    reveal(v1text::wf_doc);
}

// Each sentence's APE tokens beside its ACE line (spec `srcs_of`).
fn srcs_of(arena: &ETermArena, sentences: &T, lines: &Vec<Vec<u8>>) -> (out: Vec<
    crate::m6_numeral::ESrc,
>)
    requires
        arena_ok(arena),
        valid(arena.nodes@, sentences),
    ensures
        crate::m6_numeral::srcs_valid(arena.nodes@, out@),
        crate::m6_numeral::src_models(out@) == spec::srcs_of(
            sentences@,
            crate::m6_custody::lines_model(lines@),
        ),
        out.len() == lines.len(),
{
    let ss = match items(arena, sentences) {
        Some(ss) => ss,
        None => Vec::new(),
    };
    let ghost sm = match ckc_spec::answers::list_items(sentences@) {
        Option::Some(x) => x,
        Option::None => Seq::<Term>::empty(),
    };
    proof {
        assert(models(ss@) == sm);
    }
    let mut out: Vec<crate::m6_numeral::ESrc> = Vec::new();
    let mut i = 0usize;
    while i < lines.len()
        invariant
            arena_ok(arena),
            valid_all(arena.nodes@, ss@),
            models(ss@) == sm,
            sm == match ckc_spec::answers::list_items(sentences@) {
                Option::Some(x) => x,
                Option::None => Seq::<Term>::empty(),
            },
            i <= lines.len(),
            out.len() == i,
            crate::m6_numeral::srcs_valid(arena.nodes@, out@),
            forall|x: int|
                0 <= x < i ==> (#[trigger] out@[x])@ == spec::srcs_of(
                    sentences@,
                    crate::m6_custody::lines_model(lines@),
                )[x],
        decreases lines.len() - i,
    {
        let ghost want_toks = if (i as int) < sm.len() {
            match ckc_spec::answers::list_items(sm[i as int]) {
                Option::Some(ts) => ts,
                Option::None => Seq::<Term>::empty(),
            }
        } else {
            Seq::<Term>::empty()
        };
        let toks = if i < ss.len() {
            proof {
                assert(valid(arena.nodes@, &ss@[i as int]));
                assert(models(ss@)[i as int] == ss@[i as int]@);
            }
            let o = items(arena, &ss[i]);
            let r = match o {
                Some(ts) => ts,
                None => Vec::new(),
            };
            proof {
                assert(models(r@) == want_toks);
            }
            r
        } else {
            let r = Vec::new();
            proof {
                assert(models(r@) =~= want_toks);
            }
            r
        };
        let line = lines[i].clone();
        proof {
            assert(line@ =~= lines@[i as int]@);
            assert(crate::m6_custody::lines_model(lines@)[i as int] == lines@[i as int]@);
        }
        proof {
            assert(models(toks@) == want_toks);
        }
        let src = crate::m6_numeral::ESrc { toks, line };
        let ghost sv = src@;
        let ghost prev = out@;
        out.push(src);
        proof {
            let want = spec::srcs_of(sentences@, crate::m6_custody::lines_model(lines@));
            assert(want.len() == lines.len());
            assert(want[i as int].toks == want_toks);
            assert(want[i as int].line == lines@[i as int]@);
            assert(sv.toks == want_toks && sv.line == lines@[i as int]@);
            assert(sv == want[i as int]);
            assert forall|x: int| 0 <= x < out.len() implies crate::m6_numeral::src_valid(
                arena.nodes@,
                #[trigger] &out@[x],
            ) by {
                if x < prev.len() {
                    assert(out@[x] == prev[x]);
                }
            }
            assert forall|x: int| 0 <= x < i + 1 implies (#[trigger] out@[x])@ == want[x] by {
                if x < prev.len() {
                    assert(out@[x] == prev[x]);
                } else {
                    assert(out@[x]@ == sv);
                }
            }
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(crate::m6_numeral::src_models(out@) == spec::srcs_of(sentences@, crate::m6_custody::lines_model(lines@)));
    }
    out
}

// Isolate projection/output from intake's byte-parser quantifiers.
pub open spec fn comparison_output(
    docid: Seq<u8>,
    doc: DocFile,
    ps: Seq<spec::Projected>,
    lines: Seq<Seq<u8>>,
) -> replay::Out {
    match spec::certified(doc, ps, docid, lines) {
        Err(why) => spec::certify_reject(docid, why),
        Ok(obs) => replay::ok(replay::print_payload(obs)),
    }
}

#[verifier::spinoff_prover]
pub fn compare_projection(
    arena: &mut ETermArena,
    ms: &Vec<EBundleMeta>,
    actual: &Vec<Clause>,
    ps: &Vec<Projected>,
    lines: &Vec<Vec<u8>>,
    docid: &Vec<u8>,
    prog: &crate::m6_witness::DocProgram,
    doc: Ghost<DocFile>,
) -> (out: EOut)
    requires
        arena_ok(old(arena)),
        crate::m6_witness::program_ok(old(arena).nodes@, prog, replay::doc_db(doc@)),
        bundle_metas_ok(ms@),
        bundle_meta_models(ms@) == doc@.bundles,
        clauses_valid(old(arena).nodes@, actual@),
        clause_models(actual@) == doc_clause_models(bundle_meta_models(ms@)),
        projections_valid(old(arena).nodes@, ps@),
        ms.len() == ps.len(),
        ms.len() == lines.len(),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        out@ == comparison_output(
            docid@,
            doc@,
            project_models(ps@),
            crate::m6_custody::lines_model(lines@),
        ),
{
    hide(spec::first_mismatch);
    hide(spec::obligations);
    hide(spec::first_nonground);
    hide(replay::print_payload);
    hide(ckc_spec::witness::select_doc);
    hide(clauses_valid);
    hide(projected_valid);
    let ghost before_match = arena.nodes@;
    if let Some(s) = first_mismatch(arena, ms, actual, ps, lines) {
        let ghost mid = arena.nodes@;
        let why = slot(arena, &Sym::Clauses, s);
        proof {
            crate::k2_load::prefix_chain(before_match, mid, arena.nodes@);
        }
        return reject(arena, docid, &why);
    }
    proof {
        projections_prefix(before_match, arena.nodes@, ps@);
        crate::m6_witness::program_prefix(before_match, arena.nodes@, prog, replay::doc_db(doc@));
    }
    let ghost before_select = arena.nodes@;
    let selected = crate::m6_witness::select_doc(arena, ps, prog, Ghost(replay::doc_db(doc@)));
    let ghost after_select = arena.nodes@;
    let obs = obligations(arena, &selected, docid);
    let ghost after_obs = arena.nodes@;
    proof {
        crate::k2_load::prefix_chain(before_match, before_select, after_select);
        crate::k2_load::prefix_chain(before_match, after_select, after_obs);
    }
    if let Some(i) = first_nonground(arena, &obs) {
        proof {
            assert(crate::m6_payload::ob_valid(arena.nodes@, &obs@[i as int]));
            reveal(replay::ob_term);
        }
        let oa = args(arena, &obs[i].term);
        proof {
            assert(obs@[i as int].term@ == replay::ob_term(obs@[i as int]@));
            assert(ckc_spec::engine::args_of(replay::ob_term(obs@[i as int]@)).len() == 5);
            assert(oa.len() == 5);
            assert(valid(arena.nodes@, &oa@[1]));
            assert(valid(arena.nodes@, &oa@[2]));
            assert(models(oa@)[2] == oa@[2]@);
            assert(oa@[2]@ == replay::variant(obs@[i as int]@));
        }
        let sentence = oa[1].cp();
        let ghost before = arena.nodes@;
        let variant = oa[2].cp();
        proof {
            prefix(before, arena.nodes@, &sentence);
        }
        let why = c2(arena, &Sym::NongroundObligation, &sentence, &variant);
        proof {
            crate::k2_load::prefix_chain(before_match, after_obs, before);
            crate::k2_load::prefix_chain(before_match, before, arena.nodes@);
        }
        return reject(arena, docid, &why);
    }
    let bytes = print_payload(arena, &obs);
    EOut { rc: 0, out: bytes, err: Vec::new() }
}

// What `certify_doc` derives before `certified`: the document, its projection
// and the ACE lines, else the reject.
pub open spec fn staged(
    ace: Seq<u8>,
    asha: Seq<u8>,
    usha: Option<Seq<u8>>,
    docid: Seq<u8>,
    dump: Seq<u8>,
    pl: Seq<u8>,
    traw: Option<Seq<u8>>,
    tsha: Option<Seq<u8>>,
) -> Result<(DocFile, Seq<spec::Projected>, Seq<Seq<u8>>), Term> {
    if !v1text::accepts(pl) {
        Err(replay::atom("noncanonical"@))
    } else {
        match replay::the_v1(pl) {
            V1File::Doc(doc) => if !spec::dump_accepts(dump) {
                Err(replay::atom("dump_noncanonical"@))
            } else {
                let d = spec::the_dump(dump);
                let lines = spec::nonempty_lines(ace);
                if d.messages != Term::Nil {
                    Err(replay::atom("ape_messages"@))
                } else if spec::sentence_count(d) != Some(lines.len()) {
                    Err(replay::atom("sentence_lines"@))
                } else if doc.docid != docid {
                    Err(replay::atom("docid"@))
                } else if doc.ace != asha {
                    Err(replay::atom("ace_sha256"@))
                } else if doc.ulex != spec::ulex_of(usha) {
                    Err(replay::atom("ulex"@))
                } else if doc.vocabulary != tsha || spec::table_of(traw, tsha) is None {
                    Err(replay::atom("vocabulary"@))
                } else if doc.bundles.len() != lines.len() {
                    Err(replay::atom("bundle_count"@))
                } else {
                    match spec::project(
                        d.drs,
                        docid,
                        lines.len(),
                        spec::table_of(traw, tsha).unwrap(),
                        spec::srcs_of(d.sentences, lines),
                    ) {
                        Err(e) => Err(Term::Comp(v1text::ascii("unsupported"@), seq![e])),
                        Ok(ps) => Ok((doc, ps, lines)),
                    }
                }
            },
            _ => Err(replay::atom("record_shape"@)),
        }
    }
}

pub proof fn staged_certify(
    ace: Seq<u8>,
    asha: Seq<u8>,
    usha: Option<Seq<u8>>,
    docid: Seq<u8>,
    dump: Seq<u8>,
    pl: Seq<u8>,
    traw: Option<Seq<u8>>,
    tsha: Option<Seq<u8>>,
)
    ensures
        spec::certify_doc(ace, asha, usha, docid, dump, pl, traw, tsha) == match staged(
            ace,
            asha,
            usha,
            docid,
            dump,
            pl,
            traw,
            tsha,
        ) {
            Err(why) => Err(why),
            Ok((doc, ps, lines)) => spec::certified(doc, ps, docid, lines),
        },
{
}

// A certified-so-far document: its arena, parse, committed clauses, projection,
// ACE lines + program.
pub struct Intake {
    pub arena: ETermArena,
    pub parsed: crate::v1_term_impl::EParsedV1,
    pub actual: Vec<Clause>,
    pub ps: Vec<Projected>,
    pub lines: Vec<Vec<u8>>,
    pub prog: crate::m6_witness::DocProgram,
    pub doc: Ghost<DocFile>,
}

pub open spec fn intake_ok(i: &Intake, docid: Seq<u8>) -> bool {
    &&& arena_ok(&i.arena)
    &&& bundle_metas_ok(i.parsed.bundles@)
    &&& bundle_meta_models(i.parsed.bundles@) == i.doc@.bundles
    &&& clauses_valid(i.arena.nodes@, i.actual@)
    &&& clause_models(i.actual@) == doc_clause_models(bundle_meta_models(i.parsed.bundles@))
    &&& projections_valid(i.arena.nodes@, i.ps@)
    &&& i.parsed.bundles.len() == i.ps.len()
    &&& i.parsed.bundles.len() == i.lines.len()
    &&& i.parsed.docid@ == docid
    &&& crate::m6_witness::program_ok(i.arena.nodes@, &i.prog, replay::doc_db(i.doc@))
}

#[verifier::spinoff_prover]
pub fn intake(
    ace: &[u8],
    asha: &[u8],
    usha: Option<&Vec<u8>>,
    docid: &[u8],
    dump: &[u8],
    pl: &[u8],
    traw: Option<&Vec<u8>>,
    tsha: Option<&Vec<u8>>,
) -> (out: Result<Intake, EOut>)
    ensures
        match out {
            Err(o) => staged(
                ace@,
                asha@,
                spec::opt_view(usha),
                docid@,
                dump@,
                pl@,
                spec::opt_view(traw),
                spec::opt_view(tsha),
            ) matches Err(why) && o@ == spec::certify_reject(docid@, why),
            Ok(i) => intake_ok(&i, docid@) && staged(
                ace@,
                asha@,
                spec::opt_view(usha),
                docid@,
                dump@,
                pl@,
                spec::opt_view(traw),
                spec::opt_view(tsha),
            ) == Ok((i.doc@, project_models(i.ps@), crate::m6_custody::lines_model(i.lines@))),
        },
{
    hide(spec::project);
    // Rejects compare as whole terms; their message bytes stay folded.
    hide(spec::certify_reject);
    hide(clauses_valid);
    hide(projected_valid);
    hide(spec::srcs_of);
    hide(spec::dump_accepts);
    hide(spec::the_dump);
    hide(spec::table_of);
    hide(spec::nonempty_lines);
    hide(spec::sentence_count);
    hide(v1text::accepts);
    hide(replay::the_v1);
    hide(replay::doc_db);
    // wf_doc's ordering quantifier chains along the bundles (q12: rlimit).
    hide(v1text::wf_doc);
    let mut arena = crate::k2_reject::empty_arena();
    let parsed = match crate::v1_impl::v1_parse(pl, &mut arena) {
        Some(p) => p,
        None => return Err(reject_sym(&mut arena, docid, &Sym::Noncanonical)),
    };
    match &parsed.class {
        EV1Class::Doc => {},
        _ => {
            proof {
                reveal(parsed_metadata_ok);
                assert(!(parsed@ is Doc));
            }
            return Err(reject_sym(&mut arena, docid, &Sym::RecordShape));
        },
    }
    let ghost doc = choose|d: DocFile| parsed@ == V1File::Doc(d);
    proof {
        reveal(parsed_metadata_ok);
        assert(parsed@ is Doc);
        assert(parsed@ == V1File::Doc(doc));
        reveal(parsed_v1_ok);
        reveal(v1text::wf_v1);
        wf_doc_fields(doc);
        parsed_doc_roots_elim(arena.nodes@, &parsed);
    }
    let actual = read_clauses(&arena, &parsed.clauses, Ghost(doc_clause_models(doc.bundles)));
    let ghost before_db = arena.nodes@;
    // Each formula variant replays over the document's program (contract ws D5).
    let prog = crate::m6_witness::doc_program(&mut arena, &parsed, Ghost(doc));
    proof {
        clauses_prefix(before_db, arena.nodes@, actual@);
    }
    let ghost before_dump = arena.nodes@;
    let d = match parse_dump(dump, &mut arena) {
        Some(d) => d,
        None => return Err(reject_sym(&mut arena, docid, &Sym::DumpNoncanonical)),
    };
    proof {
        clauses_prefix(before_dump, arena.nodes@, actual@);
    }
    let messages = from_root(&arena, d.messages);
    let lines = nonempty_lines(ace);
    if !is_nil(&arena, &messages) {
        return Err(reject_sym(&mut arena, docid, &Sym::ApeMessages));
    }
    let count = match sentence_count(&arena, &d) {
        Some(n) => n,
        None => return Err(reject_sym(&mut arena, docid, &Sym::SentenceLines)),
    };
    if count != lines.len() {
        return Err(reject_sym(&mut arena, docid, &Sym::SentenceLines));
    }
    if !bytes_eq(&parsed.docid, docid) {
        return Err(reject_sym(&mut arena, docid, &Sym::Docid));
    }
    if !bytes_eq(&parsed.doc_ace, asha) {
        return Err(reject_sym(&mut arena, docid, &Sym::AceSha256));
    }
    if !ulex_matches(&parsed.doc_ulex, usha, Ghost(doc.ulex)) {
        return Err(reject_sym(&mut arena, docid, &Sym::Ulex));
    }
    let tab = match crate::m7_annotate::table_exec(traw, tsha) {
        Some(t) => t,
        None => return Err(reject_sym(&mut arena, docid, &Sym::Vocabulary)),
    };
    if !crate::m7_annotate::vocabulary_ok(&parsed.doc_vocabulary, tsha, Ghost(doc.vocabulary)) {
        return Err(reject_sym(&mut arena, docid, &Sym::Vocabulary));
    }
    if parsed.bundles.len() != lines.len() {
        return Err(reject_sym(&mut arena, docid, &Sym::BundleCount));
    }
    let drs = from_root(&arena, d.drs);
    let sentences = from_root(&arena, d.sentences);
    let srcs = srcs_of(&arena, &sentences, &lines);
    let ghost before_project = arena.nodes@;
    let ps = match project(&mut arena, &drs, &parsed.docid, lines.len(), &tab, &srcs) {
        Ok(ps) => ps,
        Err(e) => {
            let why = c1(&mut arena, &Sym::Unsupported, &e);
            return Err(reject(&arena, docid, &why));
        },
    };
    proof {
        clauses_prefix(before_project, arena.nodes@, actual@);
        crate::m6_witness::program_prefix(before_dump, before_project, &prog, replay::doc_db(doc));
        crate::m6_witness::program_prefix(before_project, arena.nodes@, &prog, replay::doc_db(doc));
    }
    Ok(Intake { arena, parsed, actual, ps, lines, prog, doc: Ghost(doc) })
}

#[verifier::spinoff_prover]
pub fn certify_doc_impl(
    ace: &[u8],
    asha: &[u8],
    usha: Option<&Vec<u8>>,
    docid: &[u8],
    dump: &[u8],
    pl: &[u8],
    traw: Option<&Vec<u8>>,
    tsha: Option<&Vec<u8>>,
) -> (out: EOut)
    ensures
        out@ == spec::certify_doc_output(
            ace@,
            asha@,
            spec::opt_view(usha),
            docid@,
            dump@,
            pl@,
            spec::opt_view(traw),
            spec::opt_view(tsha),
        ),
{
    hide(spec::certified);
    hide(spec::certify_reject);
    hide(staged);
    hide(comparison_output);
    proof {
        staged_certify(
            ace@,
            asha@,
            spec::opt_view(usha),
            docid@,
            dump@,
            pl@,
            spec::opt_view(traw),
            spec::opt_view(tsha),
        );
    }
    match intake(ace, asha, usha, docid, dump, pl, traw, tsha) {
        Err(o) => o,
        Ok(i) => {
            let Intake { mut arena, parsed, actual, ps, lines, prog, doc } = i;
            let out = compare_projection(
                &mut arena,
                &parsed.bundles,
                &actual,
                &ps,
                &lines,
                &parsed.docid,
                &prog,
                doc,
            );
            proof {
                reveal(comparison_output);
            }
            out
        },
    }
}

} // verus!
