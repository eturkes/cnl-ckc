cnl_program_record(3).
document(docid('m6-prop-pos-as'),source_sha256('861b87589655a4011cd541870168af29f92b164c98b56ffa9fba42e88d99cd78'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred('helpful pos_as',[named('John'),named('Mary')]),body([])).
clause(fact_id(sentence(2),clause(1)),pred(patient,[named('John')]),body([])).
clause(rule_id(sentence(3),clause(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),pred('helpful pos_as',[var(1),named('Mary')])])).
goal(query_id(sentence(4),clause(1)),pred('helpful pos_as',[named('John'),named('Mary')])).
