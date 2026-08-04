cnl_answer_record(3).
document(docid('qualitative-dose-lowest'),source_sha256('1111111111111111111111111111111111111111111111111111111111111111'),ulex(none)).
program(sha256('7e1cea32a8d2bc9c2ad7f4dfdb4b55f0cab329ba3faef547c65f77c5f1e4cb69')).
answer(query_id(sentence(3),clause(1)),pred(preferred,[named('Dose')]),proved).
proof(pred(preferred,[named('Dose')]),rule_id(sentence(2),clause(1)),[proof(pred(dosage,[named('Dose')]),fact_id(sentence(1),clause(1)),[]),proof(pred('low sup',[named('Dose')]),fact_id(sentence(1),clause(2)),[])]).
