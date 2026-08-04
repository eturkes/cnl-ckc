cnl_ir_record(3).
document(docid('m6u3-quantity-cross-unit'),source_sha256('3333333333333333333333333333333333333333333333333333333333333333'),ulex(none)).
rule(rule_id(sentence(1),clause(1)),pred(eligible,[named(a)]),body([quantity_compare(quantity(integer(70),unit(year)),quantity_bound(greater,open,quantity(integer(64),unit(day))))]),source(sentence(1),tokens([1]))).
query(query_id(sentence(2),clause(1)),pred(eligible,[named(a)]),source(sentence(2),tokens([1]))).
