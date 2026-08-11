% cdc2022-opioid-rec05-imp19.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec05-imp19',ace_sha256('4741a9834e1d2266fd28cdfb95f00c411d49000e9d4b24b300944a3df9756e7a'),ulex(sha256(c879b5696dbd7394030ca8105226a5d860b57777791e2f4772f9325c07278683))).
% S1: Rec05-imp19-clinician is a dosage-change-clinician.
'dosage-change-clinician'('Rec05-imp19-clinician').
% S2: Every dosage-change-clinician accesses-appropriate-expertise-when-considering-opioid-taper-during-pregnancy.
'access-appropriate-expertise-when-considering-opioid-taper-during-pregnancy'(A) :- 'dosage-change-clinician'(A).
% S3: Does Rec05-imp19-clinician access-appropriate-expertise-when-considering-opioid-taper-during-pregnancy?
guideline_query(yesno,'access-appropriate-expertise-when-considering-opioid-taper-during-pregnancy'('Rec05-imp19-clinician')).
