cnl_ir_record(3).
document(docid('temporal-before'),source_sha256('0000000000000000000000000000000000000000000000000000000000000000'),ulex(none)).
fact(fact_id(sentence(1),clause(1)),temporal(before,pred(wait,[named('John')]),anchor(named('Therapy-start'))),source(sentence(1),tokens([2,3]))).
rule(rule_id(sentence(2),clause(1)),pred(recover,[named('John')]),body([temporal(before,pred(wait,[named('John')]),anchor(named('Therapy-start')))]),source(sentence(2),tokens([2,3,5]))).
query(query_id(sentence(3),clause(1)),pred(recover,[named('John')]),source(sentence(3),tokens([3]))).
