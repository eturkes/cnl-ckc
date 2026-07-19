cnl_program_record(1).
document(docid('name-capture'),source_sha256('0000000000000000000000000000000000000000000000000000000000000000'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred(assertz,[named(a)]),body([])).
clause(fact_id(sentence(2),clause(1)),pred(clause,[named(a)]),body([])).
clause(rule_id(sentence(3),clause(1)),pred(member,[var(1)]),body([pred(assertz,[var(1)]),pred(clause,[var(1)])])).
goal(query_id(sentence(4),clause(1)),pred(member,[named(a)])).
