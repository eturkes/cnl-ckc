% cdc2022-opioid-rec02-imp23.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp23',ace_sha256('6d1dfec5c9ad028bcaeb063233fb84e8a2987e72ca69702c3f0da7da92c4504d'),ulex(sha256('7855e6f992b6a70f49da5dbd1de6620893b143919a2f28e6b50d67c2eabb7dff'))).
% S1: Rec02-imp23-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp23-clinician').
% S2: Every subacute-chronic-pain-clinician avoids-rapid-tapering-and-abrupt-opioid-discontinuation.
'avoid-rapid-tapering-and-abrupt-opioid-discontinuation'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-imp23-clinician avoid-rapid-tapering-and-abrupt-opioid-discontinuation?
guideline_query(yesno,'avoid-rapid-tapering-and-abrupt-opioid-discontinuation'('Rec02-imp23-clinician')).
