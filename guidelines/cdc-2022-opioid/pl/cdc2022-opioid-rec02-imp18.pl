% cdc2022-opioid-rec02-imp18.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp18',ace_sha256('5b2654edc1feafc2359376992ff7268c2794d186bd3c05907367fc0ae1e9115f'),ulex(sha256('453749766e15fefa9c2dbb81ef07028b238ee2cb462770b45c2c584b7cbc92e7'))).
% S1: Rec02-imp18-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp18-clinician').
% S2: Every subacute-chronic-pain-clinician considers-clinician-patient-exit-strategy-before-opioid-initiation.
'consider-clinician-patient-exit-strategy-before-opioid-initiation'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-imp18-clinician consider-clinician-patient-exit-strategy-before-opioid-initiation?
guideline_query(yesno,'consider-clinician-patient-exit-strategy-before-opioid-initiation'('Rec02-imp18-clinician')).
