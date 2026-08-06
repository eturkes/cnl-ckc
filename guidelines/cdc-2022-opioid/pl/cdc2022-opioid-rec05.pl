% cdc2022-opioid-rec05.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec05',ace_sha256('65b66ef8b6cf5af09e9c0054f325c7cddda88ffaf563b2c5e6d1e1e659ec72c8'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
:- dynamic('detect-life-threatening-warning'/1).
% S1: Rec05-clinician is a dosage-change-clinician.
'dosage-change-clinician'('Rec05-clinician').
% S2: Every dosage-change-clinician that does not provably detect-life-threatening-warning avoids-abrupt-discontinuation.
'avoid-abrupt-discontinuation'(A) :- 'dosage-change-clinician'(A), \+ 'detect-life-threatening-warning'(A).
% S3: Does Rec05-clinician avoid-abrupt-discontinuation?
guideline_query(yesno,'avoid-abrupt-discontinuation'('Rec05-clinician')).
