% cdc2022-opioid-rec04-imp03.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec04-imp03',ace_sha256('85f664fca16058edf75fa85d3650dd48156e06abb650cba3057b5fa2e40e983b'),ulex(sha256('453749766e15fefa9c2dbb81ef07028b238ee2cb462770b45c2c584b7cbc92e7'))).
% S1: Rec04-imp03-clinician is an opioid-dosage-clinician.
'opioid-dosage-clinician'('Rec04-imp03-clinician').
% S2: Every opioid-dosage-clinician applies-opioid-dosage-recommendations-to-starting-or-increasing-opioids-and-separately-assesses-dosage-reduction-benefits-and-risks.
'apply-opioid-dosage-recommendations-to-starting-or-increasing-opioids-and-separately-assess-dosage-reduction-benefits-and-risks'(A) :- 'opioid-dosage-clinician'(A).
% S3: Does Rec04-imp03-clinician apply-opioid-dosage-recommendations-to-starting-or-increasing-opioids-and-separately-assess-dosage-reduction-benefits-and-risks?
guideline_query(yesno,'apply-opioid-dosage-recommendations-to-starting-or-increasing-opioids-and-separately-assess-dosage-reduction-benefits-and-risks'('Rec04-imp03-clinician')).
