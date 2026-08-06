% cdc2022-opioid-rec02-imp20.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp20',ace_sha256(ab489b7e9ce0d7ff20ba73ba5a607e3dcde8255144d90d1efe1d6a03573a26c4),ulex(sha256('7855e6f992b6a70f49da5dbd1de6620893b143919a2f28e6b50d67c2eabb7dff'))).
% S1: Rec02-imp20-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp20-clinician').
% S2: Every subacute-chronic-pain-clinician addresses-reversible-causes-and-prevents-unreassessed-long-term-transition-after-thirty-days-of-acute-pain-opioids.
'address-reversible-causes-and-prevent-unreassessed-long-term-transition-after-thirty-days-of-acute-pain-opioids'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-imp20-clinician address-reversible-causes-and-prevent-unreassessed-long-term-transition-after-thirty-days-of-acute-pain-opioids?
guideline_query(yesno,'address-reversible-causes-and-prevent-unreassessed-long-term-transition-after-thirty-days-of-acute-pain-opioids'('Rec02-imp20-clinician')).
