% cdc2022-opioid-rec03.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec03',ace_sha256('83bef199d00f3816d682222a0fcbe55109f7d3c066b2471e186d8fe1051d1a3f'),ulex(sha256('3ffe6d9f23ab3109d7803e1121935f32d53a7768425248b943679743da1ed101'))).
% S1: Rec03-clinician is a starting-opioid-clinician.
'starting-opioid-clinician'('Rec03-clinician').
% S2: Rec03 is a category-a-recommendation.
'category-a-recommendation'('Rec03').
% S3: Rec03 is an evidence-type-4-recommendation.
'evidence-type-4-recommendation'('Rec03').
% S4: Every starting-opioid-clinician prescribes-immediate-release.
'prescribe-immediate-release'(A) :- 'starting-opioid-clinician'(A).
% S5: Does Rec03-clinician prescribe-immediate-release?
guideline_query(yesno,'prescribe-immediate-release'('Rec03-clinician')).
% S6: Is Rec03 a category-a-recommendation?
guideline_query(yesno,'category-a-recommendation'('Rec03')).
% S7: Is Rec03 an evidence-type-4-recommendation?
guideline_query(yesno,'evidence-type-4-recommendation'('Rec03')).
