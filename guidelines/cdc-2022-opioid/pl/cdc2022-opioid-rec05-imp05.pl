% cdc2022-opioid-rec05-imp05.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec05-imp05',ace_sha256('231bbeb9bff6d0d44bbd8cfe136ee22957af99a169ab86626c43b0cd75fcae8c'),ulex(sha256(c879b5696dbd7394030ca8105226a5d860b57777791e2f4772f9325c07278683))).
% S1: Rec05-imp05-clinician is a dosage-change-clinician.
'dosage-change-clinician'('Rec05-imp05-clinician').
% S2: Every dosage-change-clinician acknowledges-discordance-expresses-empathy-and-implements-patient-centered-treatment-changes-while-avoiding-abandonment-when-benefit-risk-consensus-is-unavailable.
'acknowledge-discordance-express-empathy-and-implement-patient-centered-treatment-changes-while-avoiding-abandonment-when-benefit-risk-consensus-is-unavailable'(A) :- 'dosage-change-clinician'(A).
% S3: Does Rec05-imp05-clinician acknowledge-discordance-express-empathy-and-implement-patient-centered-treatment-changes-while-avoiding-abandonment-when-benefit-risk-consensus-is-unavailable?
guideline_query(yesno,'acknowledge-discordance-express-empathy-and-implement-patient-centered-treatment-changes-while-avoiding-abandonment-when-benefit-risk-consensus-is-unavailable'('Rec05-imp05-clinician')).
