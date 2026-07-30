cnl_program_record(3).
document(docid('m6-prop-comp-than'),source_sha256('831e6b3421728f133ea6f7c74c3b4038b58440403fcc6ab0db9274712d8fe48b'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred('helpful comp_than',[named('Mary'),named('John')]),body([])).
clause(fact_id(sentence(2),clause(1)),pred(patient,[named('John')]),body([])).
clause(rule_id(sentence(3),clause(1)),pred('helpful comp_than',[var(1),named('Mary')]),body([pred(patient,[var(1)])])).
goal(query_id(sentence(4),clause(1)),pred('helpful comp_than',[named('John'),named('Mary')])).
