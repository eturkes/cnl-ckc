% cdc2022-opioid-rec05-imp27.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec05-imp27',ace_sha256(f8f6465dc4bfdc3e2745fdc8808fa0c3cf6dc8cbc38852f6bfb9cfa07d05f333),ulex(sha256('453749766e15fefa9c2dbb81ef07028b238ee2cb462770b45c2c584b7cbc92e7'))).
% S1: Rec05-imp27-clinician is a dosage-change-clinician.
'dosage-change-clinician'('Rec05-imp27-clinician').
% S2: Every dosage-change-clinician considers-tapers-of-ten-percent-per-month-or-slower-for-patients-taking-opioids-for-a-year-or-longer.
'consider-tapers-of-ten-percent-per-month-or-slower-for-patients-taking-opioids-for-a-year-or-longer'(A) :- 'dosage-change-clinician'(A).
% S3: Does Rec05-imp27-clinician consider-tapers-of-ten-percent-per-month-or-slower-for-patients-taking-opioids-for-a-year-or-longer?
guideline_query(yesno,'consider-tapers-of-ten-percent-per-month-or-slower-for-patients-taking-opioids-for-a-year-or-longer'('Rec05-imp27-clinician')).
