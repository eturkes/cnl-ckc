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
use crate::v1_term_impl::EV1Class;
#[cfg(verus_keep_ghost)]
use crate::v1_term_impl::*;
use ckc_spec::emit as spec;
use ckc_spec::replay::{self, EOut};
use ckc_spec::term::Term;
use ckc_spec::v1text::{self, DocFile, V1File};
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

pub fn certify_doc_impl(
    ace: &[u8],
    asha: &[u8],
    usha: Option<&Vec<u8>>,
    docid: &[u8],
    dump: &[u8],
    pl: &[u8],
) -> (out: EOut)
    ensures
        out@ == spec::certify_doc_output(ace@, asha@, spec::opt_view(usha), docid@, dump@, pl@),
{
    hide(spec::project);
    hide(spec::first_mismatch);
    hide(spec::obligations);
    hide(spec::first_nonground);
    hide(replay::print_payload);
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
        reveal(v1text::wf_doc);
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
    if parsed.bundles.len() != lines.len() {
        return reject_sym(&mut arena, docid, &Sym::BundleCount);
    }
    let drs = from_root(&arena, d.drs);
    let ghost before_project = arena.nodes@;
    let ps = match project(&mut arena, &drs, &parsed.docid, lines.len()) {
        Ok(ps) => ps,
        Err(e) => {
            let why = c1(&mut arena, &Sym::Unsupported, &e);
            return reject(&arena, docid, &why);
        },
    };
    proof {
        clauses_prefix(before_project, arena.nodes@, actual@);
    }
    let ghost before_match = arena.nodes@;
    if let Some(s) = first_mismatch(&mut arena, &parsed.bundles, &actual, &ps, &lines) {
        let why = slot(&mut arena, &Sym::Clauses, s);
        return reject(&arena, docid, &why);
    }
    proof {
        projections_prefix(before_match, arena.nodes@, ps@);
    }
    let obs = obligations(&mut arena, &ps, &parsed.docid);
    if let Some(i) = first_nonground(&arena, &obs) {
        proof {
            assert(crate::m6_payload::ob_valid(arena.nodes@, &obs@[i as int]));
            reveal(replay::ob_term);
        }
        let oa = args(&arena, &obs[i].term);
        proof {
            assert(obs@[i as int].term@ == replay::ob_term(obs@[i as int]@));
            assert(ckc_spec::engine::args_of(replay::ob_term(obs@[i as int]@)).len() == 5);
            assert(oa.len() == 5);
            assert(valid(arena.nodes@, &oa@[1]));
        }
        let sentence = oa[1].cp();
        let ghost before = arena.nodes@;
        let variant = slot(&mut arena, &Sym::Variant, 0);
        proof {
            prefix(before, arena.nodes@, &sentence);
        }
        let why = c2(&mut arena, &Sym::NongroundObligation, &sentence, &variant);
        return reject(&arena, docid, &why);
    }
    let bytes = print_payload(&arena, &obs);
    EOut { rc: 0, out: bytes, err: Vec::new() }
}

} // verus!
