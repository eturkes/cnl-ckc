% cdc2022-opioid-rec05.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec05',ace_sha256(ab806900a94e8d674028534465fd08ac53c54b48c61c038f8a179b14d08af27c),ulex(sha256('453749766e15fefa9c2dbb81ef07028b238ee2cb462770b45c2c584b7cbc92e7'))).
:- dynamic('detect-life-threatening-warning'/1).
% S1: Rec05-clinician is a dosage-change-clinician.
'dosage-change-clinician'('Rec05-clinician').
% S2: Rec05 is a category-b-recommendation.
'category-b-recommendation'('Rec05').
% S3: Rec05 is an evidence-type-4-recommendation.
'evidence-type-4-recommendation'('Rec05').
% S4: Every dosage-change-clinician that does not provably detect-life-threatening-warning avoids-abrupt-discontinuation.
'avoid-abrupt-discontinuation'(A) :- 'dosage-change-clinician'(A), \+ 'detect-life-threatening-warning'(A).
% S5: Does Rec05-clinician avoid-abrupt-discontinuation?
guideline_query(yesno,'avoid-abrupt-discontinuation'('Rec05-clinician')).
% S6: Is Rec05 a category-b-recommendation?
guideline_query(yesno,'category-b-recommendation'('Rec05')).
% S7: Is Rec05 an evidence-type-4-recommendation?
guideline_query(yesno,'evidence-type-4-recommendation'('Rec05')).
