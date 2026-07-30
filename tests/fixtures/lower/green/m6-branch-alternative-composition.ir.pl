cnl_ir_record(3).
document(docid('branch-alternative-composition'),source_sha256('bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb'),ulex(none)).
fact(fact_id(sentence(1),clause(1)),pred(patient,[named('John')]),source(sentence(1),tokens([2,4]))).
fact(fact_id(sentence(2),clause(1)),pred(wait,[named('John')]),source(sentence(2),tokens([2]))).
alternative_set(alternative_set_id(sentence(3),clause(1),branch(1)),members([pred(offer,[var(1),named('Mary')]),pred(arrange,[var(1),named('Mary')])]),body([pred(patient,[var(1)]),pred(wait,[var(1)])]),satisfaction(any_member),exclusivity(not_asserted),exhaustiveness(not_asserted),source(sentence(3),tokens([2,4,8,11]))).
alternative_set(alternative_set_id(sentence(3),clause(1),branch(2)),members([pred(offer,[var(1),named('Mary')]),pred(arrange,[var(1),named('Mary')])]),body([pred(patient,[var(1)]),pred(sleep,[var(1)])]),satisfaction(any_member),exclusivity(not_asserted),exhaustiveness(not_asserted),source(sentence(3),tokens([2,7,8,11]))).
query(query_id(sentence(4),clause(1)),pred(wait,[named('John')]),source(sentence(4),tokens([3]))).
