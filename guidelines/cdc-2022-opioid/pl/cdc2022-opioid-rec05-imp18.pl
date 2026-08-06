% cdc2022-opioid-rec05-imp18.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec05-imp18',ace_sha256('4b33f1c0396af0bf309d84ae2c01bd3aab7169f8ff839ee27c6efcad7a067275'),ulex(sha256('453749766e15fefa9c2dbb81ef07028b238ee2cb462770b45c2c584b7cbc92e7'))).
% S1: Rec05-imp18-clinician is a dosage-change-clinician.
'dosage-change-clinician'('Rec05-imp18-clinician').
% S2: Every dosage-change-clinician considers-extending-dose-intervals-after-smallest-dose-and-stopping-opioids-below-once-daily-when-patient-agreed-taper-goal-is-discontinuation.
'consider-extending-dose-intervals-after-smallest-dose-and-stopping-opioids-below-once-daily-when-patient-agreed-taper-goal-is-discontinuation'(A) :- 'dosage-change-clinician'(A).
% S3: Does Rec05-imp18-clinician consider-extending-dose-intervals-after-smallest-dose-and-stopping-opioids-below-once-daily-when-patient-agreed-taper-goal-is-discontinuation?
guideline_query(yesno,'consider-extending-dose-intervals-after-smallest-dose-and-stopping-opioids-below-once-daily-when-patient-agreed-taper-goal-is-discontinuation'('Rec05-imp18-clinician')).
