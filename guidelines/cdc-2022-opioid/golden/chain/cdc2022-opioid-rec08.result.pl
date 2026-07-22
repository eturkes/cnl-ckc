cnl_answer_record(2).
document(docid('cdc2022-opioid-rec08'),source_sha256('03f8f2ea32e635777708e8e0ada2c8726f8b01a134a7a5c55a1e6bfcf76025fd'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
program(sha256('274e6d6255984078019410ffd966a8bd800f6bf1540b34045ecfca639e8e898b')).
answer(query_id(sentence(4),clause(1)),wh(who),pred('offer-naloxone',[var(1)]),answers([pred('offer-naloxone',[named('Rec08-clinician')]),pred('offer-naloxone',[named('Rec08-peer')])])).
proof(pred('offer-naloxone',[named('Rec08-clinician')]),rule_id(sentence(3),clause(1)),[proof(pred('risk-mitigation-clinician',[named('Rec08-clinician')]),fact_id(sentence(1),clause(1)),[])]).
proof(pred('offer-naloxone',[named('Rec08-peer')]),rule_id(sentence(3),clause(1)),[proof(pred('risk-mitigation-clinician',[named('Rec08-peer')]),fact_id(sentence(2),clause(1)),[])]).
