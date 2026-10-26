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
    bs: Seq<v1text::Bundle>,
    ps: Seq<spec::Projected>,
    lines: Seq<Seq<u8>>,
) -> replay::Out {
    match spec::first_mismatch(bs, ps, lines, 0) {
        Some(s) => spec::certify_reject(
            docid,
            Term::Comp(v1text::ascii("clauses"@), seq![Term::Int(s as int)]),
        ),
        None => {
            let obs = spec::obligations(ps, docid);
            match spec::first_nonground(obs) {
                Some(o) => spec::certify_reject(
                    docid,
                    Term::Comp(
                        v1text::ascii("nonground_obligation"@),
                        seq![o.s, replay::variant(o)],
                    ),
                ),
                None => replay::ok(replay::print_payload(obs)),
            }
        },
    }
}

#[verifier::spinoff_prover]
fn compare_projection(
    arena: &mut ETermArena,
    ms: &Vec<EBundleMeta>,
    actual: &Vec<Clause>,
    ps: &Vec<Projected>,
    lines: &Vec<Vec<u8>>,
    docid: &Vec<u8>,
) -> (out: EOut)
    requires
        arena_ok(old(arena)),
        bundle_metas_ok(ms@),
        clauses_valid(old(arena).nodes@, actual@),
        clause_models(actual@) == doc_clause_models(bundle_meta_models(ms@)),
        projections_valid(old(arena).nodes@, ps@),
        ms.len() == ps.len(),
        ms.len() == lines.len(),
    ensures
        out@ == comparison_output(
            docid@,
            bundle_meta_models(ms@),
            project_models(ps@),
            crate::m6_custody::lines_model(lines@),
        ),
{
    hide(spec::first_mismatch);
    hide(spec::obligations);
    hide(spec::first_nonground);
    hide(replay::print_payload);
    hide(clauses_valid);
    hide(projected_valid);
    let ghost before_match = arena.nodes@;
    if let Some(s) = first_mismatch(arena, ms, actual, ps, lines) {
        let why = slot(arena, &Sym::Clauses, s);
        return reject(arena, docid, &why);
    }
    proof {
        projections_prefix(before_match, arena.nodes@, ps@);
    }
    let obs = obligations(arena, ps, docid);
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
        return reject(arena, docid, &why);
    }
    let bytes = print_payload(arena, &obs);
    EOut { rc: 0, out: bytes, err: Vec::new() }
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
    hide(spec::project);
    hide(comparison_output);
    hide(clauses_valid);
    hide(projected_valid);
    hide(spec::srcs_of);
    hide(spec::first_mismatch);
    hide(spec::obligations);
    hide(spec::first_nonground);
    hide(spec::dump_accepts);
    hide(spec::the_dump);
    hide(spec::table_of);
    hide(spec::nonempty_lines);
    hide(spec::sentence_count);
    hide(v1text::accepts);
    hide(replay::the_v1);
    hide(replay::print_payload);
    // wf_doc's ordering quantifier chains along the bundles (q12: rlimit).
    hide(v1text::wf_doc);
    let mut arena = crate::k2_reject::empty_arena();
    let parsed = match crate::v1_impl::v1_parse(pl, &mut arena) {
        Some(p) => p,
        None => return reject_sym(&mut arena, docid, &Sym::Noncanonical),
    };
    match &parsed.class {
        EV1Class::Doc => {},
        _ => {
            proof {
                reveal(parsed_metadata_ok);
                assert(!(parsed@ is Doc));
            }
            return reject_sym(&mut arena, docid, &Sym::RecordShape);
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
    let ghost before_dump = arena.nodes@;
    let d = match parse_dump(dump, &mut arena) {
        Some(d) => d,
        None => return reject_sym(&mut arena, docid, &Sym::DumpNoncanonical),
    };
    proof {
        clauses_prefix(before_dump, arena.nodes@, actual@);
    }
    let messages = from_root(&arena, d.messages);
    let lines = nonempty_lines(ace);
    if !is_nil(&arena, &messages) {
        return reject_sym(&mut arena, docid, &Sym::ApeMessages);
    }
    let count = match sentence_count(&arena, &d) {
        Some(n) => n,
        None => return reject_sym(&mut arena, docid, &Sym::SentenceLines),
    };
    if count != lines.len() {
        return reject_sym(&mut arena, docid, &Sym::SentenceLines);
    }
    if !bytes_eq(&parsed.docid, docid) {
        return reject_sym(&mut arena, docid, &Sym::Docid);
    }
    if !bytes_eq(&parsed.doc_ace, asha) {
        return reject_sym(&mut arena, docid, &Sym::AceSha256);
    }
    if !ulex_matches(&parsed.doc_ulex, usha, Ghost(doc.ulex)) {
        return reject_sym(&mut arena, docid, &Sym::Ulex);
    }
    let tab = match crate::m7_annotate::table_exec(traw, tsha) {
        Some(t) => t,
        None => return reject_sym(&mut arena, docid, &Sym::Vocabulary),
    };
    if !crate::m7_annotate::vocabulary_ok(&parsed.doc_vocabulary, tsha, Ghost(doc.vocabulary)) {
        return reject_sym(&mut arena, docid, &Sym::Vocabulary);
    }
    if parsed.bundles.len() != lines.len() {
        return reject_sym(&mut arena, docid, &Sym::BundleCount);
    }
    let drs = from_root(&arena, d.drs);
    let sentences = from_root(&arena, d.sentences);
    let srcs = srcs_of(&arena, &sentences, &lines);
    let ghost before_project = arena.nodes@;
    let ps = match project(&mut arena, &drs, &parsed.docid, lines.len(), &tab, &srcs) {
        Ok(ps) => ps,
        Err(e) => {
            let why = c1(&mut arena, &Sym::Unsupported, &e);
            return reject(&arena, docid, &why);
        },
    };
    proof {
        clauses_prefix(before_project, arena.nodes@, actual@);
    }
    let out = compare_projection(&mut arena, &parsed.bundles, &actual, &ps, &lines, &parsed.docid);
    proof {
        reveal(comparison_output);
    }
    out
}

} // verus!
