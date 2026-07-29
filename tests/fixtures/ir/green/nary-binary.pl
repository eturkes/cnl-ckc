cnl_ir_record(3).
document(docid('nary-binary'),source_sha256('6666666666666666666666666666666666666666666666666666666666666666'),ulex(none)).
fact(fact_id(sentence(1),clause(1)),pred(edge,[named(a),named(b)]),source(sentence(1),tokens([1]))).
rule(rule_id(sentence(2),clause(1)),pred(connected,[var(1),var(2)]),body([pred(edge,[var(1),var(2)])]),source(sentence(2),tokens([1]))).
query(query_id(sentence(3),clause(1)),pred(connected,[named(a),named(b)]),source(sentence(3),tokens([1]))).
