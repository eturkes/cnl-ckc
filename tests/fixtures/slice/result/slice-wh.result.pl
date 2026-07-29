cnl_answer_record(3).
document(docid('slice-wh'),source_sha256('80cf551d677bcd4ccf6b94b7299a147a92b827b84bd624312780c2f0f2c91775'),ulex(sha256('6015f9a18e4d4957b30e04342d2ff2700bf0e18b13bf3b95452a2d5563c5b614'))).
program(sha256('56cd4ae3db93210297e908d683f788d4fa02327918d54156fe1f89c9916eb723')).
answer(query_id(sentence(4),clause(1)),wh(who),pred(recover,[var(1)]),answers([pred(recover,[named('John')])])).
proof(pred(recover,[named('John')]),rule_id(sentence(3),clause(1)),[proof(pred(patient,[named('John')]),fact_id(sentence(1),clause(1)),[]),proof(pred(wait,[named('John')]),fact_id(sentence(2),clause(1)),[])]).
