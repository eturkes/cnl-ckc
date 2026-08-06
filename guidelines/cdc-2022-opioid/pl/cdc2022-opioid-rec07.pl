% cdc2022-opioid-rec07.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec07',ace_sha256('2be5e96b6b52886f8684cb7e75b73abbfcf84f94b7a4546807dde1f51cd9d1a3'),ulex(sha256('82f2a92d7aab48ce88f0c83434cf9c92e9f416cfb2d06b7685499dbaf95dc4d2'))).
% S1: Rec07-clinician is a follow-up-clinician.
'follow-up-clinician'('Rec07-clinician').
% S2: Rec07 is a category-a-recommendation.
'category-a-recommendation'('Rec07').
% S3: Rec07 is an evidence-type-4-recommendation.
'evidence-type-4-recommendation'('Rec07').
% S4: Every follow-up-clinician reevaluates-benefits-and-risks.
'reevaluate-benefits-and-risks'(A) :- 'follow-up-clinician'(A).
% S5: Does Rec07-clinician reevaluate-benefits-and-risks?
guideline_query(yesno,'reevaluate-benefits-and-risks'('Rec07-clinician')).
% S6: Is Rec07 a category-a-recommendation?
guideline_query(yesno,'category-a-recommendation'('Rec07')).
% S7: Is Rec07 an evidence-type-4-recommendation?
guideline_query(yesno,'evidence-type-4-recommendation'('Rec07')).
