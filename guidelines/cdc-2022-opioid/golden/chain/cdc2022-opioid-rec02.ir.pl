cnl_ir_record(2).
document(docid('cdc2022-opioid-rec02'),source_sha256('06212d79c2a865211f33f170fdbe7d5f5554701c83a116da02fb07531b0aa790'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
fact(fact_id(sentence(1),clause(1)),pred('subacute-chronic-pain-clinician',[named('Rec02-clinician')]),source(sentence(1),tokens([2,4]))).
rule(rule_id(sentence(2),clause(1)),pred('maximize-nonopioid-therapy',[var(1)]),body([pred('subacute-chronic-pain-clinician',[var(1)])]),source(sentence(2),tokens([2,3]))).
query(query_id(sentence(3),clause(1)),pred('maximize-nonopioid-therapy',[named('Rec02-clinician')]),source(sentence(3),tokens([3]))).
