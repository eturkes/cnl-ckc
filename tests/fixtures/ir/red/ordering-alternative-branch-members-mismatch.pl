cnl_ir_record(3).
document(docid('ordering-alternative-branch-members-mismatch'),source_sha256('aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'),ulex(none)).
fact(fact_id(sentence(1),clause(1)),pred(patient,[named('John')]),source(sentence(1),tokens([2,4]))).
alternative_set(alternative_set_id(sentence(3),clause(1),branch(1)),members([pred(offer,[var(1),named('Mary')]),pred(arrange,[var(1),named('Mary')])]),body([pred(patient,[var(1)])]),satisfaction(any_member),exclusivity(not_asserted),exhaustiveness(not_asserted),source(sentence(3),tokens([2,3,6]))).
alternative_set(alternative_set_id(sentence(3),clause(1),branch(2)),members([pred(offer,[var(1),named('Mary')]),pred(refer,[var(1),named('Mary')])]),body([pred(patient,[var(1)])]),satisfaction(any_member),exclusivity(not_asserted),exhaustiveness(not_asserted),source(sentence(3),tokens([2,3,9]))).
query(query_id(sentence(4),clause(1)),pred(patient,[named('John')]),source(sentence(4),tokens([3]))).
