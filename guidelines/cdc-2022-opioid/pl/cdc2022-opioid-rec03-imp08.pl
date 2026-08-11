% cdc2022-opioid-rec03-imp08.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec03-imp08',ace_sha256('47fc0037b6db8b05895893264b4282d6fb05e70250198295e114cc974b973810'),ulex(sha256(c879b5696dbd7394030ca8105226a5d860b57777791e2f4772f9325c07278683))).
% S1: Rec03-imp08-clinician is a starting-opioid-clinician.
'starting-opioid-clinician'('Rec03-imp08-clinician').
% S2: Every starting-opioid-clinician considers-transdermal-fentanyl-only-with-dosing-and-absorption-familiarity-and-preparation-for-patient-education.
'consider-transdermal-fentanyl-only-with-dosing-and-absorption-familiarity-and-preparation-for-patient-education'(A) :- 'starting-opioid-clinician'(A).
% S3: Does Rec03-imp08-clinician consider-transdermal-fentanyl-only-with-dosing-and-absorption-familiarity-and-preparation-for-patient-education?
guideline_query(yesno,'consider-transdermal-fentanyl-only-with-dosing-and-absorption-familiarity-and-preparation-for-patient-education'('Rec03-imp08-clinician')).
