% cdc2022-opioid-rec12.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec12',ace_sha256(ef096ece35f3a6a59bff4c95a3ca899d5c4a75187978e1f3e96b0ede56f994d3),ulex(sha256('3ffe6d9f23ab3109d7803e1121935f32d53a7768425248b943679743da1ed101'))).
% S1: Rec12-clinician is an opioid-use-disorder-clinician.
'opioid-use-disorder-clinician'('Rec12-clinician').
% S2: Rec12 is a category-a-recommendation.
'category-a-recommendation'('Rec12').
% S3: Rec12 is an evidence-type-1-recommendation.
'evidence-type-1-recommendation'('Rec12').
% S4: Every opioid-use-disorder-clinician offers-medication-treatment.
'offer-medication-treatment'(A) :- 'opioid-use-disorder-clinician'(A).
% S5: Does Rec12-clinician offer-medication-treatment?
guideline_query(yesno,'offer-medication-treatment'('Rec12-clinician')).
% S6: Is Rec12 a category-a-recommendation?
guideline_query(yesno,'category-a-recommendation'('Rec12')).
% S7: Is Rec12 an evidence-type-1-recommendation?
guideline_query(yesno,'evidence-type-1-recommendation'('Rec12')).
