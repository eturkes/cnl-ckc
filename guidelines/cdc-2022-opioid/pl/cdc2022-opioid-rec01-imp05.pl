% cdc2022-opioid-rec01-imp05.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec01-imp05',ace_sha256(bdb67948cb5b43d3d77d4a9e1568a22c57253ff5e322e7c42815992c787bfc54),ulex(sha256('3ffe6d9f23ab3109d7803e1121935f32d53a7768425248b943679743da1ed101'))).
% S1: Rec01-imp05-clinician is an acute-pain-clinician.
'acute-pain-clinician'('Rec01-imp05-clinician').
% S2: Every acute-pain-clinician explains-opioid-benefits-risks-and-alternatives-and-involves-patients-in-start-decisions.
'explain-opioid-benefits-risks-and-alternatives-and-involve-patients-in-start-decisions'(A) :- 'acute-pain-clinician'(A).
% S3: Does Rec01-imp05-clinician explain-opioid-benefits-risks-and-alternatives-and-involve-patients-in-start-decisions?
guideline_query(yesno,'explain-opioid-benefits-risks-and-alternatives-and-involve-patients-in-start-decisions'('Rec01-imp05-clinician')).
