% cdc2022-opioid-rec05-imp10.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec05-imp10',ace_sha256('7f7d0cd640f0fe2dc5c6a7f1b848d9cc94b6d081fef09e5988501cb3618f494c'),ulex(sha256('453749766e15fefa9c2dbb81ef07028b238ee2cb462770b45c2c584b7cbc92e7'))).
% S1: Rec05-imp10-clinician is a dosage-change-clinician.
'dosage-change-clinician'('Rec05-imp10-clinician').
% S2: Every dosage-change-clinician uses-taper-slow-enough-to-minimize-withdrawal-when-reducing-or-discontinuing-opioids.
'use-taper-slow-enough-to-minimize-withdrawal-when-reducing-or-discontinuing-opioids'(A) :- 'dosage-change-clinician'(A).
% S3: Does Rec05-imp10-clinician use-taper-slow-enough-to-minimize-withdrawal-when-reducing-or-discontinuing-opioids?
guideline_query(yesno,'use-taper-slow-enough-to-minimize-withdrawal-when-reducing-or-discontinuing-opioids'('Rec05-imp10-clinician')).
