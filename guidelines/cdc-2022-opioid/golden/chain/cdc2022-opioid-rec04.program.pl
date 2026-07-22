cnl_program_record(2).
document(docid('cdc2022-opioid-rec04'),source_sha256('1a151ce7d02368aab8e51c8744194f5567199863872d64469891783ae242ead4'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
clause(fact_id(sentence(1),clause(1)),pred('opioid-dosage-clinician',[named('Rec04-clinician')]),body([])).
clause(rule_id(sentence(2),clause(1)),pred('prescribe-lowest-effective-dosage',[var(1)]),body([pred('opioid-dosage-clinician',[var(1)])])).
goal(query_id(sentence(3),clause(1)),pred('prescribe-lowest-effective-dosage',[named('Rec04-clinician')])).
