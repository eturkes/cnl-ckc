% cdc2022-opioid-rec02-imp14.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp14',ace_sha256('7801802cb68bfa7cf71c8b73e4e168f2f35b8fe3e906ddab2fac7703e10025fa'),ulex(sha256(c879b5696dbd7394030ca8105226a5d860b57777791e2f4772f9325c07278683))).
% S1: Rec02-imp14-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp14-clinician').
% S2: Every subacute-chronic-pain-clinician avoids-first-line-or-routine-opioid-therapy.
'avoid-first-line-or-routine-opioid-therapy'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-imp14-clinician avoid-first-line-or-routine-opioid-therapy?
guideline_query(yesno,'avoid-first-line-or-routine-opioid-therapy'('Rec02-imp14-clinician')).
