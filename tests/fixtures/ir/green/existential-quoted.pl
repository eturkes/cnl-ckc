cnl_ir_record(1).
document(docid('quoted-case'),source_sha256('23456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef01'),ulex(none)).
fact(fact_id(sentence(1),clause(1)),pred(clinician,[named('Dr O\'Neil')]),source(sentence(1),tokens([1,2]))).
fact(fact_id(sentence(1),clause(2)),pred(treats,[named('Dr O\'Neil'),named('Case-7')]),source(sentence(1),tokens([3,4]))).
fact(fact_id(sentence(1),clause(3)),pred(case_marker,[named('Case-7')]),source(sentence(1),tokens([4]))).
rule(rule_id(sentence(2),clause(1)),pred(connected,[var(1)]),body([pred(treats,[var(1),var(2)]),pred(case_marker,[var(2)])]),source(sentence(2),tokens([1,3,5]))).
query(query_id(sentence(3),clause(1)),pred(connected,[named('Dr O\'Neil')]),source(sentence(3),tokens([2]))).
