use vstd::prelude::*;

verus! {

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

pub fn v1_check_impl(bytes: &[u8]) -> (r: ckc_spec::v1text::EV1Verdict)
    ensures
        r.is_accept() <==> ckc_spec::v1text::accepts(bytes@),
        r matches ckc_spec::v1text::EV1Verdict::Reject { at } ==> at <= bytes@.len(),
{
    let ghost witness = if ckc_spec::v1text::accepts(bytes@) {
        Some(choose|f: ckc_spec::v1text::V1File|
            ckc_spec::v1text::wf_v1(f)
                && ckc_spec::v1text::print_v1(f) == bytes@)
    } else {
        None
    };
    proof {
        if ckc_spec::v1text::accepts(bytes@) {
            reveal(ckc_spec::v1text::accepts);
            assert(witness is Some);
            assert(ckc_spec::v1text::wf_v1(witness.unwrap()));
            assert(ckc_spec::v1text::print_v1(witness.unwrap()) == bytes@);
        }
    }
    let ghost doc_expected = match witness {
        Some(ckc_spec::v1text::V1File::Doc(d)) => Some(d),
        _ => None,
    };
    let ghost query_expected = match witness {
        Some(ckc_spec::v1text::V1File::Query(q)) => Some(q),
        _ => None,
    };
    let ghost answers_expected = match witness {
        Some(ckc_spec::v1text::V1File::Answers(a)) => Some(a),
        _ => None,
    };
    let ghost traces_expected = match witness {
        Some(ckc_spec::v1text::V1File::Traces(t)) => Some(t),
        _ => None,
    };

    let doc_result = parse_doc(bytes, Ghost(doc_expected));
    if doc_result.is_some() {
        let parsed = doc_result.unwrap();
        proof { parsed_accepts(bytes@, &parsed); }
        return ckc_spec::v1text::EV1Verdict::Ok;
    }
    let query_result = parse_query(bytes, Ghost(query_expected));
    if query_result.is_some() {
        let parsed = query_result.unwrap();
        proof { parsed_accepts(bytes@, &parsed); }
        return ckc_spec::v1text::EV1Verdict::Ok;
    }
    let answers_result = parse_answers(bytes, Ghost(answers_expected));
    if answers_result.is_some() {
        let parsed = answers_result.unwrap();
        proof { parsed_accepts(bytes@, &parsed); }
        return ckc_spec::v1text::EV1Verdict::Ok;
    }
    let traces_result = parse_traces(bytes, Ghost(traces_expected));
    if traces_result.is_some() {
        let parsed = traces_result.unwrap();
        proof { parsed_accepts(bytes@, &parsed); }
        return ckc_spec::v1text::EV1Verdict::Ok;
    }

    proof {
        if ckc_spec::v1text::accepts(bytes@) {
            let f = witness.unwrap();
            reveal(ckc_spec::v1text::wf_v1);
            reveal(ckc_spec::v1text::print_v1);
            match f {
                ckc_spec::v1text::V1File::Doc(d) => {
                    assert(doc_expected == Some(d));
                    assert(doc_result is Some);
                    assert(false);
                },
                ckc_spec::v1text::V1File::Query(q) => {
                    assert(query_expected == Some(q));
                    assert(query_result is Some);
                    assert(false);
                },
                ckc_spec::v1text::V1File::Answers(a) => {
                    assert(answers_expected == Some(a));
                    assert(answers_result is Some);
                    assert(false);
                },
                ckc_spec::v1text::V1File::Traces(t) => {
                    assert(traces_expected == Some(t));
                    assert(traces_result is Some);
                    assert(false);
                },
            }
        }
    }
    ckc_spec::v1text::EV1Verdict::Reject { at: 0 }
}

} // verus!
