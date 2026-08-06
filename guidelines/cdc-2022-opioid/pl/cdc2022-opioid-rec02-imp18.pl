% cdc2022-opioid-rec02-imp18.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp18',ace_sha256(f1f02368c1142b45d61c2c61fcc218a7792bf14d150bee3cec5252ce7b255324),ulex(sha256('7855e6f992b6a70f49da5dbd1de6620893b143919a2f28e6b50d67c2eabb7dff'))).
% S1: Rec02-imp18-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp18-clinician').
% S2: Every subacute-chronic-pain-clinician establishes-clinician-patient-exit-strategy-before-opioid-initiation.
'establish-clinician-patient-exit-strategy-before-opioid-initiation'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-imp18-clinician establish-clinician-patient-exit-strategy-before-opioid-initiation?
guideline_query(yesno,'establish-clinician-patient-exit-strategy-before-opioid-initiation'('Rec02-imp18-clinician')).
