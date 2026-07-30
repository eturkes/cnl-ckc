cnl_program_record(3).
document(docid('m6-labeled-exception'),source_sha256('2222222222222222222222222222222222222222222222222222222222222222'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred(patient,[named('John')]),body([])).
closed_world(exception_id(rule(rule_id(sentence(3),clause(1))),literal(2)),affects(rule_id(sentence(3),clause(1))),predicate_key(smoker,1)).
clause(rule_id(sentence(2),clause(1)),pred(smoker,[var(1)]),body([pred(patient,[var(1)]),pred(cough,[var(1)])])).
clause(rule_id(sentence(3),clause(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),naf(exception_id(rule(rule_id(sentence(3),clause(1))),literal(2)),pred(smoker,[var(1)]))])).
goal(query_id(sentence(4),clause(1)),pred(recover,[named('John')])).
