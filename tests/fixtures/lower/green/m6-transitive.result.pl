cnl_answer_record(3).
document(docid('m6-transitive'),source_sha256('3234b9d178211bd8e791ee9c3e37ddbf8ae02d198c0ea200cf48e9a19737af11'),ulex(none)).
program(sha256('b1cf1deb42cc39dd367d42aaea98687dfa543f2795ee61c20beb09bf688ff5a7')).
answer(query_id(sentence(4),clause(1)),pred(help,[named('John'),named('Mary')]),proved).
proof(pred(help,[named('John'),named('Mary')]),rule_id(sentence(3),clause(1)),[proof(pred(patient,[named('John')]),fact_id(sentence(2),clause(1)),[]),proof(pred(like,[named('John'),named('Mary')]),fact_id(sentence(1),clause(1)),[])]).
