cnl_ir_record(3).
document(docid('m6-alternative-set'),source_sha256('3333333333333333333333333333333333333333333333333333333333333333'),ulex(none)).
fact(fact_id(sentence(1),clause(1)),pred(patient,[named('John')]),source(sentence(1),tokens([2,4]))).
fact(fact_id(sentence(2),clause(1)),pred(offer,[named('John'),named('Mary')]),source(sentence(2),tokens([2]))).
alternative_set(alternative_set_id(sentence(3),clause(1)),members([pred(offer,[var(1),named('Mary')]),pred(arrange,[var(1),named('Mary')])]),body([pred(patient,[var(1)])]),satisfaction(all_members),exclusivity(not_asserted),exhaustiveness(not_asserted),source(sentence(3),tokens([2,3,6]))).
query(query_id(sentence(4),clause(1)),pred(offer,[named('John'),named('Mary')]),source(sentence(4),tokens([3]))).
