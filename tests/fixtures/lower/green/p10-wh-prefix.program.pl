cnl_program_record(2).
document(docid('p10-wh-prefix'),source_sha256('1010101010101010101010101010101010101010101010101010101010101010'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred(patient,[named('John')]),body([])).
clause(fact_id(sentence(2),clause(1)),pred(wait,[named('John')]),body([])).
clause(rule_id(sentence(3),clause(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),pred(wait,[var(1)])])).
goal(query_id(sentence(4),clause(1)),wh(who),pred(recover,[var(1)])).
