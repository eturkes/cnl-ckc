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

    const DOC: &[u8] = concat!(
        "% d.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.\n",
        ":- multifile(guideline_schema_version/1).\n",
        ":- discontiguous(guideline_schema_version/1).\n",
        ":- multifile(guideline_document/3).\n",
        ":- discontiguous(guideline_document/3).\n",
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
        "guideline_schema_version(1).\n",
        "guideline_document(d,ace_sha256(aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa),ulex(none)).\n",
        "% S1: x\n",
        "guideline_event(a,b,c).\n",
        "guideline_event(a,d,c).\n",
    ).as_bytes();
    const QUERIES: [&[u8]; 2] = [concat!(
        "% q compiled from ACE question by ace_to_pl question mode; do not edit.\n",
        "'$guideline_query'(v1,q,ace_sha256(aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa),ulex(none)).\n",
        "% Q1: x\n",
        "'$guideline_query_projection'(goal(guideline_event(a,b,c)),answers([])).\n",
    ).as_bytes(), concat!(
        "% q compiled from ACE question by ace_to_pl question mode; do not edit.\n",
        "'$guideline_query'(v1,q,ace_sha256(aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa),ulex(none)).\n",
        "% Q1: x\n",
        "'$guideline_query_projection'(goal(guideline_event(a,d,c)),answers([])).\n",
    ).as_bytes()];
    // Raw query SHA-256 values: the shell normally supplies these to v1_answer.
    const QUERY_SHA256: [&[u8]; 2] = [
        b"4fd2d61c0ba8557964dcf18b59df04f8a955578a5427c57ca177cb5396471fb7",
        b"37c2e2002241510a0a5779db05f046574993d8759b3f4eb78824a1a9f080b4d5",
    ];

    #[kani::proof]
    #[kani::unwind(949)]
    fn engine_answer_tiny() {
        // One canonical document, two ground facts, one ground goal, two successful queries.
        // No rules/NAF/variables: at most two clause attempts; semantic arguments are atoms.
        // The longest byte scan is DOC.len() = 948; bound = 948 + one exit check.
        // One symbolic goal byte, fixed input addresses; hash bytes follow the same choice.
        let variant: u8 = kani::any();
        kani::assume(variant < 2);
        let mut query = QUERIES[0].to_vec();
        query[247] = b'b' + 2 * variant;
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
