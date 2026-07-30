cnl_program_record(3).
document(docid('m6-alternative-set'),source_sha256('3333333333333333333333333333333333333333333333333333333333333333'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred(patient,[named('John')]),body([])).
clause(fact_id(sentence(2),clause(1)),pred(offer,[named('John'),named('Mary')]),body([])).
alternative_set(alternative_set_id(sentence(3),clause(1)),members([pred(offer,[var(1),named('Mary')])]),body([pred(patient,[var(1)])]),satisfaction(any_member),exclusivity(not_asserted),exhaustiveness(not_asserted)).
goal(query_id(sentence(4),clause(1)),pred(offer,[named('John'),named('Mary')])).
