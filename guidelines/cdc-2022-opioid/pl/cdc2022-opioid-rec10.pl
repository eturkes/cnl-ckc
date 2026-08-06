% cdc2022-opioid-rec10.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec10',ace_sha256('03afe0af9d6c1b88962fe52b9e9ae91c6bc7bb36270baa1a7cc4bf6ec896db16'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
% S1: Rec10-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec10-clinician').
% S2: Rec10-peer is an acute-pain-clinician.
'acute-pain-clinician'('Rec10-peer').
% S3: Every subacute-chronic-pain-clinician considers-toxicology-testing.
'consider-toxicology-testing'(A) :- 'subacute-chronic-pain-clinician'(A).
% S4: Does Rec10-clinician consider-toxicology-testing?
guideline_query(yesno,'consider-toxicology-testing'('Rec10-clinician')).
