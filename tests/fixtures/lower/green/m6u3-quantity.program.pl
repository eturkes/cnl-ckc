cnl_program_record(3).
document(docid('quantity-compare'),source_sha256('0000000000000000000000000000000000000000000000000000000000000000'),ulex(none)).
clause(rule_id(sentence(1),clause(1)),pred('great comp_than',[quantity(integer(50),unit('morphine-milligram-equivalent')),quantity(integer(40),unit('morphine-milligram-equivalent'))]),body([quantity_compare(quantity(integer(50),unit('morphine-milligram-equivalent')),quantity_bound(greater,open,quantity(integer(40),unit('morphine-milligram-equivalent'))))])).
goal(query_id(sentence(2),clause(1)),pred('great comp_than',[quantity(integer(50),unit('morphine-milligram-equivalent')),quantity(integer(40),unit('morphine-milligram-equivalent'))])).
