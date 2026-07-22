cnl_answer_record(2).
document(docid('cdc2022-opioid-rec08'),source_sha256('03f8f2ea32e635777708e8e0ada2c8726f8b01a134a7a5c55a1e6bfcf76025fd'),ulex(sha256('36b580ac58e5ca0eb85e47ffe01815a27b3a38cd87ee4a8d63d151a4a5dd63a9'))).
program(sha256('f464a14393a3f6d6ff496f6f4d28a50a1d82834511627e21c11343e1ea942c4c')).
answer(query_id(sentence(4),clause(1)),wh(who),pred('offer-naloxone',[var(1)]),answers([pred('offer-naloxone',[named('Rec08-clinician')]),pred('offer-naloxone',[named('Rec08-peer')])])).
proof(pred('offer-naloxone',[named('Rec08-clinician')]),rule_id(sentence(3),clause(1)),[proof(pred('risk-mitigation-clinician',[named('Rec08-clinician')]),fact_id(sentence(1),clause(1)),[])]).
proof(pred('offer-naloxone',[named('Rec08-peer')]),rule_id(sentence(3),clause(1)),[proof(pred('risk-mitigation-clinician',[named('Rec08-peer')]),fact_id(sentence(2),clause(1)),[])]).
