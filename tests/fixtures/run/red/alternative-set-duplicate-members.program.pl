cnl_program_record(3).
document(docid('probe-alt-duplicate'),source_sha256('aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred(patient,[named('John')]),body([])).
alternative_set(alternative_set_id(sentence(2),clause(1)),members([pred(offer,[var(1),named('Mary')]),pred(offer,[var(1),named('Mary')])]),body([pred(patient,[var(1)])]),satisfaction(any_member),exclusivity(not_asserted),exhaustiveness(not_asserted)).
goal(query_id(sentence(3),clause(1)),pred(patient,[named('John')])).
