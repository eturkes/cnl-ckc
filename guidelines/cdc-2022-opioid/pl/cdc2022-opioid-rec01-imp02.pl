% cdc2022-opioid-rec01-imp02.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec01-imp02',ace_sha256('95267cb9dc3554f138cb8a930cc0522b2ba9fa9de840ca95836a668166db9a15'),ulex(sha256('453749766e15fefa9c2dbb81ef07028b238ee2cb462770b45c2c584b7cbc92e7'))).
% S1: Rec01-imp02-clinician is an acute-pain-clinician.
'acute-pain-clinician'('Rec01-imp02-clinician').
% S2: Every acute-pain-clinician prescribes-immediate-release-opioids-at-lowest-effective-dose-for-expected-severe-pain-duration.
'prescribe-immediate-release-opioids-at-lowest-effective-dose-for-expected-severe-pain-duration'(A) :- 'acute-pain-clinician'(A).
% S3: Does Rec01-imp02-clinician prescribe-immediate-release-opioids-at-lowest-effective-dose-for-expected-severe-pain-duration?
guideline_query(yesno,'prescribe-immediate-release-opioids-at-lowest-effective-dose-for-expected-severe-pain-duration'('Rec01-imp02-clinician')).
