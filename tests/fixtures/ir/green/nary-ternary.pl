cnl_ir_record(3).
document(docid('nary-ternary'),source_sha256('7777777777777777777777777777777777777777777777777777777777777777'),ulex(none)).
fact(fact_id(sentence(1),clause(1)),pred(triple,[named(a),named(b),named(c)]),source(sentence(1),tokens([1]))).
rule(rule_id(sentence(2),clause(1)),pred(copy,[var(1),var(2),var(3)]),body([pred(triple,[var(1),var(2),var(3)])]),source(sentence(2),tokens([1]))).
query(query_id(sentence(3),clause(1)),pred(copy,[named(a),named(b),named(c)]),source(sentence(3),tokens([1]))).
