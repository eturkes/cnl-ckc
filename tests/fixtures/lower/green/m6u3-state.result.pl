cnl_answer_record(3).
document(docid('state-ongoing'),source_sha256('0000000000000000000000000000000000000000000000000000000000000000'),ulex(none)).
program(sha256('5e77a8781548d60d407d619388932ffd54510128869d72074a851ecfad2f4cf9')).
answer(query_id(sentence(3),clause(1)),pred(wait,[named('John')]),proved).
proof(pred(wait,[named('John')]),rule_id(sentence(2),clause(1)),[proof(pred(patient,[named('John')]),fact_id(sentence(1),clause(1)),[]),proof(pred(ongoing,[named('John')]),fact_id(sentence(1),clause(2)),[])]).
