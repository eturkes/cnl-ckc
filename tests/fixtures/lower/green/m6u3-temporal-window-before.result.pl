cnl_answer_record(3).
document(docid('temporal-window-before'),source_sha256('6666666666666666666666666666666666666666666666666666666666666666'),ulex(none)).
program(sha256('57ae4fe614369079b42d9d1177f6c1974b385799d27518721ec277e8164de72e')).
answer(query_id(sentence(4),clause(1)),pred(recover,[named('John')]),proved).
proof(pred(recover,[named('John')]),rule_id(sentence(3),clause(1)),[proof(pred(patient,[named('John')]),fact_id(sentence(1),clause(1)),[]),proof(temporal_window(before,pred(wait,[named('John')]),anchor(named('Therapy-start')),interval(quantity_bound(geq,closed,quantity(integer(1),unit(week))),quantity_bound(leq,closed,quantity(integer(4),unit(week))))),fact_id(sentence(2),clause(1)),[])]).
