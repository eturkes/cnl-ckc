cnl_ir_record(3).
document(docid('m6-prop-comp-than'),source_sha256('831e6b3421728f133ea6f7c74c3b4038b58440403fcc6ab0db9274712d8fe48b'),ulex(none)).
fact(fact_id(sentence(1),clause(1)),pred('helpful comp_than',[named('Mary'),named('John')]),source(sentence(1),tokens([2,4]))).
fact(fact_id(sentence(2),clause(1)),pred(patient,[named('John')]),source(sentence(2),tokens([2,4]))).
rule(rule_id(sentence(3),clause(1)),pred('helpful comp_than',[var(1),named('Mary')]),body([pred(patient,[var(1)])]),source(sentence(3),tokens([2,3,5]))).
query(query_id(sentence(4),clause(1)),pred('helpful comp_than',[named('John'),named('Mary')]),source(sentence(4),tokens([1,4]))).
