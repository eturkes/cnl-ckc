cnl_program_record(3).
document(docid('m6u3-temporal-cross-unit'),source_sha256('3333333333333333333333333333333333333333333333333333333333333333'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),temporal_window(after,pred(wait,[named(a)]),anchor(named(start)),interval(quantity_bound(geq,closed,quantity(integer(1),unit(week))),quantity_bound(leq,closed,quantity(integer(4),unit(day))))),body([])).
goal(query_id(sentence(2),clause(1)),pred(done,[named(a)])).
