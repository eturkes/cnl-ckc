% cdc2022-opioid-rec03-imp07.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec03-imp07',ace_sha256(ae0721f71438b3a5d7506d4052786cf8031010e115d6fb4ec3d7b37122ab4164),ulex(sha256(c879b5696dbd7394030ca8105226a5d860b57777791e2f4772f9325c07278683))).
% S1: Rec03-imp07-clinician is a starting-opioid-clinician.
'starting-opioid-clinician'('Rec03-imp07-clinician').
% S2: Every starting-opioid-clinician considers-methadone-for-pain-only-with-risk-profile-familiarity-and-preparation-for-patient-education-close-monitoring-qt-risk-assessment-and-electrocardiographic-monitoring.
'consider-methadone-for-pain-only-with-risk-profile-familiarity-and-preparation-for-patient-education-close-monitoring-qt-risk-assessment-and-electrocardiographic-monitoring'(A) :- 'starting-opioid-clinician'(A).
% S3: Does Rec03-imp07-clinician consider-methadone-for-pain-only-with-risk-profile-familiarity-and-preparation-for-patient-education-close-monitoring-qt-risk-assessment-and-electrocardiographic-monitoring?
guideline_query(yesno,'consider-methadone-for-pain-only-with-risk-profile-familiarity-and-preparation-for-patient-education-close-monitoring-qt-risk-assessment-and-electrocardiographic-monitoring'('Rec03-imp07-clinician')).
