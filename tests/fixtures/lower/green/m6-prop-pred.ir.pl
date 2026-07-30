cnl_ir_record(3).
document(docid('m6-prop-pred'),source_sha256('ec3e558cf7b833e6815c03ce2f81fb8f18e2d5f55be9e0df01b7b734e4ca54f9'),ulex(none)).
fact(fact_id(sentence(1),clause(1)),pred(helpful,[named('Mary')]),source(sentence(1),tokens([2,3]))).
fact(fact_id(sentence(2),clause(1)),pred(patient,[named('John')]),source(sentence(2),tokens([2,4]))).
rule(rule_id(sentence(3),clause(1)),pred(helpful,[var(1)]),body([pred(patient,[var(1)])]),source(sentence(3),tokens([2,3,4]))).
rule(rule_id(sentence(4),clause(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),pred(helpful,[var(1)])]),source(sentence(4),tokens([2,4,5,6]))).
query(query_id(sentence(5),clause(1)),pred(helpful,[named('John')]),source(sentence(5),tokens([1,3]))).
