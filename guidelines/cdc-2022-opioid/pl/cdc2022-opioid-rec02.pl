% cdc2022-opioid-rec02.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02',ace_sha256('06212d79c2a865211f33f170fdbe7d5f5554701c83a116da02fb07531b0aa790'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
% S1: Rec02-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-clinician').
% S2: Every subacute-chronic-pain-clinician maximizes-nonopioid-therapy.
'maximize-nonopioid-therapy'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-clinician maximize-nonopioid-therapy?
guideline_query(yesno,'maximize-nonopioid-therapy'('Rec02-clinician')).
