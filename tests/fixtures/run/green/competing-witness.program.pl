cnl_program_record(1).
document(docid('competing-witness'),source_sha256('2222222222222222222222222222222222222222222222222222222222222222'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred(p,[named(a)]),body([])).
clause(fact_id(sentence(2),clause(1)),pred(r,[named(a)]),body([])).
clause(rule_id(sentence(3),clause(1)),pred(q,[var(1)]),body([pred(p,[var(1)])])).
clause(rule_id(sentence(4),clause(1)),pred(q,[var(1)]),body([pred(r,[var(1)])])).
goal(query_id(sentence(5),clause(1)),pred(q,[named(a)])).
