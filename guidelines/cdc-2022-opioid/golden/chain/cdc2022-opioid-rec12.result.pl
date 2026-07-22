cnl_answer_record(2).
document(docid('cdc2022-opioid-rec12'),source_sha256('0e8bc74eac108b480eddf0a7510697523afee483f24ca8921ebdfafd9e03a6f5'),ulex(sha256('36b580ac58e5ca0eb85e47ffe01815a27b3a38cd87ee4a8d63d151a4a5dd63a9'))).
program(sha256('98a368e0e06865b220c40cfc7f33213b94d0944c9f88beb13cf45a01b8184648')).
answer(query_id(sentence(3),clause(1)),pred('offer-medication-treatment',[named('Rec12-clinician')]),proved).
proof(pred('offer-medication-treatment',[named('Rec12-clinician')]),rule_id(sentence(2),clause(1)),[proof(pred('opioid-use-disorder-clinician',[named('Rec12-clinician')]),fact_id(sentence(1),clause(1)),[])]).
