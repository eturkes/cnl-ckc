cnl_program_record(2).
document(docid('slice-naf'),source_sha256('074d6ca7f0e5127e06af01f24a04ce434010ed1e9e80613a85fd9ad81f78ff6e'),ulex(sha256('7be3ff7a729f2d12bbc7d204b70ab93c419f936f6ad751afd8018c3c09cc0bdc'))).
clause(fact_id(sentence(1),clause(1)),pred(patient,[named('John')]),body([])).
clause(rule_id(sentence(2),clause(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),naf(pred(smoke,[var(1)]))])).
goal(query_id(sentence(3),clause(1)),pred(recover,[named('John')])).
