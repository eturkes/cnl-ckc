cnl_ir_record(2).
document(docid('p10-wh-prefix'),source_sha256('1010101010101010101010101010101010101010101010101010101010101010'),ulex(none)).
fact(fact_id(sentence(1),clause(1)),pred(patient,[named('John')]),source(sentence(1),tokens([2,4]))).
fact(fact_id(sentence(2),clause(1)),pred(wait,[named('John')]),source(sentence(2),tokens([2]))).
rule(rule_id(sentence(3),clause(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),pred(wait,[var(1)])]),source(sentence(3),tokens([2,4,5]))).
query(query_id(sentence(4),clause(1)),wh(who),pred(recover,[var(1)]),source(sentence(4),tokens([1,2]))).
