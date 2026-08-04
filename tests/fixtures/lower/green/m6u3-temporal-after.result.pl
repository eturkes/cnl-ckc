cnl_answer_record(3).
document(docid('temporal-after'),source_sha256('0000000000000000000000000000000000000000000000000000000000000000'),ulex(none)).
program(sha256('bd3759e5f2eed4cb73f3661004c77b4cc5909fddbe4e45b6c49c44f6f6b88b40')).
answer(query_id(sentence(3),clause(1)),pred(recover,[named('John')]),proved).
proof(pred(recover,[named('John')]),rule_id(sentence(2),clause(1)),[proof(temporal(after,pred(wait,[named('John')]),anchor(named('Therapy-start'))),fact_id(sentence(1),clause(1)),[])]).
