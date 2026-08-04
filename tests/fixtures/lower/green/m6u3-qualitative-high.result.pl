cnl_answer_record(3).
document(docid('qualitative-dose-high'),source_sha256('0000000000000000000000000000000000000000000000000000000000000000'),ulex(none)).
program(sha256('fa20a9bde5552df6c25723de2aacefaf38baffb5ab3a8ad39eb60f172a8b0138')).
answer(query_id(sentence(3),clause(1)),pred(preferred,[named('Dose')]),proved).
proof(pred(preferred,[named('Dose')]),rule_id(sentence(2),clause(1)),[proof(pred(dosage,[named('Dose')]),fact_id(sentence(1),clause(1)),[]),proof(pred('high comp',[named('Dose')]),fact_id(sentence(1),clause(2)),[])]).
