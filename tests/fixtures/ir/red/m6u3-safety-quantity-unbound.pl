cnl_ir_record(3).
document(docid('m6u3-safety-quantity-unbound'),source_sha256('3333333333333333333333333333333333333333333333333333333333333333'),ulex(none)).
rule(rule_id(sentence(1),clause(1)),pred(eligible,[var(1)]),body([pred(adult,[var(1)]),quantity_compare(var(2),quantity_bound(greater,open,quantity(integer(64),unit(year))))]),source(sentence(1),tokens([1]))).
query(query_id(sentence(2),clause(1)),pred(eligible,[named(a)]),source(sentence(2),tokens([1]))).
