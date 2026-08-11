% cdc2022-opioid-rec02-imp22.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp22',ace_sha256(cbfbd9075bfd69359e2903eb6219ddfd2781dfe660d2f4e19d54da958e969330),ulex(sha256(c879b5696dbd7394030ca8105226a5d860b57777791e2f4772f9325c07278683))).
% S1: Rec02-imp22-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp22-clinician').
% S2: Every subacute-chronic-pain-clinician establishes-functional-treatment-goals-for-new-patients-already-receiving-opioids.
'establish-functional-treatment-goals-for-new-patients-already-receiving-opioids'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-imp22-clinician establish-functional-treatment-goals-for-new-patients-already-receiving-opioids?
guideline_query(yesno,'establish-functional-treatment-goals-for-new-patients-already-receiving-opioids'('Rec02-imp22-clinician')).
