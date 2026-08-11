% cdc2022-opioid-rec05-imp08.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec05-imp08',ace_sha256('53ab702bd1373f43a92791f3b74f07d3d2012900a3c26ff77fc38e70ebb67c35'),ulex(sha256('26c94eb4c3e425db0b6c278b6861f816b856b8629608554394634e35ee8e1f3c'))).
% S1: Rec05-imp08-clinician is a dosage-change-clinician.
'dosage-change-clinician'('Rec05-imp08-clinician').
% S2: Every dosage-change-clinician follows-up-at-least-monthly-with-patients-engaging-in-opioid-tapering.
'follow-up-at-least-monthly-with-patients-engaging-in-opioid-tapering'(A) :- 'dosage-change-clinician'(A).
% S3: Does Rec05-imp08-clinician follow-up-at-least-monthly-with-patients-engaging-in-opioid-tapering?
guideline_query(yesno,'follow-up-at-least-monthly-with-patients-engaging-in-opioid-tapering'('Rec05-imp08-clinician')).
