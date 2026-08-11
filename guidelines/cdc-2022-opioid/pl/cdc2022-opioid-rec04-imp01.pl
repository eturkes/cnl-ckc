% cdc2022-opioid-rec04-imp01.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec04-imp01',ace_sha256('65a1bbf9296f64fcfc327436b9c8cc9acf4833aea79e54579a15813c6f428367'),ulex(sha256('26c94eb4c3e425db0b6c278b6861f816b856b8629608554394634e35ee8e1f3c'))).
% S1: Rec04-imp01-clinician is an opioid-dosage-clinician.
'opioid-dosage-clinician'('Rec04-imp01-clinician').
% S2: Every opioid-dosage-clinician uses-opioid-dosage-recommendations-as-flexible-guideposts-for-clinician-patient-decisions.
'use-opioid-dosage-recommendations-as-flexible-guideposts-for-clinician-patient-decisions'(A) :- 'opioid-dosage-clinician'(A).
% S3: Does Rec04-imp01-clinician use-opioid-dosage-recommendations-as-flexible-guideposts-for-clinician-patient-decisions?
guideline_query(yesno,'use-opioid-dosage-recommendations-as-flexible-guideposts-for-clinician-patient-decisions'('Rec04-imp01-clinician')).
