cnl_program_record(2).
document(docid('cdc2022-opioid-rec10'),source_sha256('d28d0c2f2a893dc418d646e852d8dd4a0ebbeadcc8577ea970153af95ecf0c7a'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
clause(fact_id(sentence(1),clause(1)),pred('subacute-chronic-pain-clinician',[named('Rec10-clinician')]),body([])).
clause(fact_id(sentence(2),clause(1)),pred('acute-pain-clinician',[named('Rec10-peer')]),body([])).
clause(rule_id(sentence(3),clause(1)),pred('consider-toxicology-testing',[var(1)]),body([pred('subacute-chronic-pain-clinician',[var(1)])])).
goal(query_id(sentence(4),clause(1)),pred('consider-toxicology-testing',[named('Rec10-peer')])).
