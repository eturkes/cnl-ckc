cnl_ir_record(2).
document(docid('cdc2022-opioid-rec03'),source_sha256('65bafa8f9ffab76dd476a936f9b124e6baf210182ed1e23b4314ea3268db2e7f'),ulex(sha256('36b580ac58e5ca0eb85e47ffe01815a27b3a38cd87ee4a8d63d151a4a5dd63a9'))).
fact(fact_id(sentence(1),clause(1)),pred('starting-opioid-clinician',[named('Rec03-clinician')]),source(sentence(1),tokens([2,4]))).
rule(rule_id(sentence(2),clause(1)),pred('prescribe-immediate-release',[var(1)]),body([pred('starting-opioid-clinician',[var(1)])]),source(sentence(2),tokens([2,3]))).
query(query_id(sentence(3),clause(1)),pred('prescribe-immediate-release',[named('Rec03-clinician')]),source(sentence(3),tokens([3]))).
