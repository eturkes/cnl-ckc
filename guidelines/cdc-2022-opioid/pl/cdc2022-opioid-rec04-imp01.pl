% cdc2022-opioid-rec04-imp01.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec04-imp01',ace_sha256('65a1bbf9296f64fcfc327436b9c8cc9acf4833aea79e54579a15813c6f428367'),ulex(sha256('453749766e15fefa9c2dbb81ef07028b238ee2cb462770b45c2c584b7cbc92e7'))).
% S1: Rec04-imp01-clinician is an opioid-dosage-clinician.
'opioid-dosage-clinician'('Rec04-imp01-clinician').
% S2: Every opioid-dosage-clinician uses-opioid-dosage-recommendations-as-flexible-guideposts-for-clinician-patient-decisions.
'use-opioid-dosage-recommendations-as-flexible-guideposts-for-clinician-patient-decisions'(A) :- 'opioid-dosage-clinician'(A).
% S3: Does Rec04-imp01-clinician use-opioid-dosage-recommendations-as-flexible-guideposts-for-clinician-patient-decisions?
guideline_query(yesno,'use-opioid-dosage-recommendations-as-flexible-guideposts-for-clinician-patient-decisions'('Rec04-imp01-clinician')).
