cnl_program_record(2).
document(docid('cdc2022-opioid-rec05'),source_sha256('65b66ef8b6cf5af09e9c0054f325c7cddda88ffaf563b2c5e6d1e1e659ec72c8'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
clause(fact_id(sentence(1),clause(1)),pred('dosage-change-clinician',[named('Rec05-clinician')]),body([])).
clause(rule_id(sentence(2),clause(1)),pred('avoid-abrupt-discontinuation',[var(1)]),body([pred('dosage-change-clinician',[var(1)]),naf(pred('detect-life-threatening-warning',[var(1)]))])).
goal(query_id(sentence(3),clause(1)),pred('avoid-abrupt-discontinuation',[named('Rec05-clinician')])).
