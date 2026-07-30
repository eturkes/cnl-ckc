cnl_program_record(3).
document(docid('m6-disjunction'),source_sha256('1111111111111111111111111111111111111111111111111111111111111111'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred(patient,[named('John')]),body([])).
clause(fact_id(sentence(2),clause(1)),pred(wait,[named('John')]),body([])).
clause(rule_id(sentence(3),clause(1),branch(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),pred(wait,[var(1)])])).
goal(query_id(sentence(4),clause(1)),pred(recover,[named('John')])).
