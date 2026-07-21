cnl_ir_record(2).
document(docid('cycle-signed-transitive'),source_sha256('0000000000000000000000000000000000000000000000000000000000000000'),ulex(none)).
rule(rule_id(sentence(1),clause(1)),pred(p,[var(1)]),body([pred(q,[var(1)])]),source(sentence(1),tokens([1]))).
rule(rule_id(sentence(2),clause(1)),pred(q,[var(1)]),body([pred(r,[var(1)])]),source(sentence(2),tokens([1]))).
rule(rule_id(sentence(3),clause(1)),pred(r,[var(1)]),body([pred(base,[var(1)]),naf(pred(p,[var(1)]))]),source(sentence(3),tokens([1,2]))).
query(query_id(sentence(4),clause(1)),pred(p,[named(a)]),source(sentence(4),tokens([1]))).
