cnl_program_record(2).
document(docid('cdc2022-opioid-rec02'),source_sha256('06212d79c2a865211f33f170fdbe7d5f5554701c83a116da02fb07531b0aa790'),ulex(sha256('36b580ac58e5ca0eb85e47ffe01815a27b3a38cd87ee4a8d63d151a4a5dd63a9'))).
clause(fact_id(sentence(1),clause(1)),pred('subacute-chronic-pain-clinician',[named('Rec02-clinician')]),body([])).
clause(rule_id(sentence(2),clause(1)),pred('maximize-nonopioid-therapy',[var(1)]),body([pred('subacute-chronic-pain-clinician',[var(1)])])).
goal(query_id(sentence(3),clause(1)),pred('maximize-nonopioid-therapy',[named('Rec02-clinician')])).
