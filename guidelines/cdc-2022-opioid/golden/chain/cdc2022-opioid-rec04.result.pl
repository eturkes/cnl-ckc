cnl_answer_record(2).
document(docid('cdc2022-opioid-rec04'),source_sha256('1a151ce7d02368aab8e51c8744194f5567199863872d64469891783ae242ead4'),ulex(sha256('36b580ac58e5ca0eb85e47ffe01815a27b3a38cd87ee4a8d63d151a4a5dd63a9'))).
program(sha256('a2b4669aab649ee1faba040c94a14c9edd75ada3ef3c4a9b74f4e5b5a8da8ba5')).
answer(query_id(sentence(3),clause(1)),pred('prescribe-lowest-effective-dosage',[named('Rec04-clinician')]),proved).
proof(pred('prescribe-lowest-effective-dosage',[named('Rec04-clinician')]),rule_id(sentence(2),clause(1)),[proof(pred('opioid-dosage-clinician',[named('Rec04-clinician')]),fact_id(sentence(1),clause(1)),[])]).
