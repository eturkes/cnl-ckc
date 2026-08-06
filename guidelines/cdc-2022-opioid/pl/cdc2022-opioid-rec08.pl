% cdc2022-opioid-rec08.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec08',ace_sha256('05c677fe507de0ad5fea949b688ba3f709b00772a5f07d46f22ffc1f13a400d6'),ulex(sha256('82f2a92d7aab48ce88f0c83434cf9c92e9f416cfb2d06b7685499dbaf95dc4d2'))).
% S1: Rec08-clinician is a risk-mitigation-clinician.
'risk-mitigation-clinician'('Rec08-clinician').
% S2: Rec08-peer is a risk-mitigation-clinician.
'risk-mitigation-clinician'('Rec08-peer').
% S3: Rec08 is a category-a-recommendation.
'category-a-recommendation'('Rec08').
% S4: Rec08 is an evidence-type-4-recommendation.
'evidence-type-4-recommendation'('Rec08').
% S5: Every risk-mitigation-clinician offers-naloxone.
'offer-naloxone'(A) :- 'risk-mitigation-clinician'(A).
% S6: Who offers-naloxone?
guideline_query(who(A),'offer-naloxone'(A)).
% S7: Is Rec08 a category-a-recommendation?
guideline_query(yesno,'category-a-recommendation'('Rec08')).
% S8: Is Rec08 an evidence-type-4-recommendation?
guideline_query(yesno,'evidence-type-4-recommendation'('Rec08')).
