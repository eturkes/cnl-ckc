cnl_program_record(3).
document(docid('temporal-after'),source_sha256('0000000000000000000000000000000000000000000000000000000000000000'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),temporal(after,pred(wait,[named('John')]),anchor(named('Therapy-start'))),body([])).
clause(rule_id(sentence(2),clause(1)),pred(recover,[named('John')]),body([temporal(after,pred(wait,[named('John')]),anchor(named('Therapy-start')))])).
goal(query_id(sentence(3),clause(1)),pred(recover,[named('John')])).
