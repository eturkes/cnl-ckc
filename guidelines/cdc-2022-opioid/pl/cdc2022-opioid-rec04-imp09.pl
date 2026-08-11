% cdc2022-opioid-rec04-imp09.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec04-imp09',ace_sha256('2b7a59381fdc45ce216c12ddf7bfff63321615e78a72d3bf848989df835906ff'),ulex(sha256('26c94eb4c3e425db0b6c278b6861f816b856b8629608554394634e35ee8e1f3c'))).
% S1: Rec04-imp09-clinician is an opioid-dosage-clinician.
'opioid-dosage-clinician'('Rec04-imp09-clinician').
% S2: Every opioid-dosage-clinician evaluates-further-dosage-increase-through-individualized-benefit-risk-assessment.
'evaluate-further-dosage-increase-through-individualized-benefit-risk-assessment'(A) :- 'opioid-dosage-clinician'(A).
% S3: Does Rec04-imp09-clinician evaluate-further-dosage-increase-through-individualized-benefit-risk-assessment?
guideline_query(yesno,'evaluate-further-dosage-increase-through-individualized-benefit-risk-assessment'('Rec04-imp09-clinician')).
