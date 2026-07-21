cnl_program_record(2).
document(docid('body-only'),source_sha256('1111111111111111111111111111111111111111111111111111111111111111'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred(likes,[named(a),named(b)]),body([])).
clause(rule_id(sentence(2),clause(1)),pred(happy,[var(1)]),body([pred(likes,[var(1),var(2)])])).
goal(query_id(sentence(3),clause(1)),pred(happy,[named(a)])).
