cnl_program_record(3).
document(docid('qualitative-dose-high'),source_sha256('0000000000000000000000000000000000000000000000000000000000000000'),ulex(none)).
clause(fact_id(sentence(1),clause(1)),pred(dosage,[named('Dose')]),body([])).
clause(fact_id(sentence(1),clause(2)),pred('high comp',[named('Dose')]),body([])).
clause(rule_id(sentence(2),clause(1)),pred(preferred,[var(1)]),body([pred(dosage,[var(1)]),pred('high comp',[var(1)])])).
goal(query_id(sentence(3),clause(1)),pred(preferred,[named('Dose')])).
