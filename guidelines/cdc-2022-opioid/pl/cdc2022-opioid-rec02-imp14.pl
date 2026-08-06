% cdc2022-opioid-rec02-imp14.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp14',ace_sha256('7801802cb68bfa7cf71c8b73e4e168f2f35b8fe3e906ddab2fac7703e10025fa'),ulex(sha256('7855e6f992b6a70f49da5dbd1de6620893b143919a2f28e6b50d67c2eabb7dff'))).
% S1: Rec02-imp14-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp14-clinician').
% S2: Every subacute-chronic-pain-clinician avoids-first-line-or-routine-opioid-therapy.
'avoid-first-line-or-routine-opioid-therapy'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-imp14-clinician avoid-first-line-or-routine-opioid-therapy?
guideline_query(yesno,'avoid-first-line-or-routine-opioid-therapy'('Rec02-imp14-clinician')).
