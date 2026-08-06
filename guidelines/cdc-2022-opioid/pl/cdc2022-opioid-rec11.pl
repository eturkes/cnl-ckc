% cdc2022-opioid-rec11.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec11',ace_sha256('82f481566c93cebc9a5612e09a417531cd0283da485fcfcd82043a9573ddb32c'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
% S1: Rec11-clinician is a concurrent-medication-clinician.
'concurrent-medication-clinician'('Rec11-clinician').
% S2: Every concurrent-medication-clinician uses-particular-caution.
'use-particular-caution'(A) :- 'concurrent-medication-clinician'(A).
% S3: Does Rec11-clinician use-particular-caution?
guideline_query(yesno,'use-particular-caution'('Rec11-clinician')).
