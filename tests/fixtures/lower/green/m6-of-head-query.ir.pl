cnl_ir_record(3).
document(docid('m6-of-head-query'),source_sha256('0000000000000000000000000000000000000000000000000000000000000000'),ulex(none)).
fact(fact_id(sentence(1),clause(1)),pred(caregiver,[named('Alice')]),source(sentence(1),tokens([2,4]))).
fact(fact_id(sentence(1),clause(2)),pred(patient,[named('Bob')]),source(sentence(1),tokens([7,9]))).
fact(fact_id(sentence(1),clause(3)),pred(of,[named('Alice'),named('Bob')]),source(sentence(1),tokens([2,5,7]))).
rule(rule_id(sentence(2),clause(1)),pred(of,[var(1),named('Mary')]),body([pred(patient,[var(1)])]),source(sentence(2),tokens([2,4]))).
query(query_id(sentence(3),clause(1)),pred(of,[named('Bob'),named('Mary')]),source(sentence(3),tokens([3]))).
