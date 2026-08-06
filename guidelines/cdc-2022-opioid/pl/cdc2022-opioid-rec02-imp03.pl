% cdc2022-opioid-rec02-imp03.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp03',ace_sha256('5d7c9336b91d7042f5621176baa41c32242abbc2c9432cb3001e4e905813d25f'),ulex(sha256('453749766e15fefa9c2dbb81ef07028b238ee2cb462770b45c2c584b7cbc92e7'))).
% S1: Rec02-imp03-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp03-clinician').
% S2: Every subacute-chronic-pain-clinician considers-physical-therapy-for-exercise-access-or-response-barriers.
'consider-physical-therapy-for-exercise-access-or-response-barriers'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-imp03-clinician consider-physical-therapy-for-exercise-access-or-response-barriers?
guideline_query(yesno,'consider-physical-therapy-for-exercise-access-or-response-barriers'('Rec02-imp03-clinician')).
