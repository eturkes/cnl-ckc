cnl_program_record(2).
document(docid('naf-open'),source_sha256('3333333333333333333333333333333333333333333333333333333333333333'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred(patient,[named('café patient')]),body([])).
clause(rule_id(sentence(2),clause(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),naf(pred(smoke,[var(1)]))])).
goal(query_id(sentence(3),clause(1)),pred(recover,[named('café patient')])).
