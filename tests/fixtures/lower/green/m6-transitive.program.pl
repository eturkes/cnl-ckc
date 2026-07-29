cnl_program_record(3).
document(docid('m6-transitive'),source_sha256('3234b9d178211bd8e791ee9c3e37ddbf8ae02d198c0ea200cf48e9a19737af11'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred(like,[named('John'),named('Mary')]),body([])).
clause(fact_id(sentence(2),clause(1)),pred(patient,[named('John')]),body([])).
clause(rule_id(sentence(3),clause(1)),pred(help,[var(1),named('Mary')]),body([pred(patient,[var(1)]),pred(like,[var(1),named('Mary')])])).
goal(query_id(sentence(4),clause(1)),pred(help,[named('John'),named('Mary')])).
