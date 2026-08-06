% cdc2022-opioid-rec02-imp09.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp09',ace_sha256(a83e109265d3e9d663ddc1df184263120fdf4b62b7555a3edcfc87b88c26b88c),ulex(sha256('453749766e15fefa9c2dbb81ef07028b238ee2cb462770b45c2c584b7cbc92e7'))).
% S1: Rec02-imp09-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp09-clinician').
% S2: Every subacute-chronic-pain-clinician considers-nsaids-or-duloxetine-for-noncontraindicated-patients-with-chronic-low-back-pain-after-insufficient-nonpharmacologic-response.
'consider-nsaids-or-duloxetine-for-noncontraindicated-patients-with-chronic-low-back-pain-after-insufficient-nonpharmacologic-response'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-imp09-clinician consider-nsaids-or-duloxetine-for-noncontraindicated-patients-with-chronic-low-back-pain-after-insufficient-nonpharmacologic-response?
guideline_query(yesno,'consider-nsaids-or-duloxetine-for-noncontraindicated-patients-with-chronic-low-back-pain-after-insufficient-nonpharmacologic-response'('Rec02-imp09-clinician')).
