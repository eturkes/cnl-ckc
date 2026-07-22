cnl_program_record(2).
document(docid('cdc2022-opioid-rec08'),source_sha256('03f8f2ea32e635777708e8e0ada2c8726f8b01a134a7a5c55a1e6bfcf76025fd'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
clause(fact_id(sentence(1),clause(1)),pred('risk-mitigation-clinician',[named('Rec08-clinician')]),body([])).
clause(fact_id(sentence(2),clause(1)),pred('risk-mitigation-clinician',[named('Rec08-peer')]),body([])).
clause(rule_id(sentence(3),clause(1)),pred('offer-naloxone',[var(1)]),body([pred('risk-mitigation-clinician',[var(1)])])).
goal(query_id(sentence(4),clause(1)),wh(who),pred('offer-naloxone',[var(1)])).
