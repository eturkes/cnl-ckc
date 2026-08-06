% cdc2022-opioid-rec03-imp06.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec03-imp06',ace_sha256('9e9ec358e0fbd7e537ff812b231e2a30dfb13230bd9a3dca2a70601e79c69aa9'),ulex(sha256('453749766e15fefa9c2dbb81ef07028b238ee2cb462770b45c2c584b7cbc92e7'))).
% S1: Rec03-imp06-clinician is a starting-opioid-clinician.
'starting-opioid-clinician'('Rec03-imp06-clinician').
% S2: Every starting-opioid-clinician avoids-methadone-as-first-choice-er-la-opioid.
'avoid-methadone-as-first-choice-er-la-opioid'(A) :- 'starting-opioid-clinician'(A).
% S3: Does Rec03-imp06-clinician avoid-methadone-as-first-choice-er-la-opioid?
guideline_query(yesno,'avoid-methadone-as-first-choice-er-la-opioid'('Rec03-imp06-clinician')).
