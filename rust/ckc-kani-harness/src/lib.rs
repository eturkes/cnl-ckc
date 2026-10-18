#[cfg(kani)]
mod harness {
    use ckc_kernel::{ECheck, ESrc, EV1Verdict, contract};

    #[kani::proof]
    #[kani::unwind(8)]
    fn align_two_byte_domain() {
        // One character, no spans; bound = seven trips + one on the two early-error paths.
        let end: char = kani::any();
        kani::assume(end == '\n' || end == 'x');
        let result = contract::align_check(&[end], &[], &[]);
        assert!(matches!(result, ECheck::Err(_)));
    }

    #[kani::proof]
    #[kani::unwind(9)]
    fn reader_v1_check_small() {
        // Eight input bytes: each scan has at most eight trips + one exit check.
        let bytes: [u8; 8] = kani::any();
        for byte in bytes {
            kani::assume(matches!(
                byte,
                b'%' | b'\n' | b'(' | b')' | b'\'' | b'.' | b'a' | b' '
            ));
        }
        if let EV1Verdict::Reject { at } = contract::v1_check(&bytes) {
            assert!(at <= bytes.len());
        }
    }

    #[kani::proof]
    #[kani::unwind(43)]
    fn reader_v1_check_prefix_of_committed() {
        // Fixed prefix = first 40 bytes of guidelines/cdc-2022-opioid/pl/cdc2022-opioid-rec01-imp01.pl.
        // Two tail bytes; each input scan has at most 42 trips + one exit check.
        let tail: [u8; 2] = kani::any();
        for byte in tail {
            kani::assume(matches!(byte, b'\n' | b'(' | b')' | b'\'' | b'.' | b'a'));
        }
        let mut bytes = *b"% cdc2022-opioid-rec01-imp01.pl compiled  ";
        bytes[40] = tail[0];
        bytes[41] = tail[1];
        if let EV1Verdict::Reject { at } = contract::v1_check(&bytes) {
            assert!(at <= bytes.len());
        }
    }

    #[kani::proof]
    #[kani::unwind(1728)]
    fn reader_trace_check_small() {
        // Constant preflight; eight trace bytes cannot form a v1 envelope.
        // The exact custody error keeps derivation unreachable. DOC has 1727 bytes.
        let bytes: [u8; 8] = kani::any();
        for byte in bytes {
            kani::assume(matches!(
                byte,
                b'%' | b'\n' | b'(' | b')' | b'\'' | b'.' | b'a' | b' '
            ));
        }
        const ANSWERS: &[u8] = concat!(
            "% q answered against the loaded composition by ckc queries; do not edit.\n",
            "'$guideline_answers'(v1,q,query_sha256(dd523d5359c8a608a6ac911f7c1734f0fabfd86381bf8a07b6c5a530c6719f0a),result(yes)).\n",
        ).as_bytes();
        let manifest = ESrc::Bytes(b"d\tp\n".to_vec());
        let docs = vec![ESrc::Bytes(DOC.to_vec())];
        let payloads = vec![ESrc::Bytes(Vec::new())];
        let query = ESrc::Bytes(QUERIES[0].to_vec());
        let answers = ESrc::Bytes(ANSWERS.to_vec());
        let trace = ESrc::Bytes(bytes.to_vec());
        let result = contract::v1_trace_check(
            b"m",
            &manifest,
            &docs,
            &payloads,
            &query,
            QUERY_SHA256[0],
            &answers,
            b"b7f9bfbc307f93705715cc723da79426c178316eb66a55579901a112a3115c45",
            &trace,
            &Vec::new(),
        );
        assert_eq!(result.rc, 2);
        assert!(result.out.is_empty());
        assert_eq!(
            result.err.as_slice(),
            b"ace_to_pl_error(check_load,trace_file(noncanonical)).\n"
        );
    }

