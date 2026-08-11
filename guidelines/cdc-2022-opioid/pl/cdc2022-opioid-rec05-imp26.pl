% cdc2022-opioid-rec05-imp26.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec05-imp26',ace_sha256('21fa379d0ea382d5643abe22c1a2e72af4aa070da5a5730843d2149359cd5462'),ulex(sha256('26c94eb4c3e425db0b6c278b6861f816b856b8629608554394634e35ee8e1f3c'))).
% S1: Rec05-imp26-clinician is a dosage-change-clinician.
'dosage-change-clinician'('Rec05-imp26-clinician').
% S2: Every dosage-change-clinician considers-communication-pain-management-behavioral-support-and-slower-taper-principles-for-shorter-duration-opioid-discontinuation.
'consider-communication-pain-management-behavioral-support-and-slower-taper-principles-for-shorter-duration-opioid-discontinuation'(A) :- 'dosage-change-clinician'(A).
% S3: Does Rec05-imp26-clinician consider-communication-pain-management-behavioral-support-and-slower-taper-principles-for-shorter-duration-opioid-discontinuation?
guideline_query(yesno,'consider-communication-pain-management-behavioral-support-and-slower-taper-principles-for-shorter-duration-opioid-discontinuation'('Rec05-imp26-clinician')).
