% cdc2022-opioid-rec08.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec08',ace_sha256('6ffee2042bcb7e56f9630b479f08b9dd97a770b7350afca246e2105c05ade0b6'),ulex(sha256('7855e6f992b6a70f49da5dbd1de6620893b143919a2f28e6b50d67c2eabb7dff'))).
% S1: Rec08-clinician is a risk-mitigation-clinician.
'risk-mitigation-clinician'('Rec08-clinician').
% S2: Rec08 is a category-a-recommendation.
'category-a-recommendation'('Rec08').
% S3: Rec08 is an evidence-type-4-recommendation.
'evidence-type-4-recommendation'('Rec08').
% S4: Every risk-mitigation-clinician offers-naloxone.
'offer-naloxone'(A) :- 'risk-mitigation-clinician'(A).
% S5: Who offers-naloxone?
guideline_query(who(A),'offer-naloxone'(A)).
% S6: Is Rec08 a category-a-recommendation?
guideline_query(yesno,'category-a-recommendation'('Rec08')).
% S7: Is Rec08 an evidence-type-4-recommendation?
guideline_query(yesno,'evidence-type-4-recommendation'('Rec08')).
