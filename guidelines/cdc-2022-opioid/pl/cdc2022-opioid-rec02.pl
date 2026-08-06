% cdc2022-opioid-rec02.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02',ace_sha256('728c80f5ad1de89dfe2d81cfe057a22929802aab3f172559783c0ef69f1fd06f'),ulex(sha256('82f2a92d7aab48ce88f0c83434cf9c92e9f416cfb2d06b7685499dbaf95dc4d2'))).
% S1: Rec02-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-clinician').
% S2: Rec02 is a category-a-recommendation.
'category-a-recommendation'('Rec02').
% S3: Rec02 is an evidence-type-2-recommendation.
'evidence-type-2-recommendation'('Rec02').
% S4: Every subacute-chronic-pain-clinician maximizes-nonopioid-therapy.
'maximize-nonopioid-therapy'(A) :- 'subacute-chronic-pain-clinician'(A).
% S5: Does Rec02-clinician maximize-nonopioid-therapy?
guideline_query(yesno,'maximize-nonopioid-therapy'('Rec02-clinician')).
% S6: Is Rec02 a category-a-recommendation?
guideline_query(yesno,'category-a-recommendation'('Rec02')).
% S7: Is Rec02 an evidence-type-2-recommendation?
guideline_query(yesno,'evidence-type-2-recommendation'('Rec02')).
