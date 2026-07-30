cnl_ir_record(3).
document(docid('m6-labeled-exception'),source_sha256('2222222222222222222222222222222222222222222222222222222222222222'),ulex(none)).
fact(fact_id(sentence(1),clause(1)),pred(patient,[named('John')]),source(sentence(1),tokens([2,4]))).
closed_world(exception_id(rule(rule_id(sentence(3),clause(1))),literal(2)),affects(rule_id(sentence(3),clause(1))),predicate_key(smoker,arity(1))).
rule(rule_id(sentence(2),clause(1)),pred(smoker,[var(1)]),body([pred(patient,[var(1)]),pred(cough,[var(1)])]),source(sentence(2),tokens([2,4,5,7]))).
rule(rule_id(sentence(3),clause(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),naf(pred(smoker,[var(1)]))]),source(sentence(3),tokens([2,4,8,9]))).
query(query_id(sentence(4),clause(1)),pred(recover,[named('John')]),source(sentence(4),tokens([3]))).
