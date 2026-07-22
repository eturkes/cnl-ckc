cnl_program_record(2).
document(docid('cdc2022-opioid-rec12'),source_sha256('0e8bc74eac108b480eddf0a7510697523afee483f24ca8921ebdfafd9e03a6f5'),ulex(sha256('36b580ac58e5ca0eb85e47ffe01815a27b3a38cd87ee4a8d63d151a4a5dd63a9'))).
clause(fact_id(sentence(1),clause(1)),pred('opioid-use-disorder-clinician',[named('Rec12-clinician')]),body([])).
clause(rule_id(sentence(2),clause(1)),pred('offer-medication-treatment',[var(1)]),body([pred('opioid-use-disorder-clinician',[var(1)])])).
goal(query_id(sentence(3),clause(1)),pred('offer-medication-treatment',[named('Rec12-clinician')])).
