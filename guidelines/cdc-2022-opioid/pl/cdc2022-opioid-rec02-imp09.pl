% cdc2022-opioid-rec02-imp09.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp09',ace_sha256('644438d27da156b293c3982ca4cdea927dc35430a11a3e6e876284842bb44758'),ulex(sha256('7855e6f992b6a70f49da5dbd1de6620893b143919a2f28e6b50d67c2eabb7dff'))).
:- dynamic('have-nsaid-or-duloxetine-contraindication'/1).
% S1: Rec02-imp09-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp09-clinician').
% S2: Every subacute-chronic-pain-clinician that does not provably have-nsaid-or-duloxetine-contraindication considers-nsaids-or-duloxetine-for-chronic-low-back-pain-after-insufficient-nonpharmacologic-response.
'consider-nsaids-or-duloxetine-for-chronic-low-back-pain-after-insufficient-nonpharmacologic-response'(A) :- 'subacute-chronic-pain-clinician'(A), \+ 'have-nsaid-or-duloxetine-contraindication'(A).
% S3: Does Rec02-imp09-clinician consider-nsaids-or-duloxetine-for-chronic-low-back-pain-after-insufficient-nonpharmacologic-response?
guideline_query(yesno,'consider-nsaids-or-duloxetine-for-chronic-low-back-pain-after-insufficient-nonpharmacologic-response'('Rec02-imp09-clinician')).
