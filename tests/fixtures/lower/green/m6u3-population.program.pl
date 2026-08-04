cnl_program_record(3).
document(docid('population-numeric'),source_sha256('0000000000000000000000000000000000000000000000000000000000000000'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred(adult,[named('John')]),body([])).
clause(fact_id(sentence(2),clause(1)),pred(have,[named('John'),quantity(integer(70),unit(year))]),body([])).
clause(rule_id(sentence(3),clause(1)),pred(wait,[var(1)]),body([pred(adult,[var(1)]),pred(have,[var(1),var(2)]),quantity_compare(var(2),quantity_bound(greater,open,quantity(integer(64),unit(year))))])).
goal(query_id(sentence(4),clause(1)),pred(wait,[named('John')])).
