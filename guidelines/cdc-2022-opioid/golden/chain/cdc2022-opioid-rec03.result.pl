cnl_answer_record(2).
document(docid('cdc2022-opioid-rec03'),source_sha256('65bafa8f9ffab76dd476a936f9b124e6baf210182ed1e23b4314ea3268db2e7f'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
program(sha256('6d25ca602e1d282a427ccd7520b74d608a3f10127df76959b0e1567521053f22')).
answer(query_id(sentence(3),clause(1)),pred('prescribe-immediate-release',[named('Rec03-clinician')]),proved).
proof(pred('prescribe-immediate-release',[named('Rec03-clinician')]),rule_id(sentence(2),clause(1)),[proof(pred('starting-opioid-clinician',[named('Rec03-clinician')]),fact_id(sentence(1),clause(1)),[])]).
