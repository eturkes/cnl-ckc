cnl_ir_record(2).
document(docid('cdc2022-opioid-rec07'),source_sha256('703b611339c19a38f1eede135c1c9f799f4cdbd81b36d676e614dd8ca42b89d2'),ulex(sha256('36b580ac58e5ca0eb85e47ffe01815a27b3a38cd87ee4a8d63d151a4a5dd63a9'))).
fact(fact_id(sentence(1),clause(1)),pred('follow-up-clinician',[named('Rec07-clinician')]),source(sentence(1),tokens([2,4]))).
rule(rule_id(sentence(2),clause(1)),pred('reevaluate-benefits-and-risks',[var(1)]),body([pred('follow-up-clinician',[var(1)])]),source(sentence(2),tokens([2,3]))).
query(query_id(sentence(3),clause(1)),pred('reevaluate-benefits-and-risks',[named('Rec07-clinician')]),source(sentence(3),tokens([3]))).
