cnl_program_record(3).
document(docid('m6u3-runtime-not-quantity'),source_sha256('7777777777777777777777777777777777777777777777777777777777777777'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred(have,[named(a),named(old)]),body([])).
clause(rule_id(sentence(2),clause(1)),pred(eligible,[var(1)]),body([pred(have,[var(1),var(2)]),quantity_compare(var(2),quantity_bound(greater,open,quantity(integer(64),unit(year))))])).
goal(query_id(sentence(3),clause(1)),pred(eligible,[named(a)])).
