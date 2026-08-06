% cdc2022-opioid-rec05-imp16.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec05-imp16',ace_sha256('4b68e8186ef11a6b01f24af3f943680b3a352903a6fc7c26f117f5d956380904'),ulex(sha256('453749766e15fefa9c2dbb81ef07028b238ee2cb462770b45c2c584b7cbc92e7'))).
% S1: Rec05-imp16-clinician is a dosage-change-clinician.
'dosage-change-clinician'('Rec05-imp16-clinician').
% S2: Every dosage-change-clinician assesses-and-discusses-increasing-dosage-benefits-and-risks-with-patient-before-reversing-taper.
'assess-and-discuss-increasing-dosage-benefits-and-risks-with-patient-before-reversing-taper'(A) :- 'dosage-change-clinician'(A).
% S3: Does Rec05-imp16-clinician assess-and-discuss-increasing-dosage-benefits-and-risks-with-patient-before-reversing-taper?
guideline_query(yesno,'assess-and-discuss-increasing-dosage-benefits-and-risks-with-patient-before-reversing-taper'('Rec05-imp16-clinician')).
