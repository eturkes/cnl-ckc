cnl_ir_record(3).
document(docid('state-ongoing'),source_sha256('0000000000000000000000000000000000000000000000000000000000000000'),ulex(none)).
fact(fact_id(sentence(1),clause(1)),pred(patient,[named('John')]),source(sentence(1),tokens([2,4]))).
fact(fact_id(sentence(1),clause(2)),pred(ongoing,[named('John')]),source(sentence(1),tokens([2,3]))).
rule(rule_id(sentence(2),clause(1)),pred(wait,[var(1)]),body([pred(patient,[var(1)]),pred(ongoing,[var(1)])]),source(sentence(2),tokens([2,3,4]))).
query(query_id(sentence(3),clause(1)),pred(wait,[named('John')]),source(sentence(3),tokens([3]))).
