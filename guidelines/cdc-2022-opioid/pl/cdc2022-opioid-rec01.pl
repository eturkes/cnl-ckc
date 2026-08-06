% cdc2022-opioid-rec01.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec01',ace_sha256('25f4bc0b5109717d108c138377be615898d95b4306ea1a0fc582dda009b47b35'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
% S1: Rec01-clinician is an acute-pain-clinician.
'acute-pain-clinician'('Rec01-clinician').
% S2: Every acute-pain-clinician maximizes-nonopioid-therapy.
'maximize-nonopioid-therapy'(A) :- 'acute-pain-clinician'(A).
% S3: Does Rec01-clinician maximize-nonopioid-therapy?
guideline_query(yesno,'maximize-nonopioid-therapy'('Rec01-clinician')).
