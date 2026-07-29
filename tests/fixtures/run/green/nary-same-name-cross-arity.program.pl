cnl_program_record(3).
document(docid('nary-same-name-cross-arity'),source_sha256('8888888888888888888888888888888888888888888888888888888888888888'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred(p,[named(a)]),body([])).
clause(rule_id(sentence(2),clause(1)),pred(p,[var(1),named(b)]),body([pred(p,[var(1)])])).
goal(query_id(sentence(3),clause(1)),pred(p,[named(a),named(b)])).
