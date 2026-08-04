cnl_answer_record(3).
document(docid('m6u3-constructors'),source_sha256('2222222222222222222222222222222222222222222222222222222222222222'),ulex(none)).
program(sha256('e46964db69300006a6b1ed6b712b5de09ddc4923d276882837c992adfba4ae10')).
answer(query_id(sentence(3),clause(1)),pred(eligible,[named('John')]),proved).
proof(pred(eligible,[named('John')]),rule_id(sentence(2),clause(1)),[proof(pred(adult,[named('John')]),fact_id(sentence(1),clause(1)),[]),proof(pred(have,[named('John'),quantity(integer(70),unit(year))]),fact_id(sentence(1),clause(2)),[]),quantity_compare(quantity(integer(70),unit(year)),quantity_bound(greater,open,quantity(integer(64),unit(year)))),proof(temporal(during,pred(wait,[named('John')]),anchor(named('Therapy-start'))),fact_id(sentence(1),clause(6)),[])]).
