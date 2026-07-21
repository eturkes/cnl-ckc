cnl_ir_record(2).
document(docid('naf-rule'),source_sha256('2222222222222222222222222222222222222222222222222222222222222222'),ulex(none)).
fact(fact_id(sentence(1),clause(1)),pred(patient,[named(a)]),source(sentence(1),tokens([1]))).
rule(rule_id(sentence(2),clause(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),naf(pred(smoke,[var(1)]))]),source(sentence(2),tokens([1,2,3]))).
query(query_id(sentence(3),clause(1)),pred(recover,[named(a)]),source(sentence(3),tokens([1]))).
