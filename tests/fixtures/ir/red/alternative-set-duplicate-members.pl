cnl_ir_record(3).
document(docid('probe-alt-duplicate'),source_sha256('aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'),ulex(none)).
fact(fact_id(sentence(1),clause(1)),pred(patient,[named('John')]),source(sentence(1),tokens([2]))).
alternative_set(alternative_set_id(sentence(2),clause(1)),members([pred(offer,[var(1),named('Mary')]),pred(offer,[var(1),named('Mary')])]),body([pred(patient,[var(1)])]),satisfaction(any_member),exclusivity(not_asserted),exhaustiveness(not_asserted),source(sentence(2),tokens([2,3,5]))).
query(query_id(sentence(3),clause(1)),pred(patient,[named('John')]),source(sentence(3),tokens([3]))).
