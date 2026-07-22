cnl_ir_record(2).
document(docid('cdc2022-opioid-rec11'),source_sha256('82f481566c93cebc9a5612e09a417531cd0283da485fcfcd82043a9573ddb32c'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
fact(fact_id(sentence(1),clause(1)),pred('concurrent-medication-clinician',[named('Rec11-clinician')]),source(sentence(1),tokens([2,4]))).
rule(rule_id(sentence(2),clause(1)),pred('use-particular-caution',[var(1)]),body([pred('concurrent-medication-clinician',[var(1)])]),source(sentence(2),tokens([2,3]))).
query(query_id(sentence(3),clause(1)),pred('use-particular-caution',[named('Rec11-clinician')]),source(sentence(3),tokens([3]))).
