cnl_answer_record(3).
document(docid('temporal-before'),source_sha256('0000000000000000000000000000000000000000000000000000000000000000'),ulex(none)).
program(sha256('6b182fb30b468767396052c0ddaaa718a350e32a37f273832b49ed7ff0062c2a')).
answer(query_id(sentence(3),clause(1)),pred(recover,[named('John')]),proved).
proof(pred(recover,[named('John')]),rule_id(sentence(2),clause(1)),[proof(temporal(before,pred(wait,[named('John')]),anchor(named('Therapy-start'))),fact_id(sentence(1),clause(1)),[])]).
