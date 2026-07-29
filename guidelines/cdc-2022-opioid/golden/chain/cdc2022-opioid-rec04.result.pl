cnl_answer_record(3).
document(docid('cdc2022-opioid-rec04'),source_sha256('1a151ce7d02368aab8e51c8744194f5567199863872d64469891783ae242ead4'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
program(sha256('d721cc80f2a612bdc11eda071a745a5c4b25310325275a13b593c0333fa3b8f8')).
answer(query_id(sentence(3),clause(1)),pred('prescribe-lowest-effective-dosage',[named('Rec04-clinician')]),proved).
proof(pred('prescribe-lowest-effective-dosage',[named('Rec04-clinician')]),rule_id(sentence(2),clause(1)),[proof(pred('opioid-dosage-clinician',[named('Rec04-clinician')]),fact_id(sentence(1),clause(1)),[])]).
