cnl_program_record(3).
document(docid('nary-binary'),source_sha256('6666666666666666666666666666666666666666666666666666666666666666'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred(edge,[named(a),named(b)]),body([])).
clause(rule_id(sentence(2),clause(1)),pred(connected,[var(1),var(2)]),body([pred(edge,[var(1),var(2)])])).
goal(query_id(sentence(3),clause(1)),pred(connected,[named(a),named(b)])).
