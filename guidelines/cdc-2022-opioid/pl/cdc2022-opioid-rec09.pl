% cdc2022-opioid-rec09.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec09',ace_sha256('1c4b7ce93a336d223b8ae657e1b59ac4be1931adb6b62f98ce7d9900df837c0b'),ulex(sha256('82f2a92d7aab48ce88f0c83434cf9c92e9f416cfb2d06b7685499dbaf95dc4d2'))).
% S1: Rec09-clinician is a pdmp-review-clinician.
'pdmp-review-clinician'('Rec09-clinician').
% S2: Rec09 is a category-b-recommendation.
'category-b-recommendation'('Rec09').
% S3: Rec09 is an evidence-type-4-recommendation.
'evidence-type-4-recommendation'('Rec09').
% S4: Every pdmp-review-clinician reviews-pdmp-data.
'review-pdmp-data'(A) :- 'pdmp-review-clinician'(A).
% S5: Does Rec09-clinician review-pdmp-data?
guideline_query(yesno,'review-pdmp-data'('Rec09-clinician')).
% S6: Is Rec09 a category-b-recommendation?
guideline_query(yesno,'category-b-recommendation'('Rec09')).
% S7: Is Rec09 an evidence-type-4-recommendation?
guideline_query(yesno,'evidence-type-4-recommendation'('Rec09')).
