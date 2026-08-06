% cdc2022-opioid-rec02-imp01.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp01',ace_sha256('23db90b08fef40f8b7345b795b6e4a3d34e65e7b9d3ad03e556f2e7b063a942c'),ulex(sha256('453749766e15fefa9c2dbb81ef07028b238ee2cb462770b45c2c584b7cbc92e7'))).
% S1: Rec02-imp01-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp01-clinician').
% S2: Every subacute-chronic-pain-clinician evaluates-patients-and-confirms-diagnosis.
'evaluate-patients-and-confirm-diagnosis'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-imp01-clinician evaluate-patients-and-confirm-diagnosis?
guideline_query(yesno,'evaluate-patients-and-confirm-diagnosis'('Rec02-imp01-clinician')).
