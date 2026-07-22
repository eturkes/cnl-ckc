cnl_answer_record(2).
document(docid('cdc2022-opioid-rec01'),source_sha256('25f4bc0b5109717d108c138377be615898d95b4306ea1a0fc582dda009b47b35'),ulex(sha256('36b580ac58e5ca0eb85e47ffe01815a27b3a38cd87ee4a8d63d151a4a5dd63a9'))).
program(sha256('456c3141c96dd30ab5d1548a273909e9f4a441f61dc8e2fb407ebfd480f4f601')).
answer(query_id(sentence(3),clause(1)),pred('maximize-nonopioid-therapy',[named('Rec01-clinician')]),proved).
proof(pred('maximize-nonopioid-therapy',[named('Rec01-clinician')]),rule_id(sentence(2),clause(1)),[proof(pred('acute-pain-clinician',[named('Rec01-clinician')]),fact_id(sentence(1),clause(1)),[])]).
