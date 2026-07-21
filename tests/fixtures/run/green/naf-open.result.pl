cnl_answer_record(2).
document(docid('naf-open'),source_sha256('3333333333333333333333333333333333333333333333333333333333333333'),ulex(none)).
program(sha256('a4e4c11f215a57cb3154537cda48a6e04c9ce3ffa0a0c6dcfdde3646f690f467')).
answer(query_id(sentence(3),clause(1)),pred(recover,[named('café patient')]),proved).
proof(pred(recover,[named('café patient')]),rule_id(sentence(2),clause(1)),[proof(pred(patient,[named('café patient')]),fact_id(sentence(1),clause(1)),[]),naf(pred(smoke,[named('café patient')]))]).
