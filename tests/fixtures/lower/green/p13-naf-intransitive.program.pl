cnl_program_record(2).
document(docid('p13-naf-intransitive'),source_sha256('1313131313131313131313131313131313131313131313131313131313131313'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred(patient,[named('John')]),body([])).
clause(rule_id(sentence(2),clause(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),naf(pred(smoke,[var(1)]))])).
goal(query_id(sentence(3),clause(1)),pred(recover,[named('John')])).
