cnl_program_record(2).
document(docid('cdc2022-opioid-rec10'),source_sha256('0e35951999fa969405448e75e86a014aed0d53a5f3e8474e19324221bbe09640'),ulex(sha256('36b580ac58e5ca0eb85e47ffe01815a27b3a38cd87ee4a8d63d151a4a5dd63a9'))).
clause(fact_id(sentence(1),clause(1)),pred('toxicology-testing-clinician',[named('Rec10-clinician')]),body([])).
clause(fact_id(sentence(2),clause(1)),pred('acute-pain-clinician',[named('Rec10-peer')]),body([])).
clause(rule_id(sentence(3),clause(1)),pred('consider-toxicology-testing',[var(1)]),body([pred('toxicology-testing-clinician',[var(1)])])).
goal(query_id(sentence(4),clause(1)),pred('consider-toxicology-testing',[named('Rec10-peer')])).
