cnl_answer_record(3).
document(docid('branch-alternative-composition'),source_sha256('bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb'),ulex(none)).
program(sha256('6a1388022b4cfe4b32bc1ef4f37dce016523dbf780647e12c9479d8a51b86d2e')).
alternative_set(alternative_set_id(sentence(3),clause(1),branch(1)),members([pred(offer,[var(1),named('Mary')]),pred(arrange,[var(1),named('Mary')])]),body([pred(patient,[var(1)]),pred(wait,[var(1)])]),satisfaction(any_member),exclusivity(not_asserted),exhaustiveness(not_asserted)).
alternative_set(alternative_set_id(sentence(3),clause(1),branch(2)),members([pred(offer,[var(1),named('Mary')]),pred(arrange,[var(1),named('Mary')])]),body([pred(patient,[var(1)]),pred(sleep,[var(1)])]),satisfaction(any_member),exclusivity(not_asserted),exhaustiveness(not_asserted)).
answer(query_id(sentence(4),clause(1)),pred(wait,[named('John')]),proved).
proof(pred(wait,[named('John')]),fact_id(sentence(2),clause(1)),[]).
