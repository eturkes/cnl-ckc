cnl_ir_record(3).
document(docid('m6-prop-pos-as'),source_sha256('861b87589655a4011cd541870168af29f92b164c98b56ffa9fba42e88d99cd78'),ulex(none)).
fact(fact_id(sentence(1),clause(1)),pred('helpful pos_as',[named('John'),named('Mary')]),source(sentence(1),tokens([2,4]))).
fact(fact_id(sentence(2),clause(1)),pred(patient,[named('John')]),source(sentence(2),tokens([2,4]))).
rule(rule_id(sentence(3),clause(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),pred('helpful pos_as',[var(1),named('Mary')])]),source(sentence(3),tokens([2,4,6,9]))).
query(query_id(sentence(4),clause(1)),pred('helpful pos_as',[named('John'),named('Mary')]),source(sentence(4),tokens([1,4]))).
