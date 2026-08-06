% cdc2022-opioid-rec05-imp11.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec05-imp11',ace_sha256('0ee85c27ad45486f79d84977069f7311b43fd5f02f1353b21e73c8dfc3845564'),ulex(sha256('453749766e15fefa9c2dbb81ef07028b238ee2cb462770b45c2c584b7cbc92e7'))).
% S1: Rec05-imp11-clinician is a dosage-change-clinician.
'dosage-change-clinician'('Rec05-imp11-clinician').
% S2: Every dosage-change-clinician considers-longer-taper-after-longer-duration-of-opioid-therapy.
'consider-longer-taper-after-longer-duration-of-opioid-therapy'(A) :- 'dosage-change-clinician'(A).
% S3: Does Rec05-imp11-clinician consider-longer-taper-after-longer-duration-of-opioid-therapy?
guideline_query(yesno,'consider-longer-taper-after-longer-duration-of-opioid-therapy'('Rec05-imp11-clinician')).
