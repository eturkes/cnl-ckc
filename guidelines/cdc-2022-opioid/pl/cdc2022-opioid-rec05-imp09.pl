% cdc2022-opioid-rec05-imp09.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec05-imp09',ace_sha256(af47e17359aa26a195aea4918e92c38949ec4fe270099bc7b7bec3f3626b2bbe),ulex(sha256(c879b5696dbd7394030ca8105226a5d860b57777791e2f4772f9325c07278683))).
% S1: Rec05-imp09-team-member is a taper-support-team-member.
'taper-support-team-member'('Rec05-imp09-team-member').
% S2: Every taper-support-team-member considers-supporting-clinician-and-patient-during-taper-through-telephone-telehealth-or-face-to-face-visits.
'consider-supporting-clinician-and-patient-during-taper-through-telephone-telehealth-or-face-to-face-visits'(A) :- 'taper-support-team-member'(A).
% S3: Does Rec05-imp09-team-member consider-supporting-clinician-and-patient-during-taper-through-telephone-telehealth-or-face-to-face-visits?
guideline_query(yesno,'consider-supporting-clinician-and-patient-during-taper-through-telephone-telehealth-or-face-to-face-visits'('Rec05-imp09-team-member')).
