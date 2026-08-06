% cdc2022-opioid-rec10.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec10',ace_sha256(e4c37f7f8732adbef380acc5b0d6a1dac1c0309bcc0395b9aa2f629ff7084375),ulex(sha256('453749766e15fefa9c2dbb81ef07028b238ee2cb462770b45c2c584b7cbc92e7'))).
% S1: Rec10-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec10-clinician').
% S2: Rec10 is a category-b-recommendation.
'category-b-recommendation'('Rec10').
% S3: Rec10 is an evidence-type-4-recommendation.
'evidence-type-4-recommendation'('Rec10').
% S4: Every subacute-chronic-pain-clinician considers-toxicology-testing.
'consider-toxicology-testing'(A) :- 'subacute-chronic-pain-clinician'(A).
% S5: Does Rec10-clinician consider-toxicology-testing?
guideline_query(yesno,'consider-toxicology-testing'('Rec10-clinician')).
% S6: Is Rec10 a category-b-recommendation?
guideline_query(yesno,'category-b-recommendation'('Rec10')).
% S7: Is Rec10 an evidence-type-4-recommendation?
guideline_query(yesno,'evidence-type-4-recommendation'('Rec10')).
