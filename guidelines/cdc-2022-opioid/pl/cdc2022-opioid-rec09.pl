% cdc2022-opioid-rec09.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec09',ace_sha256('5d617e7fa8ce7cff0081b4814fc261212810447908e77ebb6089c8e7a37c07a0'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
% S1: Rec09-clinician is a pdmp-review-clinician.
'pdmp-review-clinician'('Rec09-clinician').
% S2: Every pdmp-review-clinician reviews-pdmp-data.
'review-pdmp-data'(A) :- 'pdmp-review-clinician'(A).
% S3: Does Rec09-clinician review-pdmp-data?
guideline_query(yesno,'review-pdmp-data'('Rec09-clinician')).
