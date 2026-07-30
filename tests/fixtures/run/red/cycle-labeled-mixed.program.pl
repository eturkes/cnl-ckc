cnl_program_record(3).
document(docid('cycle-labeled-mixed'),source_sha256('0000000000000000000000000000000000000000000000000000000000000000'),ulex(none)).
closed_world(exception_id(rule(rule_id(sentence(3),clause(1))),literal(2)),affects(rule_id(sentence(3),clause(1))),predicate_key(p,arity(1))).
clause(rule_id(sentence(1),clause(1)),pred(p,[var(1)]),body([pred(q,[var(1)])])).
clause(rule_id(sentence(2),clause(1)),pred(q,[var(1)]),body([pred(r,[var(1)])])).
clause(rule_id(sentence(3),clause(1)),pred(r,[var(1)]),body([pred(base,[var(1)]),naf(exception_id(rule(rule_id(sentence(3),clause(1))),literal(2)),pred(p,[var(1)]))])).
goal(query_id(sentence(4),clause(1)),pred(p,[named(a)])).
