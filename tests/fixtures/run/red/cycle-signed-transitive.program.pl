cnl_program_record(2).
document(docid('cycle-signed-transitive'),source_sha256('0000000000000000000000000000000000000000000000000000000000000000'),ulex(none)).
clause(rule_id(sentence(1),clause(1)),pred(p,[var(1)]),body([pred(q,[var(1)])])).
clause(rule_id(sentence(2),clause(1)),pred(q,[var(1)]),body([pred(r,[var(1)])])).
clause(rule_id(sentence(3),clause(1)),pred(r,[var(1)]),body([pred(base,[var(1)]),naf(pred(p,[var(1)]))])).
goal(query_id(sentence(4),clause(1)),pred(p,[named(a)])).
