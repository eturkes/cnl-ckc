% cdc2022-opioid-rec12.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec12',ace_sha256('0e8bc74eac108b480eddf0a7510697523afee483f24ca8921ebdfafd9e03a6f5'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
% S1: Rec12-clinician is an opioid-use-disorder-clinician.
'opioid-use-disorder-clinician'('Rec12-clinician').
% S2: Every opioid-use-disorder-clinician offers-medication-treatment.
'offer-medication-treatment'(A) :- 'opioid-use-disorder-clinician'(A).
% S3: Does Rec12-clinician offer-medication-treatment?
guideline_query(yesno,'offer-medication-treatment'('Rec12-clinician')).
