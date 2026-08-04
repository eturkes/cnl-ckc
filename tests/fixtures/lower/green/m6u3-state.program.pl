cnl_program_record(3).
document(docid('state-ongoing'),source_sha256('0000000000000000000000000000000000000000000000000000000000000000'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred(patient,[named('John')]),body([])).
clause(fact_id(sentence(1),clause(2)),pred(ongoing,[named('John')]),body([])).
clause(rule_id(sentence(2),clause(1)),pred(wait,[var(1)]),body([pred(patient,[var(1)]),pred(ongoing,[var(1)])])).
goal(query_id(sentence(3),clause(1)),pred(wait,[named('John')])).
