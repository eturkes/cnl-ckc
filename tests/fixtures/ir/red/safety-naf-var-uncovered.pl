cnl_ir_record(2).
document(docid('safety-naf-var-uncovered'),source_sha256('0000000000000000000000000000000000000000000000000000000000000000'),ulex(none)).
rule(rule_id(sentence(1),clause(1)),pred(p,[var(1)]),body([pred(q,[var(1)]),naf(pred(r,[var(2)]))]),source(sentence(1),tokens([1,2]))).
query(query_id(sentence(2),clause(1)),pred(p,[named(a)]),source(sentence(2),tokens([1]))).
