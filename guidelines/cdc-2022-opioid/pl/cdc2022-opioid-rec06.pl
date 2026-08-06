% cdc2022-opioid-rec06.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec06',ace_sha256('190146a9ecc7a3a6357cf2233df9365d445c9c101b676359ec7f9e8aabd687f3'),ulex(sha256('7855e6f992b6a70f49da5dbd1de6620893b143919a2f28e6b50d67c2eabb7dff'))).
% S1: Rec06-clinician is an acute-opioid-clinician.
'acute-opioid-clinician'('Rec06-clinician').
% S2: Rec06 is a category-a-recommendation.
'category-a-recommendation'('Rec06').
% S3: Rec06 is an evidence-type-4-recommendation.
'evidence-type-4-recommendation'('Rec06').
% S4: Every acute-opioid-clinician limits-prescription-quantity.
'limit-prescription-quantity'(A) :- 'acute-opioid-clinician'(A).
% S5: Does Rec06-clinician limit-prescription-quantity?
guideline_query(yesno,'limit-prescription-quantity'('Rec06-clinician')).
% S6: Is Rec06 a category-a-recommendation?
guideline_query(yesno,'category-a-recommendation'('Rec06')).
% S7: Is Rec06 an evidence-type-4-recommendation?
guideline_query(yesno,'evidence-type-4-recommendation'('Rec06')).
