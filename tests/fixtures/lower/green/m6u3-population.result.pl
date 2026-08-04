cnl_answer_record(3).
document(docid('population-numeric'),source_sha256('0000000000000000000000000000000000000000000000000000000000000000'),ulex(none)).
program(sha256('ab11356de1226de6dd9bbe014a164578d5d995cc4a7184979fb288a6374464d0')).
answer(query_id(sentence(4),clause(1)),pred(wait,[named('John')]),proved).
proof(pred(wait,[named('John')]),rule_id(sentence(3),clause(1)),[proof(pred(adult,[named('John')]),fact_id(sentence(1),clause(1)),[]),proof(pred(have,[named('John'),quantity(integer(70),unit(year))]),fact_id(sentence(2),clause(1)),[]),quantity_compare(quantity(integer(70),unit(year)),quantity_bound(greater,open,quantity(integer(64),unit(year))))]).
