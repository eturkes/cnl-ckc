cnl_program_record(3).
document(docid('m6u3-temporal-matrix'),source_sha256('1717171717171717171717171717171717171717171717171717171717171717'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),temporal_window(before,pred(wait,[named(a)]),anchor(named(start)),interval(quantity_bound(greater,open,quantity(integer(1),unit(week))),quantity_bound(less,open,quantity(integer(4),unit(week))))),body([])).
clause(fact_id(sentence(2),clause(1)),temporal_window(after,pred(stay,[named(a)]),anchor(named(start)),interval(quantity_bound(geq,closed,quantity(integer(2),unit(week))),quantity_bound(leq,closed,quantity(integer(2),unit(week))))),body([])).
clause(rule_id(sentence(3),clause(1)),pred(ready,[var(1)]),body([temporal_window(before,pred(wait,[var(1)]),anchor(named(start)),interval(quantity_bound(greater,open,quantity(integer(1),unit(week))),quantity_bound(less,open,quantity(integer(4),unit(week))))),temporal_window(after,pred(stay,[var(1)]),anchor(var(2)),interval(quantity_bound(geq,closed,quantity(integer(2),unit(week))),quantity_bound(leq,closed,quantity(integer(2),unit(week)))))])).
goal(query_id(sentence(4),clause(1)),pred(ready,[named(a)])).
