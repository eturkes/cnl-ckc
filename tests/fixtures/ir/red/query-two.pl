cnl_ir_record(2).
document(docid('red'),source_sha256('0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef'),ulex(none)).
fact(fact_id(sentence(1),clause(1)),pred(p,[named(alice)]),source(sentence(1),tokens([1]))).
query(query_id(sentence(2),clause(1)),pred(p,[named(alice)]),source(sentence(2),tokens([1]))).
query(query_id(sentence(3),clause(1)),pred(p,[named(bob)]),source(sentence(3),tokens([1]))).
