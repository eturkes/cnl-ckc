cnl_answer_record(3).
document(docid('cdc2022-opioid-rec06'),source_sha256('4f45424d645b948f4e32145ed673c2718c58129c0325962658bab91b1d7291dd'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
program(sha256('c406a0d29469f74fde48aa53a1306c998fc1f3f9bbb71d2c7321b3d041bc68ea')).
answer(query_id(sentence(3),clause(1)),pred('limit-prescription-quantity',[named('Rec06-clinician')]),proved).
proof(pred('limit-prescription-quantity',[named('Rec06-clinician')]),rule_id(sentence(2),clause(1)),[proof(pred('acute-opioid-clinician',[named('Rec06-clinician')]),fact_id(sentence(1),clause(1)),[])]).
