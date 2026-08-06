% cdc2022-opioid-rec11.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec11',ace_sha256('4bb26e026937294eb007370a27c9b54208264ab4fb3facf6555f0a181d6c0274'),ulex(sha256('82f2a92d7aab48ce88f0c83434cf9c92e9f416cfb2d06b7685499dbaf95dc4d2'))).
% S1: Rec11-clinician is a concurrent-medication-clinician.
'concurrent-medication-clinician'('Rec11-clinician').
% S2: Rec11 is a category-b-recommendation.
'category-b-recommendation'('Rec11').
% S3: Rec11 is an evidence-type-3-recommendation.
'evidence-type-3-recommendation'('Rec11').
% S4: Every concurrent-medication-clinician uses-particular-caution.
'use-particular-caution'(A) :- 'concurrent-medication-clinician'(A).
% S5: Does Rec11-clinician use-particular-caution?
guideline_query(yesno,'use-particular-caution'('Rec11-clinician')).
% S6: Is Rec11 a category-b-recommendation?
guideline_query(yesno,'category-b-recommendation'('Rec11')).
% S7: Is Rec11 an evidence-type-3-recommendation?
guideline_query(yesno,'evidence-type-3-recommendation'('Rec11')).
