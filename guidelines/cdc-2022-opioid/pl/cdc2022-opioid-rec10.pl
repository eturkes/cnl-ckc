% cdc2022-opioid-rec10.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec10',ace_sha256(f3a8c920f621cf3d2117d6b795fd0fc445c1335d17d44c94391800b330f28b7d),ulex(sha256('82f2a92d7aab48ce88f0c83434cf9c92e9f416cfb2d06b7685499dbaf95dc4d2'))).
% S1: Rec10-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec10-clinician').
% S2: Rec10-peer is an acute-pain-clinician.
'acute-pain-clinician'('Rec10-peer').
% S3: Rec10 is a category-b-recommendation.
'category-b-recommendation'('Rec10').
% S4: Rec10 is an evidence-type-4-recommendation.
'evidence-type-4-recommendation'('Rec10').
% S5: Every subacute-chronic-pain-clinician considers-toxicology-testing.
'consider-toxicology-testing'(A) :- 'subacute-chronic-pain-clinician'(A).
% S6: Does Rec10-clinician consider-toxicology-testing?
guideline_query(yesno,'consider-toxicology-testing'('Rec10-clinician')).
% S7: Is Rec10 a category-b-recommendation?
guideline_query(yesno,'category-b-recommendation'('Rec10')).
% S8: Is Rec10 an evidence-type-4-recommendation?
guideline_query(yesno,'evidence-type-4-recommendation'('Rec10')).
