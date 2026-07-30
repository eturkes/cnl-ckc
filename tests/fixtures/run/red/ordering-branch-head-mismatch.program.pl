cnl_program_record(3).
document(docid('ordering-branch-head-mismatch'),source_sha256('9999999999999999999999999999999999999999999999999999999999999999'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred(b,[named('John')]),body([])).
clause(rule_id(sentence(3),clause(1),branch(1)),pred(recover,[var(1)]),body([pred(a,[var(1)])])).
clause(rule_id(sentence(3),clause(1),branch(2)),pred(harm,[var(1)]),body([pred(b,[var(1)])])).
goal(query_id(sentence(4),clause(1)),pred(harm,[named('John')])).
