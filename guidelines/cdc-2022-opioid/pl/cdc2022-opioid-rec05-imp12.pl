% cdc2022-opioid-rec05-imp12.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec05-imp12',ace_sha256('0dff2ea49ac974f6f1adcdb7837f67d04c08fbd0d8ed9885d52a7609004696a9'),ulex(sha256(c879b5696dbd7394030ca8105226a5d860b57777791e2f4772f9325c07278683))).
% S1: Rec05-imp12-clinician is a dosage-change-clinician.
'dosage-change-clinician'('Rec05-imp12-clinician').
% S2: Every dosage-change-clinician considers-months-to-years-individualized-taper-for-at-least-one-year-opioid-therapy-based-on-dosage-patient-goals-and-concerns.
'consider-months-to-years-individualized-taper-for-at-least-one-year-opioid-therapy-based-on-dosage-patient-goals-and-concerns'(A) :- 'dosage-change-clinician'(A).
% S3: Does Rec05-imp12-clinician consider-months-to-years-individualized-taper-for-at-least-one-year-opioid-therapy-based-on-dosage-patient-goals-and-concerns?
guideline_query(yesno,'consider-months-to-years-individualized-taper-for-at-least-one-year-opioid-therapy-based-on-dosage-patient-goals-and-concerns'('Rec05-imp12-clinician')).
