cnl_ir_record(2).
document(docid('cdc2022-opioid-rec08'),source_sha256('03f8f2ea32e635777708e8e0ada2c8726f8b01a134a7a5c55a1e6bfcf76025fd'),ulex(sha256('36b580ac58e5ca0eb85e47ffe01815a27b3a38cd87ee4a8d63d151a4a5dd63a9'))).
fact(fact_id(sentence(1),clause(1)),pred('risk-mitigation-clinician',[named('Rec08-clinician')]),source(sentence(1),tokens([2,4]))).
fact(fact_id(sentence(2),clause(1)),pred('risk-mitigation-clinician',[named('Rec08-peer')]),source(sentence(2),tokens([2,4]))).
rule(rule_id(sentence(3),clause(1)),pred('offer-naloxone',[var(1)]),body([pred('risk-mitigation-clinician',[var(1)])]),source(sentence(3),tokens([2,3]))).
query(query_id(sentence(4),clause(1)),wh(who),pred('offer-naloxone',[var(1)]),source(sentence(4),tokens([1,2]))).
