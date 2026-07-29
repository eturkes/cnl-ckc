cnl_ir_record(3).
document(docid('nary-same-name-cross-arity'),source_sha256('8888888888888888888888888888888888888888888888888888888888888888'),ulex(none)).
fact(fact_id(sentence(1),clause(1)),pred(p,[named(a)]),source(sentence(1),tokens([1]))).
rule(rule_id(sentence(2),clause(1)),pred(p,[var(1),named(b)]),body([pred(p,[var(1)])]),source(sentence(2),tokens([1]))).
query(query_id(sentence(3),clause(1)),pred(p,[named(a),named(b)]),source(sentence(3),tokens([1]))).
