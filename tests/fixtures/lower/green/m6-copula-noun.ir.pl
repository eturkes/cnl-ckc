cnl_ir_record(3).
document(docid('m6-copula-noun'),source_sha256('decb4fc0822f7971187eae532642508d55a458312af4bbf880166b9254e1917f'),ulex(none)).
fact(fact_id(sentence(1),clause(1)),pred(person,[named('John')]),source(sentence(1),tokens([2,4]))).
rule(rule_id(sentence(2),clause(1)),pred(patient,[var(1)]),body([pred(person,[var(1)])]),source(sentence(2),tokens([2,3,5]))).
query(query_id(sentence(3),clause(1)),pred(patient,[named('John')]),source(sentence(3),tokens([1,4]))).
