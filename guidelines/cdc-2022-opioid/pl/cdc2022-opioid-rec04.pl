% cdc2022-opioid-rec04.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec04',ace_sha256('1a151ce7d02368aab8e51c8744194f5567199863872d64469891783ae242ead4'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
% S1: Rec04-clinician is an opioid-dosage-clinician.
'opioid-dosage-clinician'('Rec04-clinician').
% S2: Every opioid-dosage-clinician prescribes-lowest-effective-dosage.
'prescribe-lowest-effective-dosage'(A) :- 'opioid-dosage-clinician'(A).
% S3: Does Rec04-clinician prescribe-lowest-effective-dosage?
guideline_query(yesno,'prescribe-lowest-effective-dosage'('Rec04-clinician')).
