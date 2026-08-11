% cdc2022-opioid-rec04-imp03.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec04-imp03',ace_sha256('85f664fca16058edf75fa85d3650dd48156e06abb650cba3057b5fa2e40e983b'),ulex(sha256('26c94eb4c3e425db0b6c278b6861f816b856b8629608554394634e35ee8e1f3c'))).
% S1: Rec04-imp03-clinician is an opioid-dosage-clinician.
'opioid-dosage-clinician'('Rec04-imp03-clinician').
% S2: Every opioid-dosage-clinician applies-opioid-dosage-recommendations-to-starting-or-increasing-opioids-and-separately-assesses-dosage-reduction-benefits-and-risks.
'apply-opioid-dosage-recommendations-to-starting-or-increasing-opioids-and-separately-assess-dosage-reduction-benefits-and-risks'(A) :- 'opioid-dosage-clinician'(A).
% S3: Does Rec04-imp03-clinician apply-opioid-dosage-recommendations-to-starting-or-increasing-opioids-and-separately-assess-dosage-reduction-benefits-and-risks?
guideline_query(yesno,'apply-opioid-dosage-recommendations-to-starting-or-increasing-opioids-and-separately-assess-dosage-reduction-benefits-and-risks'('Rec04-imp03-clinician')).
