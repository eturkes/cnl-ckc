% cdc2022-opioid-rec02-imp11.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp11',ace_sha256('5958bac89d3fc6639d603f0cf095509293b5c143ae0dd403bcbdb16fc65e65d3'),ulex(sha256('7855e6f992b6a70f49da5dbd1de6620893b143919a2f28e6b50d67c2eabb7dff'))).
% S1: Rec02-imp11-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp11-clinician').
% S2: Every subacute-chronic-pain-clinician uses-tricyclic-antidepressants-judiciously-case-by-case-in-older-adults.
'use-tricyclic-antidepressants-judiciously-case-by-case-in-older-adults'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-imp11-clinician use-tricyclic-antidepressants-judiciously-case-by-case-in-older-adults?
guideline_query(yesno,'use-tricyclic-antidepressants-judiciously-case-by-case-in-older-adults'('Rec02-imp11-clinician')).
