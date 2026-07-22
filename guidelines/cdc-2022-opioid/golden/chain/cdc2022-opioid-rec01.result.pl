cnl_answer_record(2).
document(docid('cdc2022-opioid-rec01'),source_sha256('25f4bc0b5109717d108c138377be615898d95b4306ea1a0fc582dda009b47b35'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
program(sha256('b5e89fd79ce0d736cd207ffdcd2213be49374c3fd57f54d7ea714bfd6590f334')).
answer(query_id(sentence(3),clause(1)),pred('maximize-nonopioid-therapy',[named('Rec01-clinician')]),proved).
proof(pred('maximize-nonopioid-therapy',[named('Rec01-clinician')]),rule_id(sentence(2),clause(1)),[proof(pred('acute-pain-clinician',[named('Rec01-clinician')]),fact_id(sentence(1),clause(1)),[])]).
