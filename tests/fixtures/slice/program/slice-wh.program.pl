cnl_program_record(2).
document(docid('slice-wh'),source_sha256('80cf551d677bcd4ccf6b94b7299a147a92b827b84bd624312780c2f0f2c91775'),ulex(sha256('6015f9a18e4d4957b30e04342d2ff2700bf0e18b13bf3b95452a2d5563c5b614'))).
clause(fact_id(sentence(1),clause(1)),pred(patient,[named('John')]),body([])).
clause(fact_id(sentence(2),clause(1)),pred(wait,[named('John')]),body([])).
clause(rule_id(sentence(3),clause(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),pred(wait,[var(1)])])).
goal(query_id(sentence(4),clause(1)),wh(who),pred(recover,[var(1)])).
