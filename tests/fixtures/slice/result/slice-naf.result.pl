cnl_answer_record(2).
document(docid('slice-naf'),source_sha256('074d6ca7f0e5127e06af01f24a04ce434010ed1e9e80613a85fd9ad81f78ff6e'),ulex(sha256('7be3ff7a729f2d12bbc7d204b70ab93c419f936f6ad751afd8018c3c09cc0bdc'))).
program(sha256('ba25a7498095c49e8c249374844cf035f9a54d5e70411f9055fe48dd97d9375d')).
answer(query_id(sentence(3),clause(1)),pred(recover,[named('John')]),proved).
proof(pred(recover,[named('John')]),rule_id(sentence(2),clause(1)),[proof(pred(patient,[named('John')]),fact_id(sentence(1),clause(1)),[]),naf(pred(smoke,[named('John')]))]).
