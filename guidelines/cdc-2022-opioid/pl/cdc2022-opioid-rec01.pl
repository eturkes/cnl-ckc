% cdc2022-opioid-rec01.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec01',ace_sha256(fe7487a540ca128b091bfdbaf9a42a1feffdb145447f7933e2ebf0e60e770b0f),ulex(sha256('3006a2a125138b9e3f7c52e9f411829b69d4de097d4f147de53aea6239a294b9'))).
% S1: Rec01-clinician is an acute-pain-clinician.
'acute-pain-clinician'('Rec01-clinician').
% S2: Rec01 is a category-b-recommendation.
'category-b-recommendation'('Rec01').
% S3: Rec01 is an evidence-type-3-recommendation.
'evidence-type-3-recommendation'('Rec01').
% S4: Every acute-pain-clinician maximizes-nonopioid-therapy.
'maximize-nonopioid-therapy'(A) :- 'acute-pain-clinician'(A).
% S5: Does Rec01-clinician maximize-nonopioid-therapy?
guideline_query(yesno,'maximize-nonopioid-therapy'('Rec01-clinician')).
% S6: Is Rec01 a category-b-recommendation?
guideline_query(yesno,'category-b-recommendation'('Rec01')).
% S7: Is Rec01 an evidence-type-3-recommendation?
guideline_query(yesno,'evidence-type-3-recommendation'('Rec01')).
