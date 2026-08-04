cnl_answer_record(3).
document(docid('m6u3-temporal-matrix'),source_sha256('1717171717171717171717171717171717171717171717171717171717171717'),ulex(none)).
program(sha256('458170c595af3856bbd1d840486a796265ea05331513a4fbfe5da18d3898ba56')).
answer(query_id(sentence(4),clause(1)),pred(ready,[named(a)]),proved).
proof(pred(ready,[named(a)]),rule_id(sentence(3),clause(1)),[proof(temporal_window(before,pred(wait,[named(a)]),anchor(named(start)),interval(quantity_bound(greater,open,quantity(integer(1),unit(week))),quantity_bound(less,open,quantity(integer(4),unit(week))))),fact_id(sentence(1),clause(1)),[]),proof(temporal_window(after,pred(stay,[named(a)]),anchor(named(start)),interval(quantity_bound(geq,closed,quantity(integer(2),unit(week))),quantity_bound(leq,closed,quantity(integer(2),unit(week))))),fact_id(sentence(2),clause(1)),[])]).
