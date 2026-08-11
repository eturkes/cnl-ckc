% cdc2022-opioid-rec05-imp20.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec05-imp20',ace_sha256('10fc4b151de3c08fc682182c23113e05169ced60aaa97ba5c612b99667ec62e3'),ulex(sha256('26c94eb4c3e425db0b6c278b6861f816b856b8629608554394634e35ee8e1f3c'))).
% S1: Rec05-imp20-clinician is a dosage-change-clinician.
'dosage-change-clinician'('Rec05-imp20-clinician').
% S2: Every dosage-change-clinician advises-about-overdose-risk-after-abrupt-return-to-higher-dose-provides-overdose-education-and-offers-naloxone.
'advise-about-overdose-risk-after-abrupt-return-to-higher-dose-provide-overdose-education-and-offer-naloxone'(A) :- 'dosage-change-clinician'(A).
% S3: Does Rec05-imp20-clinician advise-about-overdose-risk-after-abrupt-return-to-higher-dose-provide-overdose-education-and-offer-naloxone?
guideline_query(yesno,'advise-about-overdose-risk-after-abrupt-return-to-higher-dose-provide-overdose-education-and-offer-naloxone'('Rec05-imp20-clinician')).
