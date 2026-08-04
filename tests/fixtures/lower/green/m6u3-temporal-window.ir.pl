cnl_ir_record(3).
document(docid('temporal-window'),source_sha256('0000000000000000000000000000000000000000000000000000000000000000'),ulex(none)).
fact(fact_id(sentence(1),clause(1)),pred(patient,[named('John')]),source(sentence(1),tokens([2,4]))).
fact(fact_id(sentence(2),clause(1)),temporal_window(after,pred(wait,[named('John')]),anchor(named('Therapy-start')),interval(quantity_bound(geq,closed,quantity(integer(1),unit(week))),quantity_bound(leq,closed,quantity(integer(4),unit(week))))),source(sentence(2),tokens([1,2,3,5,6,7,8]))).
rule(rule_id(sentence(3),clause(1)),pred(recover,[var(1)]),body([pred(patient,[var(1)]),temporal_window(after,pred(wait,[var(1)]),anchor(named('Therapy-start')),interval(quantity_bound(geq,closed,quantity(integer(1),unit(week))),quantity_bound(leq,closed,quantity(integer(4),unit(week)))))]),source(sentence(3),tokens([2,4,5,6,7,8,9,10,12]))).
query(query_id(sentence(4),clause(1)),pred(recover,[named('John')]),source(sentence(4),tokens([3]))).
