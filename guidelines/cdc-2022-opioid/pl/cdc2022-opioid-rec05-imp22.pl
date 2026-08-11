% cdc2022-opioid-rec05-imp22.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec05-imp22',ace_sha256(b79dfc0ce893d254dacd87ff51c52efec46c3d56d20e77d7e647102b23f1c129),ulex(sha256('26c94eb4c3e425db0b6c278b6861f816b856b8629608554394634e35ee8e1f3c'))).
% S1: Rec05-imp22-clinician is a dosage-change-clinician.
'dosage-change-clinician'('Rec05-imp22-clinician').
% S2: Every dosage-change-clinician monitors-patients-unable-to-taper-on-high-dose-or-high-risk-regimens-and-mitigates-overdose-risk-with-education-and-naloxone.
'monitor-patients-unable-to-taper-on-high-dose-or-high-risk-regimens-and-mitigate-overdose-risk-with-education-and-naloxone'(A) :- 'dosage-change-clinician'(A).
% S3: Does Rec05-imp22-clinician monitor-patients-unable-to-taper-on-high-dose-or-high-risk-regimens-and-mitigate-overdose-risk-with-education-and-naloxone?
guideline_query(yesno,'monitor-patients-unable-to-taper-on-high-dose-or-high-risk-regimens-and-mitigate-overdose-risk-with-education-and-naloxone'('Rec05-imp22-clinician')).
