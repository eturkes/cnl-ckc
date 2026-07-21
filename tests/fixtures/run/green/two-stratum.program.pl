cnl_program_record(2).
document(docid('two-stratum'),source_sha256('5555555555555555555555555555555555555555555555555555555555555555'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred(r,[named(a)]),body([])).
clause(rule_id(sentence(2),clause(1)),pred(t,[var(1)]),body([pred(r,[var(1)]),naf(pred(s,[var(1)]))])).
clause(rule_id(sentence(3),clause(1)),pred(s,[var(1)]),body([pred(r,[var(1)])])).
goal(query_id(sentence(4),clause(1)),pred(t,[named(a)])).
