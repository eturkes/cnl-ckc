% cdc2022-opioid-rec03.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec03',ace_sha256('65bafa8f9ffab76dd476a936f9b124e6baf210182ed1e23b4314ea3268db2e7f'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
% S1: Rec03-clinician is a starting-opioid-clinician.
'starting-opioid-clinician'('Rec03-clinician').
% S2: Every starting-opioid-clinician prescribes-immediate-release.
'prescribe-immediate-release'(A) :- 'starting-opioid-clinician'(A).
% S3: Does Rec03-clinician prescribe-immediate-release?
guideline_query(yesno,'prescribe-immediate-release'('Rec03-clinician')).
