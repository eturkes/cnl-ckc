% cdc2022-opioid-rec05-imp15.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec05-imp15',ace_sha256('567a00eab3f92fbe715f8fb9b291648ba40d49f9d45176d8f26ab55556f0f0fe'),ulex(sha256(c879b5696dbd7394030ca8105226a5d860b57777791e2f4772f9325c07278683))).
% S1: Rec05-imp15-clinician is a dosage-change-clinician.
'dosage-change-clinician'('Rec05-imp15-clinician').
% S2: Every dosage-change-clinician considers-pausing-and-restarting-tapers-when-patient-is-ready-and-slowing-tapers-near-low-dosages.
'consider-pausing-and-restarting-tapers-when-patient-is-ready-and-slowing-tapers-near-low-dosages'(A) :- 'dosage-change-clinician'(A).
% S3: Does Rec05-imp15-clinician consider-pausing-and-restarting-tapers-when-patient-is-ready-and-slowing-tapers-near-low-dosages?
guideline_query(yesno,'consider-pausing-and-restarting-tapers-when-patient-is-ready-and-slowing-tapers-near-low-dosages'('Rec05-imp15-clinician')).
