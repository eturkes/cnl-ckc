cnl_answer_record(3).
document(docid('m6-labeled-exception'),source_sha256('2222222222222222222222222222222222222222222222222222222222222222'),ulex(none)).
program(sha256('6c877917eac3c8d0f575adc6bf0fbeafaca7bef8b5d25938bd09527d0805a1b2')).
answer(query_id(sentence(4),clause(1)),pred(recover,[named('John')]),proved).
proof(pred(recover,[named('John')]),rule_id(sentence(3),clause(1)),[proof(pred(patient,[named('John')]),fact_id(sentence(1),clause(1)),[]),naf(exception_id(rule(rule_id(sentence(3),clause(1))),literal(2)),pred(smoker,[named('John')]))]).
