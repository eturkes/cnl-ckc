% cdc2022-opioid-rec05-imp12.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec05-imp12',ace_sha256('0dff2ea49ac974f6f1adcdb7837f67d04c08fbd0d8ed9885d52a7609004696a9'),ulex(sha256('26c94eb4c3e425db0b6c278b6861f816b856b8629608554394634e35ee8e1f3c'))).
% S1: Rec05-imp12-clinician is a dosage-change-clinician.
'dosage-change-clinician'('Rec05-imp12-clinician').
% S2: Every dosage-change-clinician considers-months-to-years-individualized-taper-for-at-least-one-year-opioid-therapy-based-on-dosage-patient-goals-and-concerns.
'consider-months-to-years-individualized-taper-for-at-least-one-year-opioid-therapy-based-on-dosage-patient-goals-and-concerns'(A) :- 'dosage-change-clinician'(A).
% S3: Does Rec05-imp12-clinician consider-months-to-years-individualized-taper-for-at-least-one-year-opioid-therapy-based-on-dosage-patient-goals-and-concerns?
guideline_query(yesno,'consider-months-to-years-individualized-taper-for-at-least-one-year-opioid-therapy-based-on-dosage-patient-goals-and-concerns'('Rec05-imp12-clinician')).
