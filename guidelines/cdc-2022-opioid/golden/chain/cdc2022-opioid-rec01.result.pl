cnl_answer_record(3).
document(docid('cdc2022-opioid-rec01'),source_sha256('25f4bc0b5109717d108c138377be615898d95b4306ea1a0fc582dda009b47b35'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
program(sha256('1eb227bac8049459e37c1e56ef0077843c93edcca7edbd47737299191ff0d8c6')).
answer(query_id(sentence(3),clause(1)),pred('maximize-nonopioid-therapy',[named('Rec01-clinician')]),proved).
proof(pred('maximize-nonopioid-therapy',[named('Rec01-clinician')]),rule_id(sentence(2),clause(1)),[proof(pred('acute-pain-clinician',[named('Rec01-clinician')]),fact_id(sentence(1),clause(1)),[])]).
