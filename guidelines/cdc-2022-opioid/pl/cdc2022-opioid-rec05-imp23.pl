% cdc2022-opioid-rec05-imp23.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec05-imp23',ace_sha256('4f28e9f79dc4fd30eefb9fb9b4861b0b4199b19cd45083e520229ccc01b15176'),ulex(sha256(c879b5696dbd7394030ca8105226a5d860b57777791e2f4772f9325c07278683))).
% S1: Rec05-imp23-clinician is a dosage-change-clinician.
'dosage-change-clinician'('Rec05-imp23-clinician').
% S2: Every dosage-change-clinician considers-using-periodic-strategic-motivational-questions-and-statements-to-encourage-therapeutic-changes-and-functional-goals.
'consider-using-periodic-strategic-motivational-questions-and-statements-to-encourage-therapeutic-changes-and-functional-goals'(A) :- 'dosage-change-clinician'(A).
% S3: Does Rec05-imp23-clinician consider-using-periodic-strategic-motivational-questions-and-statements-to-encourage-therapeutic-changes-and-functional-goals?
guideline_query(yesno,'consider-using-periodic-strategic-motivational-questions-and-statements-to-encourage-therapeutic-changes-and-functional-goals'('Rec05-imp23-clinician')).
