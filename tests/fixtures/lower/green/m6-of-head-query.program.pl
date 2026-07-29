cnl_program_record(3).
document(docid('m6-of-head-query'),source_sha256('0000000000000000000000000000000000000000000000000000000000000000'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred(caregiver,[named('Alice')]),body([])).
clause(fact_id(sentence(1),clause(2)),pred(patient,[named('Bob')]),body([])).
clause(fact_id(sentence(1),clause(3)),pred(of,[named('Alice'),named('Bob')]),body([])).
clause(rule_id(sentence(2),clause(1)),pred(of,[var(1),named('Mary')]),body([pred(patient,[var(1)])])).
goal(query_id(sentence(3),clause(1)),pred(of,[named('Bob'),named('Mary')])).
