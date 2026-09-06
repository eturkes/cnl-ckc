use vstd::prelude::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::v1text as text;

verus! {

use crate::k2_term::ETermArena;
use crate::v1_term_impl::{
    parse_answers, parse_doc, parse_query, parse_traces, EParsedV1,
};

proof fn parsed_accepts(bytes: Seq<u8>, parsed: &EParsedV1)
    requires crate::v1_term_impl::parsed_v1_ok(bytes, parsed),
    ensures ckc_spec::v1text::accepts(bytes),
{
    reveal(crate::v1_term_impl::parsed_v1_ok);
    reveal(ckc_spec::v1text::accepts);
    assert(exists|f: ckc_spec::v1text::V1File|
        #[trigger] ckc_spec::v1text::wf_v1(f)
            && ckc_spec::v1text::print_v1(f) == bytes) by {
        let f = parsed@;
        assert(ckc_spec::v1text::wf_v1(f));
        assert(ckc_spec::v1text::print_v1(f) == bytes);
    }
}

pub open spec fn v1_header_name(f: text::V1File) -> Seq<u8> {
    match f {
        text::V1File::Doc(d) => d.docid,
        text::V1File::Query(q) => q.qid,
        text::V1File::Answers(a) => a.qid,
        text::V1File::Traces(t) => t.qid,
    }
}

pub open spec fn v1_header_mark(f: text::V1File) -> u8 {
    match f {
        text::V1File::Doc(_) => 0x70,
        text::V1File::Query(_) => 0x63,
        text::V1File::Answers(_) => 0x61,
        text::V1File::Traces(_) => 0x74,
    }
}

proof fn v1_header_shape(f: text::V1File)
    requires text::wf_v1(f),
    ensures
        text::name_ok(v1_header_name(f)),
        text::print_v1(f).len() >= v1_header_name(f).len() + 4,
        forall|i: int| 0 <= i < v1_header_name(f).len()
            ==> #[trigger] text::print_v1(f)[2 + i] == v1_header_name(f)[i],
        text::print_v1(f)[2 + v1_header_name(f).len() as int]
            == if f is Doc { 0x2eu8 } else { 0x20u8 },
        text::print_v1(f)[3 + v1_header_name(f).len() as int] == v1_header_mark(f),
{
    reveal(text::wf_v1);
    reveal(text::print_v1);
    match f {
        text::V1File::Doc(_) => {
            reveal(text::wf_doc);
            reveal(text::print_doc);
            reveal(text::doc_line1);
            reveal_strlit(".pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.\n");
        },
        text::V1File::Query(_) => {
            reveal(text::wf_query);
            reveal(text::print_query);
            reveal(text::query_line1);
            reveal_strlit(" compiled from ACE question by ace_to_pl question mode; do not edit.\n");
        },
        text::V1File::Answers(_) => {
            reveal(text::wf_answers);
            reveal(text::print_answers);
            reveal(text::answers_line1);
            reveal_strlit(" answered against the loaded composition by ace_to_pl answer mode; do not edit.\n");
        },
        text::V1File::Traces(_) => {
            reveal(text::wf_traces);
            reveal(text::print_traces);
            reveal(text::traces_line1);
            reveal_strlit(" traced against the loaded composition by ace_to_pl trace mode; do not edit.\n");
        },
    }
    reveal_strlit("% ");
    reveal(text::ascii);
    assert forall|i: int| 0 <= i < v1_header_name(f).len()
        implies #[trigger] text::print_v1(f)[2 + i] == v1_header_name(f)[i] by {}
}

proof fn v1_header_class(left: text::V1File, right: text::V1File)
    requires text::wf_v1(left), text::wf_v1(right), text::print_v1(left) == text::print_v1(right),
    ensures match left {
        text::V1File::Doc(_) => right is Doc,
        text::V1File::Query(_) => right is Query,
        text::V1File::Answers(_) => right is Answers,
        text::V1File::Traces(_) => right is Traces,
    },
{
    v1_header_shape(left);
    v1_header_shape(right);
    let l = v1_header_name(left);
    let r = v1_header_name(right);
    reveal(text::name_ok);
    reveal(text::all_in);
    reveal(text::is_lower_b);
    reveal(text::is_digit_b);
    if l.len() < r.len() {
        assert(text::print_v1(right)[2 + l.len() as int] == r[l.len() as int]);
        assert(text::is_lower_b(r[l.len() as int])
            || text::is_digit_b(r[l.len() as int]) || r[l.len() as int] == 0x2d);
        assert(false);
    }
    if r.len() < l.len() {
        assert(text::print_v1(left)[2 + r.len() as int] == l[r.len() as int]);
        assert(text::is_lower_b(l[r.len() as int])
            || text::is_digit_b(l[r.len() as int]) || l[r.len() as int] == 0x2d);
        assert(false);
    }
    assert(l.len() == r.len());
    assert(v1_header_mark(left) == v1_header_mark(right));
}

fn v1_parse_at(
    bytes: &[u8],
    arena: &mut ETermArena,
    at: &mut usize,
) -> (r: Option<EParsedV1>)
    requires crate::k2_term::arena_ok(old(arena)), *old(at) <= bytes@.len(),
    ensures
        crate::k2_term::arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        *old(at) <= *final(at) <= bytes@.len(),
        r.is_some() <==> text::accepts(bytes@),
        r matches Some(p) ==> {
            &&& crate::v1_term_impl::parsed_v1_ok(bytes@, &p)
            &&& crate::v1_term_impl::parsed_metadata_ok(&p)
            &&& p@ == ckc_spec::replay::the_v1(bytes@)
            &&& crate::v1_term_impl::parsed_doc_roots_ok(final(arena).nodes@, &p)
            &&& crate::v1_term_impl::parsed_query_roots_ok(final(arena).nodes@, &p)
        },
{
    let ghost entry_nodes = arena.nodes@;
    let ghost witness = if text::accepts(bytes@) {
        Some(ckc_spec::replay::the_v1(bytes@))
    } else {
        None
    };
    proof {
        if text::accepts(bytes@) {
            reveal(text::accepts);
            reveal(ckc_spec::replay::the_v1);
            assert(witness is Some);
            assert(text::wf_v1(witness.unwrap()));
            assert(text::print_v1(witness.unwrap()) == bytes@);
        }
    }
    let ghost doc_expected = match witness {
        Some(text::V1File::Doc(d)) => Some(d),
        _ => None,
    };
    let ghost query_expected = match witness {
        Some(text::V1File::Query(q)) => Some(q),
        _ => None,
    };
    let ghost answers_expected = match witness {
        Some(text::V1File::Answers(a)) => Some(a),
        _ => None,
    };
    let ghost traces_expected = match witness {
        Some(text::V1File::Traces(t)) => Some(t),
        _ => None,
    };

    let doc_result = parse_doc(bytes, arena, Ghost(doc_expected), at);
    if doc_result.is_some() {
        let parsed = doc_result.unwrap();
        proof {
            parsed_accepts(bytes@, &parsed);
            v1_header_class(parsed@, witness.unwrap());
            assert(doc_expected is Some);
        }
        return Some(parsed);
    }
    let ghost before_query = arena.nodes@;
    let query_result = parse_query(bytes, arena, Ghost(query_expected), at);
    proof { crate::v1_term_impl::nodes_prefix_transitive(entry_nodes, before_query, arena.nodes@); }
    if query_result.is_some() {
        let parsed = query_result.unwrap();
        proof {
            parsed_accepts(bytes@, &parsed);
            v1_header_class(parsed@, witness.unwrap());
            assert(query_expected is Some);
        }
        return Some(parsed);
    }
    let ghost before_answers = arena.nodes@;
    let answers_result = parse_answers(bytes, arena, Ghost(answers_expected), at);
    proof { crate::v1_term_impl::nodes_prefix_transitive(entry_nodes, before_answers, arena.nodes@); }
    if answers_result.is_some() {
        let parsed = answers_result.unwrap();
        proof {
            parsed_accepts(bytes@, &parsed);
            v1_header_class(parsed@, witness.unwrap());
            assert(answers_expected is Some);
        }
        return Some(parsed);
    }
    let ghost before_traces = arena.nodes@;
    let traces_result = parse_traces(bytes, arena, Ghost(traces_expected), at);
    proof { crate::v1_term_impl::nodes_prefix_transitive(entry_nodes, before_traces, arena.nodes@); }
    if traces_result.is_some() {
        let parsed = traces_result.unwrap();
        proof {
            parsed_accepts(bytes@, &parsed);
            v1_header_class(parsed@, witness.unwrap());
            assert(traces_expected is Some);
        }
        return Some(parsed);
    }

    proof {
        if text::accepts(bytes@) {
            let f = witness.unwrap();
            reveal(text::wf_v1);
            reveal(text::print_v1);
            match f {
                text::V1File::Doc(d) => {
                    assert(doc_expected == Some(d));
                    assert(doc_result is Some);
                    assert(false);
                },
                text::V1File::Query(q) => {
                    assert(query_expected == Some(q));
                    assert(query_result is Some);
                    assert(false);
                },
                text::V1File::Answers(a) => {
                    assert(answers_expected == Some(a));
                    assert(answers_result is Some);
                    assert(false);
                },
                text::V1File::Traces(t) => {
                    assert(traces_expected == Some(t));
                    assert(traces_result is Some);
                    assert(false);
                },
            }
        }
    }
    None
}

pub fn v1_parse(bytes: &[u8], arena: &mut ETermArena) -> (r: Option<EParsedV1>)
    requires crate::k2_term::arena_ok(old(arena)),
    ensures
        crate::k2_term::arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        r.is_some() <==> text::accepts(bytes@),
        r matches Some(p) ==> {
            &&& crate::v1_term_impl::parsed_v1_ok(bytes@, &p)
            &&& crate::v1_term_impl::parsed_metadata_ok(&p)
            &&& p@ == ckc_spec::replay::the_v1(bytes@)
            &&& crate::v1_term_impl::parsed_doc_roots_ok(final(arena).nodes@, &p)
            &&& crate::v1_term_impl::parsed_query_roots_ok(final(arena).nodes@, &p)
        },
{
    let mut at = 0usize;
    v1_parse_at(bytes, arena, &mut at)
}

pub fn v1_check_impl(bytes: &[u8]) -> (r: ckc_spec::v1text::EV1Verdict)
    ensures
        r.is_accept() <==> ckc_spec::v1text::accepts(bytes@),
        r matches ckc_spec::v1text::EV1Verdict::Reject { at } ==> at <= bytes@.len(),
{
    let mut at = 0usize;
    let mut arena = ETermArena { nodes: Vec::new() };
    proof { reveal(crate::k2_term::arena_ok); }
    match v1_parse_at(bytes, &mut arena, &mut at) {
        Some(_) => ckc_spec::v1text::EV1Verdict::Ok,
        None => ckc_spec::v1text::EV1Verdict::Reject { at },
    }
}

} // verus!
