cnl_answer_record(3).
document(docid('qualitative-dose-highest'),source_sha256('2222222222222222222222222222222222222222222222222222222222222222'),ulex(none)).
program(sha256('51bbca005d2c2dd46392465a88732fb8eb616f294db6a7c94239f135893bb807')).
answer(query_id(sentence(3),clause(1)),pred(preferred,[named('Dose')]),proved).
proof(pred(preferred,[named('Dose')]),rule_id(sentence(2),clause(1)),[proof(pred(dosage,[named('Dose')]),fact_id(sentence(1),clause(1)),[]),proof(pred('high sup',[named('Dose')]),fact_id(sentence(1),clause(2)),[])]).
