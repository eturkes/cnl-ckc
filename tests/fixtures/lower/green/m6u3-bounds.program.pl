cnl_program_record(3).
document(docid('quantity-bounds'),source_sha256('5555555555555555555555555555555555555555555555555555555555555555'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred('eq-dose',[quantity_bound(eq,closed,quantity(integer(5),unit(mg)))]),body([])).
clause(fact_id(sentence(1),clause(2)),pred('geq-dose',[quantity_bound(geq,closed,quantity(integer(5),unit(mg)))]),body([])).
clause(fact_id(sentence(1),clause(3)),pred('leq-dose',[quantity_bound(leq,closed,quantity(integer(5),unit(mg)))]),body([])).
clause(fact_id(sentence(1),clause(4)),pred('greater-dose',[quantity_bound(greater,open,quantity(integer(5),unit(mg)))]),body([])).
clause(fact_id(sentence(1),clause(5)),pred('less-dose',[quantity_bound(less,open,quantity(integer(5),unit(mg)))]),body([])).
goal(query_id(sentence(2),clause(1)),pred('leq-dose',[quantity_bound(leq,closed,quantity(integer(5),unit(mg)))])).
