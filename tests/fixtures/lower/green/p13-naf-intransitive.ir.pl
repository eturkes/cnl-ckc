cnl_ir_record(2).
document(docid('p13-naf-intransitive'),source_sha256('1313131313131313131313131313131313131313131313131313131313131313'),ulex(none)).
fact(fact_id(sentence(1),clause(1)),pred(patient,[named('John')]),source(sentence(1),tokens([2,4]))).
rule(rule_id(sentence(2),clause(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),naf(pred(smoke,[var(1)]))]),source(sentence(2),tokens([2,7,8]))).
query(query_id(sentence(3),clause(1)),pred(recover,[named('John')]),source(sentence(3),tokens([3]))).
