% cdc2022-opioid-rec02-imp15.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp15',ace_sha256(fee1cfaf624f09c09955fca5999f9be69b1c23ab63907cf548ae9011658f89e5),ulex(sha256('7855e6f992b6a70f49da5dbd1de6620893b143919a2f28e6b50d67c2eabb7dff'))).
% S1: Rec02-imp15-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp15-clinician').
% S2: Every subacute-chronic-pain-clinician avoids-requiring-sequential-failure-or-specific-treatment-before-opioid-therapy.
'avoid-requiring-sequential-failure-or-specific-treatment-before-opioid-therapy'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-imp15-clinician avoid-requiring-sequential-failure-or-specific-treatment-before-opioid-therapy?
guideline_query(yesno,'avoid-requiring-sequential-failure-or-specific-treatment-before-opioid-therapy'('Rec02-imp15-clinician')).
