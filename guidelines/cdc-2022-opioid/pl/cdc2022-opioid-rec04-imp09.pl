% cdc2022-opioid-rec04-imp09.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec04-imp09',ace_sha256('2b7a59381fdc45ce216c12ddf7bfff63321615e78a72d3bf848989df835906ff'),ulex(sha256(c879b5696dbd7394030ca8105226a5d860b57777791e2f4772f9325c07278683))).
% S1: Rec04-imp09-clinician is an opioid-dosage-clinician.
'opioid-dosage-clinician'('Rec04-imp09-clinician').
% S2: Every opioid-dosage-clinician evaluates-further-dosage-increase-through-individualized-benefit-risk-assessment.
'evaluate-further-dosage-increase-through-individualized-benefit-risk-assessment'(A) :- 'opioid-dosage-clinician'(A).
% S3: Does Rec04-imp09-clinician evaluate-further-dosage-increase-through-individualized-benefit-risk-assessment?
guideline_query(yesno,'evaluate-further-dosage-increase-through-individualized-benefit-risk-assessment'('Rec04-imp09-clinician')).
