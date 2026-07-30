cnl_program_record(3).
document(docid('m6-prop-pred'),source_sha256('ec3e558cf7b833e6815c03ce2f81fb8f18e2d5f55be9e0df01b7b734e4ca54f9'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred(helpful,[named('Mary')]),body([])).
clause(fact_id(sentence(2),clause(1)),pred(patient,[named('John')]),body([])).
clause(rule_id(sentence(3),clause(1)),pred(helpful,[var(1)]),body([pred(patient,[var(1)])])).
clause(rule_id(sentence(4),clause(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),pred(helpful,[var(1)])])).
goal(query_id(sentence(5),clause(1)),pred(helpful,[named('John')])).
