cnl_answer_record(2).
document(docid('cdc2022-opioid-rec05'),source_sha256('65b66ef8b6cf5af09e9c0054f325c7cddda88ffaf563b2c5e6d1e1e659ec72c8'),ulex(sha256('36b580ac58e5ca0eb85e47ffe01815a27b3a38cd87ee4a8d63d151a4a5dd63a9'))).
program(sha256('3cd08512a1ae390895182276c7809cf79e2443a0df743cc39887784fe9ee2a3a')).
answer(query_id(sentence(3),clause(1)),pred('avoid-abrupt-discontinuation',[named('Rec05-clinician')]),proved).
proof(pred('avoid-abrupt-discontinuation',[named('Rec05-clinician')]),rule_id(sentence(2),clause(1)),[proof(pred('dosage-change-clinician',[named('Rec05-clinician')]),fact_id(sentence(1),clause(1)),[]),naf(pred('detect-life-threatening-warning',[named('Rec05-clinician')]))]).
