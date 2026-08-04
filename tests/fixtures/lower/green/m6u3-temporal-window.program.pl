cnl_program_record(3).
document(docid('temporal-window'),source_sha256('0000000000000000000000000000000000000000000000000000000000000000'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred(patient,[named('John')]),body([])).
clause(fact_id(sentence(2),clause(1)),temporal_window(after,pred(wait,[named('John')]),anchor(named('Therapy-start')),interval(quantity_bound(geq,closed,quantity(integer(1),unit(week))),quantity_bound(leq,closed,quantity(integer(4),unit(week))))),body([])).
clause(rule_id(sentence(3),clause(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),temporal_window(after,pred(wait,[var(1)]),anchor(named('Therapy-start')),interval(quantity_bound(geq,closed,quantity(integer(1),unit(week))),quantity_bound(leq,closed,quantity(integer(4),unit(week)))))])).
goal(query_id(sentence(4),clause(1)),pred(recover,[named('John')])).
