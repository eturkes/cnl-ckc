cnl_ir_record(3).
document(docid('qualitative-dose-lowest'),source_sha256('1111111111111111111111111111111111111111111111111111111111111111'),ulex(none)).
fact(fact_id(sentence(1),clause(1)),pred(dosage,[named('Dose')]),source(sentence(1),tokens([2,5]))).
fact(fact_id(sentence(1),clause(2)),pred('low sup',[named('Dose')]),source(sentence(1),tokens([2,4]))).
rule(rule_id(sentence(2),clause(1)),pred(preferred,[var(1)]),body([pred(dosage,[var(1)]),pred('low sup',[var(1)])]),source(sentence(2),tokens([2,3,4,5]))).
query(query_id(sentence(3),clause(1)),pred(preferred,[named('Dose')]),source(sentence(3),tokens([1,3]))).
