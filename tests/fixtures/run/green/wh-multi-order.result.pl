cnl_answer_record(2).
document(docid('wh-multi-order'),source_sha256('4444444444444444444444444444444444444444444444444444444444444444'),ulex(none)).
program(sha256('b8aabde529e4a544979f104e7e65df4d19feb899651c32981f3a4d75ae2adadd')).
answer(query_id(sentence(4),clause(1)),wh(who),pred(recover,[var(1)]),answers([pred(recover,[named('z z')]),pred(recover,[named(a)])])).
proof(pred(recover,[named('z z')]),rule_id(sentence(3),clause(1)),[proof(pred(patient,[named('z z')]),fact_id(sentence(2),clause(1)),[])]).
proof(pred(recover,[named(a)]),rule_id(sentence(3),clause(1)),[proof(pred(patient,[named(a)]),fact_id(sentence(1),clause(1)),[])]).
