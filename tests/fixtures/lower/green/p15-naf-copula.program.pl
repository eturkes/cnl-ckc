cnl_program_record(2).
document(docid('p15-naf-copula'),source_sha256('1515151515151515151515151515151515151515151515151515151515151515'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred(patient,[named('John')]),body([])).
clause(rule_id(sentence(2),clause(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),naf(pred(smoker,[var(1)]))])).
goal(query_id(sentence(3),clause(1)),pred(recover,[named('John')])).
