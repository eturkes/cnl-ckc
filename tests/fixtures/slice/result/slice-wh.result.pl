cnl_answer_record(2).
document(docid('slice-wh'),source_sha256('80cf551d677bcd4ccf6b94b7299a147a92b827b84bd624312780c2f0f2c91775'),ulex(sha256('6015f9a18e4d4957b30e04342d2ff2700bf0e18b13bf3b95452a2d5563c5b614'))).
program(sha256('2485691fef94ff20e510592e54a6e5a909714ebec9bf7345808e6db032b677c0')).
answer(query_id(sentence(4),clause(1)),wh(who),pred(recover,[var(1)]),answers([pred(recover,[named('John')])])).
proof(pred(recover,[named('John')]),rule_id(sentence(3),clause(1)),[proof(pred(patient,[named('John')]),fact_id(sentence(1),clause(1)),[]),proof(pred(wait,[named('John')]),fact_id(sentence(2),clause(1)),[])]).
