cnl_ir_record(2).
document(docid('cdc2022-opioid-rec06'),source_sha256('4f45424d645b948f4e32145ed673c2718c58129c0325962658bab91b1d7291dd'),ulex(sha256('36b580ac58e5ca0eb85e47ffe01815a27b3a38cd87ee4a8d63d151a4a5dd63a9'))).
fact(fact_id(sentence(1),clause(1)),pred('acute-opioid-clinician',[named('Rec06-clinician')]),source(sentence(1),tokens([2,4]))).
rule(rule_id(sentence(2),clause(1)),pred('limit-prescription-quantity',[var(1)]),body([pred('acute-opioid-clinician',[var(1)])]),source(sentence(2),tokens([2,3]))).
query(query_id(sentence(3),clause(1)),pred('limit-prescription-quantity',[named('Rec06-clinician')]),source(sentence(3),tokens([3]))).
