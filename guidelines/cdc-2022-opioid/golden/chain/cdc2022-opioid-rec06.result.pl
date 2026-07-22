cnl_answer_record(2).
document(docid('cdc2022-opioid-rec06'),source_sha256('4f45424d645b948f4e32145ed673c2718c58129c0325962658bab91b1d7291dd'),ulex(sha256('36b580ac58e5ca0eb85e47ffe01815a27b3a38cd87ee4a8d63d151a4a5dd63a9'))).
program(sha256('a563df113dd1ba128b535646cb3acbc5dc4e58ebb5afc913110bc98b42392ccd')).
answer(query_id(sentence(3),clause(1)),pred('limit-prescription-quantity',[named('Rec06-clinician')]),proved).
proof(pred('limit-prescription-quantity',[named('Rec06-clinician')]),rule_id(sentence(2),clause(1)),[proof(pred('acute-opioid-clinician',[named('Rec06-clinician')]),fact_id(sentence(1),clause(1)),[])]).
