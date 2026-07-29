cnl_answer_record(3).
document(docid('cdc2022-opioid-rec05'),source_sha256('65b66ef8b6cf5af09e9c0054f325c7cddda88ffaf563b2c5e6d1e1e659ec72c8'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
program(sha256('2f1bd7f862b415b4865c8a0e6afb9ba9b988b3722ca4b4b8c6dd7e764c26b41d')).
answer(query_id(sentence(3),clause(1)),pred('avoid-abrupt-discontinuation',[named('Rec05-clinician')]),proved).
proof(pred('avoid-abrupt-discontinuation',[named('Rec05-clinician')]),rule_id(sentence(2),clause(1)),[proof(pred('dosage-change-clinician',[named('Rec05-clinician')]),fact_id(sentence(1),clause(1)),[]),naf(pred('detect-life-threatening-warning',[named('Rec05-clinician')]))]).