    const DOC: &[u8] = concat!(
        "% d.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.\n",
        ":- multifile(guideline_schema_version/1).\n",
        ":- discontiguous(guideline_schema_version/1).\n",
        ":- multifile(guideline_document/4).\n",
        ":- discontiguous(guideline_document/4).\n",
        ":- multifile(guideline_entity/4).\n",
        ":- discontiguous(guideline_entity/4).\n",
        ":- multifile(guideline_cardinality/5).\n",
        ":- discontiguous(guideline_cardinality/5).\n",
        ":- multifile(guideline_event/3).\n",
        ":- discontiguous(guideline_event/3).\n",
        ":- multifile(guideline_arg/4).\n",
        ":- discontiguous(guideline_arg/4).\n",
        ":- multifile(guideline_pp/4).\n",
        ":- discontiguous(guideline_pp/4).\n",
        ":- multifile(guideline_property/4).\n",
        ":- discontiguous(guideline_property/4).\n",
        ":- multifile(guideline_operator/3).\n",
        ":- discontiguous(guideline_operator/3).\n",
        ":- multifile(guideline_interval/6).\n",
        ":- discontiguous(guideline_interval/6).\n",
        ":- multifile(guideline_recurrence/4).\n",
        ":- discontiguous(guideline_recurrence/4).\n",
        ":- multifile(guideline_frequency/5).\n",
        ":- discontiguous(guideline_frequency/5).\n",
        ":- multifile(guideline_order/4).\n",
        ":- discontiguous(guideline_order/4).\n",
        ":- multifile(guideline_recurrence_window/7).\n",
        ":- discontiguous(guideline_recurrence_window/7).\n",
        ":- multifile(guideline_range/3).\n",
        ":- discontiguous(guideline_range/3).\n",
        ":- multifile(guideline_grade/5).\n",
        ":- discontiguous(guideline_grade/5).\n",
        ":- multifile(guideline_preference/4).\n",
        ":- discontiguous(guideline_preference/4).\n",
        ":- multifile(guideline_qualifier/4).\n",
        ":- discontiguous(guideline_qualifier/4).\n",
        ":- multifile(guideline_unit/3).\n",
        ":- discontiguous(guideline_unit/3).\n",
        "guideline_schema_version(1).\n",
        "guideline_document(d,ace_sha256(aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa),ulex(none),vocabulary(none)).\n",
        "% S1: x\n",
        "guideline_event(a,b,c).\n",
        "guideline_event(a,d,c).\n",
    ).as_bytes();
    const QUERIES: [&[u8]; 2] = [concat!(
        "% q compiled from ACE question by ace_to_pl question mode; do not edit.\n",
        "'$guideline_query'(v1,q,ace_sha256(aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa),ulex(none),vocabulary(none)).\n",
        "% Q1: x\n",
        "'$guideline_query_projection'(goal(guideline_event(a,b,c)),answers([])).\n",
    ).as_bytes(), concat!(
        "% q compiled from ACE question by ace_to_pl question mode; do not edit.\n",
        "'$guideline_query'(v1,q,ace_sha256(aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa),ulex(none),vocabulary(none)).\n",
        "% Q1: x\n",
        "'$guideline_query_projection'(goal(guideline_event(a,d,c)),answers([])).\n",
    ).as_bytes()];
    // Raw query SHA-256 values: the shell normally supplies these to v1_answer.
    const QUERY_SHA256: [&[u8]; 2] = [
        b"dd523d5359c8a608a6ac911f7c1734f0fabfd86381bf8a07b6c5a530c6719f0a",
        b"d900583f8ae07892e279df2a3fc0e25866db9702b277122efa50dd91d5692614",
    ];

    #[kani::proof]
    #[kani::unwind(1728)]
    fn engine_answer_tiny() {
        // One canonical document, two ground facts, one ground goal, two successful queries.
        // No rules/NAF/variables: at most two clause attempts; semantic arguments are atoms.
        // The longest byte scan is DOC.len() = 1727; bound = 1727 + one exit check.
        // One symbolic goal byte, fixed input addresses; hash bytes follow the same choice.
        let variant: u8 = kani::any();
        kani::assume(variant < 2);
        let mut query = QUERIES[0].to_vec();
        query[264] = b'b' + 2 * variant;
        let mut qsha = [0u8; 64];
        for (index, byte) in qsha.iter_mut().enumerate() {
            *byte = if variant == 0 {
                QUERY_SHA256[0][index]
            } else {
                QUERY_SHA256[1][index]
            };
        }
        let manifest = ESrc::Bytes(b"d\tp\n".to_vec());
        let docs = vec![ESrc::Bytes(DOC.to_vec())];
        let payloads = vec![ESrc::Bytes(Vec::new())];
        let query = ESrc::Bytes(query);
        let result = contract::v1_answer(b"m", &manifest, &docs, &payloads, &query, &qsha);
        assert!(matches!(result.rc, 0..=2));
        if result.rc == 0 {
            assert!(result.out.ends_with(b"\n"));
        }
    }
}
