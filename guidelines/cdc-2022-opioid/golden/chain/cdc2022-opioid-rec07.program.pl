cnl_program_record(2).
document(docid('cdc2022-opioid-rec07'),source_sha256('703b611339c19a38f1eede135c1c9f799f4cdbd81b36d676e614dd8ca42b89d2'),ulex(sha256('36b580ac58e5ca0eb85e47ffe01815a27b3a38cd87ee4a8d63d151a4a5dd63a9'))).
clause(fact_id(sentence(1),clause(1)),pred('follow-up-clinician',[named('Rec07-clinician')]),body([])).
clause(rule_id(sentence(2),clause(1)),pred('reevaluate-benefits-and-risks',[var(1)]),body([pred('follow-up-clinician',[var(1)])])).
goal(query_id(sentence(3),clause(1)),pred('reevaluate-benefits-and-risks',[named('Rec07-clinician')])).
