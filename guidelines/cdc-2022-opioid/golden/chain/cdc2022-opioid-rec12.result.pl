cnl_answer_record(3).
document(docid('cdc2022-opioid-rec12'),source_sha256('0e8bc74eac108b480eddf0a7510697523afee483f24ca8921ebdfafd9e03a6f5'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
program(sha256('17d2e1c2f3520f95a22b02fc66b9b497eec01141d2d373115ac985978b3e8579')).
answer(query_id(sentence(3),clause(1)),pred('offer-medication-treatment',[named('Rec12-clinician')]),proved).
proof(pred('offer-medication-treatment',[named('Rec12-clinician')]),rule_id(sentence(2),clause(1)),[proof(pred('opioid-use-disorder-clinician',[named('Rec12-clinician')]),fact_id(sentence(1),clause(1)),[])]).
