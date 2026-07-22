cnl_program_record(2).
document(docid('cdc2022-opioid-rec11'),source_sha256('82f481566c93cebc9a5612e09a417531cd0283da485fcfcd82043a9573ddb32c'),ulex(sha256('36b580ac58e5ca0eb85e47ffe01815a27b3a38cd87ee4a8d63d151a4a5dd63a9'))).
clause(fact_id(sentence(1),clause(1)),pred('concurrent-medication-clinician',[named('Rec11-clinician')]),body([])).
clause(rule_id(sentence(2),clause(1)),pred('use-particular-caution',[var(1)]),body([pred('concurrent-medication-clinician',[var(1)])])).
goal(query_id(sentence(3),clause(1)),pred('use-particular-caution',[named('Rec11-clinician')])).
