cnl_answer_record(3).
document(docid('quantity-compare'),source_sha256('0000000000000000000000000000000000000000000000000000000000000000'),ulex(none)).
program(sha256('1416fa55f84946e6e889300e674f08c9274cb69bc23c7216bebf4841117f04f2')).
answer(query_id(sentence(2),clause(1)),pred('great comp_than',[quantity(integer(50),unit('morphine-milligram-equivalent')),quantity(integer(40),unit('morphine-milligram-equivalent'))]),proved).
proof(pred('great comp_than',[quantity(integer(50),unit('morphine-milligram-equivalent')),quantity(integer(40),unit('morphine-milligram-equivalent'))]),rule_id(sentence(1),clause(1)),[quantity_compare(quantity(integer(50),unit('morphine-milligram-equivalent')),quantity_bound(greater,open,quantity(integer(40),unit('morphine-milligram-equivalent'))))]).
