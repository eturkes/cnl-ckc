cnl_answer_record(3).
document(docid('cdc2022-opioid-rec02'),source_sha256('06212d79c2a865211f33f170fdbe7d5f5554701c83a116da02fb07531b0aa790'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
program(sha256('10d14730076b81fbd0a3a5a037ca4194fb4c01483dd4a2224a50776bf93c7a0d')).
answer(query_id(sentence(3),clause(1)),pred('maximize-nonopioid-therapy',[named('Rec02-clinician')]),proved).
proof(pred('maximize-nonopioid-therapy',[named('Rec02-clinician')]),rule_id(sentence(2),clause(1)),[proof(pred('subacute-chronic-pain-clinician',[named('Rec02-clinician')]),fact_id(sentence(1),clause(1)),[])]).
