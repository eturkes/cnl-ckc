% cdc2022-opioid-rec02-imp07.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp07',ace_sha256('6ef153770462e958fa66e9a2e4dc0dd94e4aeda824a7d6b62fdbaf794915fee0'),ulex(sha256('453749766e15fefa9c2dbb81ef07028b238ee2cb462770b45c2c584b7cbc92e7'))).
% S1: Rec02-imp07-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp07-clinician').
% S2: Every subacute-chronic-pain-clinician considers-duloxetine-or-systemic-nsaids-for-multijoint-or-incompletely-controlled-osteoarthritis-pain.
'consider-duloxetine-or-systemic-nsaids-for-multijoint-or-incompletely-controlled-osteoarthritis-pain'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-imp07-clinician consider-duloxetine-or-systemic-nsaids-for-multijoint-or-incompletely-controlled-osteoarthritis-pain?
guideline_query(yesno,'consider-duloxetine-or-systemic-nsaids-for-multijoint-or-incompletely-controlled-osteoarthritis-pain'('Rec02-imp07-clinician')).
