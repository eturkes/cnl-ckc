cnl_answer_record(3).
document(docid('qualitative-dose-low'),source_sha256('0000000000000000000000000000000000000000000000000000000000000000'),ulex(none)).
program(sha256('b5d2d46cecaf1c30c38386f31728149f9b9b10101e4411a5ad095efe606cf2ea')).
answer(query_id(sentence(3),clause(1)),pred(preferred,[named('Dose')]),proved).
proof(pred(preferred,[named('Dose')]),rule_id(sentence(2),clause(1)),[proof(pred(dosage,[named('Dose')]),fact_id(sentence(1),clause(1)),[]),proof(pred('low comp',[named('Dose')]),fact_id(sentence(1),clause(2)),[])]).
