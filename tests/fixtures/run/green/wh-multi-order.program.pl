cnl_program_record(2).
document(docid('wh-multi-order'),source_sha256('4444444444444444444444444444444444444444444444444444444444444444'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred(patient,[named(a)]),body([])).
clause(fact_id(sentence(2),clause(1)),pred(patient,[named('z z')]),body([])).
clause(rule_id(sentence(3),clause(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)])])).
goal(query_id(sentence(4),clause(1)),wh(who),pred(recover,[var(1)])).
