cnl_ir_record(3).
document(docid('m6-of-role'),source_sha256('c46b2b00fa78feb9a460d0abcae4706a8fd4485b3157c9b05aaa6061ba14acb6'),ulex(none)).
fact(fact_id(sentence(1),clause(1)),pred(patient,[named('John')]),source(sentence(1),tokens([2,4]))).
fact(fact_id(sentence(1),clause(2)),pred(of,[named('John'),named('Mary')]),source(sentence(1),tokens([2,5]))).
rule(rule_id(sentence(2),clause(1)),pred(wait,[var(1)]),body([pred(patient,[var(1)]),pred(of,[var(1),named('Mary')])]),source(sentence(2),tokens([2,3,5]))).
query(query_id(sentence(3),clause(1)),pred(wait,[named('John')]),source(sentence(3),tokens([3]))).
