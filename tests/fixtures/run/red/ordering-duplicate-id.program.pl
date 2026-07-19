cnl_program_record(1).
document(docid('duplicate-id'),source_sha256('0000000000000000000000000000000000000000000000000000000000000000'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred(p,[named(a)]),body([])).
clause(fact_id(sentence(1),clause(1)),pred(q,[named(a)]),body([])).
goal(query_id(sentence(2),clause(1)),pred(p,[named(a)])).
