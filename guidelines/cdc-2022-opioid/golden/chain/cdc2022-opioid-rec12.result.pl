cnl_answer_record(2).
document(docid('cdc2022-opioid-rec12'),source_sha256('0e8bc74eac108b480eddf0a7510697523afee483f24ca8921ebdfafd9e03a6f5'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
program(sha256('e35afa27a5acc8b337cabb099e3a440f9f20dea8b032abaf4921ec4612ecdaa5')).
answer(query_id(sentence(3),clause(1)),pred('offer-medication-treatment',[named('Rec12-clinician')]),proved).
proof(pred('offer-medication-treatment',[named('Rec12-clinician')]),rule_id(sentence(2),clause(1)),[proof(pred('opioid-use-disorder-clinician',[named('Rec12-clinician')]),fact_id(sentence(1),clause(1)),[])]).
