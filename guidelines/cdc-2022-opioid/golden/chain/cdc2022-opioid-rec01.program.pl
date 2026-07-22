cnl_program_record(2).
document(docid('cdc2022-opioid-rec01'),source_sha256('25f4bc0b5109717d108c138377be615898d95b4306ea1a0fc582dda009b47b35'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
clause(fact_id(sentence(1),clause(1)),pred('acute-pain-clinician',[named('Rec01-clinician')]),body([])).
clause(rule_id(sentence(2),clause(1)),pred('maximize-nonopioid-therapy',[var(1)]),body([pred('acute-pain-clinician',[var(1)])])).
goal(query_id(sentence(3),clause(1)),pred('maximize-nonopioid-therapy',[named('Rec01-clinician')])).
