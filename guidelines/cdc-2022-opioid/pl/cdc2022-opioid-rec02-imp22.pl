% cdc2022-opioid-rec02-imp22.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp22',ace_sha256(cbfbd9075bfd69359e2903eb6219ddfd2781dfe660d2f4e19d54da958e969330),ulex(sha256('453749766e15fefa9c2dbb81ef07028b238ee2cb462770b45c2c584b7cbc92e7'))).
% S1: Rec02-imp22-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp22-clinician').
% S2: Every subacute-chronic-pain-clinician establishes-functional-treatment-goals-for-new-patients-already-receiving-opioids.
'establish-functional-treatment-goals-for-new-patients-already-receiving-opioids'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-imp22-clinician establish-functional-treatment-goals-for-new-patients-already-receiving-opioids?
guideline_query(yesno,'establish-functional-treatment-goals-for-new-patients-already-receiving-opioids'('Rec02-imp22-clinician')).
