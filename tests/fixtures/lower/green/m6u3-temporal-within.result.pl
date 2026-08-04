cnl_answer_record(3).
document(docid('temporal-within'),source_sha256('0000000000000000000000000000000000000000000000000000000000000000'),ulex(none)).
program(sha256('2940fdca1cd2704d811c554870ac246dbdf20a7ff0cb8486a09978e68064be19')).
answer(query_id(sentence(3),clause(1)),pred(recover,[named('John')]),proved).
proof(pred(recover,[named('John')]),rule_id(sentence(2),clause(1)),[proof(temporal(within,pred(wait,[named('John')]),anchor(named('Therapy-start'))),fact_id(sentence(1),clause(1)),[])]).
