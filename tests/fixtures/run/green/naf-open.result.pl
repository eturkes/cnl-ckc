cnl_answer_record(3).
document(docid('naf-open'),source_sha256('3333333333333333333333333333333333333333333333333333333333333333'),ulex(none)).
program(sha256('d9a9e271f0eb88d6132f4a9ccf0be159c42e49dd8ecc9eb50b010d4d298b46f9')).
answer(query_id(sentence(3),clause(1)),pred(recover,[named('café patient')]),proved).
proof(pred(recover,[named('café patient')]),rule_id(sentence(2),clause(1)),[proof(pred(patient,[named('café patient')]),fact_id(sentence(1),clause(1)),[]),naf(pred(smoke,[named('café patient')]))]).
