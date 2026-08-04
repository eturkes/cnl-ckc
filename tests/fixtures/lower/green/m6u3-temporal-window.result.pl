cnl_answer_record(3).
document(docid('temporal-window'),source_sha256('0000000000000000000000000000000000000000000000000000000000000000'),ulex(none)).
program(sha256('2703253d117c1af099a5f64c7e1dc3941bba133820949cc611fd7204178dcc05')).
answer(query_id(sentence(4),clause(1)),pred(recover,[named('John')]),proved).
proof(pred(recover,[named('John')]),rule_id(sentence(3),clause(1)),[proof(pred(patient,[named('John')]),fact_id(sentence(1),clause(1)),[]),proof(temporal_window(after,pred(wait,[named('John')]),anchor(named('Therapy-start')),interval(quantity_bound(geq,closed,quantity(integer(1),unit(week))),quantity_bound(leq,closed,quantity(integer(4),unit(week))))),fact_id(sentence(2),clause(1)),[])]).
