cnl_ir_record(2).
document(docid('cdc2022-opioid-rec09'),source_sha256('5d617e7fa8ce7cff0081b4814fc261212810447908e77ebb6089c8e7a37c07a0'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
fact(fact_id(sentence(1),clause(1)),pred('pdmp-review-clinician',[named('Rec09-clinician')]),source(sentence(1),tokens([2,4]))).
rule(rule_id(sentence(2),clause(1)),pred('review-pdmp-data',[var(1)]),body([pred('pdmp-review-clinician',[var(1)])]),source(sentence(2),tokens([2,3]))).
query(query_id(sentence(3),clause(1)),pred('review-pdmp-data',[named('Rec09-clinician')]),source(sentence(3),tokens([3]))).
