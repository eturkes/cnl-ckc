cnl_program_record(3).
document(docid('m6-prop-attr'),source_sha256('e07ccd306fc8494c02bed3cf26e7af5f2288e4765b54d52ab7451c4aa8297176'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred(patient,[named('John')]),body([])).
clause(fact_id(sentence(1),clause(2)),pred(helpful,[named('John')]),body([])).
clause(fact_id(sentence(1),clause(3)),pred(careful,[named('John')]),body([])).
clause(rule_id(sentence(2),clause(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),pred(helpful,[var(1)]),pred(careful,[var(1)])])).
goal(query_id(sentence(3),clause(1)),pred(recover,[named('John')])).
