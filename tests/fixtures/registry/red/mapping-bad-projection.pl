cnl_guideline_mapping(1).
mapping_document('synthetic-guideline','doc-a',ace(relpath('x/doc-a.ace'),ace_sha256('0000000000000000000000000000000000000000000000000000000000000000')),ulex(relpath('x/doc-a.ulex'),ulex_sha256('1111111111111111111111111111111111111111111111111111111111111111'))).
mapping_region('synthetic-guideline','region-a').
mapping_claim('synthetic-guideline','region-a','claim-a',projection(conditional),docid('doc-a'),items([query_id(sentence(1),clause(2)),rule_id(sentence(1),clause(1))]),expected_answer(answer(query_id(sentence(1),clause(2)),pred(applies,[named(patient)]),proved))).
