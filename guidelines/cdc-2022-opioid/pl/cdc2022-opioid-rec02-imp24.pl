% cdc2022-opioid-rec02-imp24.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp24',ace_sha256('695df014b158f7afdaeb497a975a3104e58a479a6e90df1e4e41b465891a0f86'),ulex(sha256('3ffe6d9f23ab3109d7803e1121935f32d53a7768425248b943679743da1ed101'))).
% S1: Rec02-imp24-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp24-clinician').
% S2: Every subacute-chronic-pain-clinician educates-patients-before-opioid-therapy-to-inform-preference-sensitive-decisions.
'educate-patients-before-opioid-therapy-to-inform-preference-sensitive-decisions'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-imp24-clinician educate-patients-before-opioid-therapy-to-inform-preference-sensitive-decisions?
guideline_query(yesno,'educate-patients-before-opioid-therapy-to-inform-preference-sensitive-decisions'('Rec02-imp24-clinician')).
