% cdc2022-opioid-rec05-imp11.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec05-imp11',ace_sha256('0ee85c27ad45486f79d84977069f7311b43fd5f02f1353b21e73c8dfc3845564'),ulex(sha256(c879b5696dbd7394030ca8105226a5d860b57777791e2f4772f9325c07278683))).
% S1: Rec05-imp11-clinician is a dosage-change-clinician.
'dosage-change-clinician'('Rec05-imp11-clinician').
% S2: Every dosage-change-clinician considers-longer-taper-after-longer-duration-of-opioid-therapy.
'consider-longer-taper-after-longer-duration-of-opioid-therapy'(A) :- 'dosage-change-clinician'(A).
% S3: Does Rec05-imp11-clinician consider-longer-taper-after-longer-duration-of-opioid-therapy?
guideline_query(yesno,'consider-longer-taper-after-longer-duration-of-opioid-therapy'('Rec05-imp11-clinician')).
