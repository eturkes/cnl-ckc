% cdc2022-opioid-rec05-imp07.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec05-imp07',ace_sha256('001efb9138a941c7b2ebf99fd81cd9c804ee158094ce7ea57fa255c6a65f9e3a'),ulex(sha256(c879b5696dbd7394030ca8105226a5d860b57777791e2f4772f9325c07278683))).
% S1: Rec05-imp07-clinician is a dosage-change-clinician.
'dosage-change-clinician'('Rec05-imp07-clinician').
% S2: Every dosage-change-clinician collaborates-with-patients-on-taper-plans-including-speed-and-pause-decisions.
'collaborate-with-patients-on-taper-plans-including-speed-and-pause-decisions'(A) :- 'dosage-change-clinician'(A).
% S3: Does Rec05-imp07-clinician collaborate-with-patients-on-taper-plans-including-speed-and-pause-decisions?
guideline_query(yesno,'collaborate-with-patients-on-taper-plans-including-speed-and-pause-decisions'('Rec05-imp07-clinician')).
