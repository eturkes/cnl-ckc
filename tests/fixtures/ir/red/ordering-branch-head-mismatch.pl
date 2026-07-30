cnl_ir_record(3).
document(docid('ordering-branch-head-mismatch'),source_sha256('9999999999999999999999999999999999999999999999999999999999999999'),ulex(none)).
fact(fact_id(sentence(1),clause(1)),pred(b,[named('John')]),source(sentence(1),tokens([2]))).
rule(rule_id(sentence(3),clause(1),branch(1)),pred(recover,[var(1)]),body([pred(a,[var(1)])]),source(sentence(3),tokens([2,4]))).
rule(rule_id(sentence(3),clause(1),branch(2)),pred(harm,[var(1)]),body([pred(b,[var(1)])]),source(sentence(3),tokens([2,7]))).
query(query_id(sentence(4),clause(1)),pred(harm,[named('John')]),source(sentence(4),tokens([3]))).
