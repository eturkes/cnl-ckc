% cdc2022-opioid-rec03-imp07.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec03-imp07',ace_sha256(ae0721f71438b3a5d7506d4052786cf8031010e115d6fb4ec3d7b37122ab4164),ulex(sha256('453749766e15fefa9c2dbb81ef07028b238ee2cb462770b45c2c584b7cbc92e7'))).
% S1: Rec03-imp07-clinician is a starting-opioid-clinician.
'starting-opioid-clinician'('Rec03-imp07-clinician').
% S2: Every starting-opioid-clinician considers-methadone-for-pain-only-with-risk-profile-familiarity-and-preparation-for-patient-education-close-monitoring-qt-risk-assessment-and-electrocardiographic-monitoring.
'consider-methadone-for-pain-only-with-risk-profile-familiarity-and-preparation-for-patient-education-close-monitoring-qt-risk-assessment-and-electrocardiographic-monitoring'(A) :- 'starting-opioid-clinician'(A).
% S3: Does Rec03-imp07-clinician consider-methadone-for-pain-only-with-risk-profile-familiarity-and-preparation-for-patient-education-close-monitoring-qt-risk-assessment-and-electrocardiographic-monitoring?
guideline_query(yesno,'consider-methadone-for-pain-only-with-risk-profile-familiarity-and-preparation-for-patient-education-close-monitoring-qt-risk-assessment-and-electrocardiographic-monitoring'('Rec03-imp07-clinician')).
