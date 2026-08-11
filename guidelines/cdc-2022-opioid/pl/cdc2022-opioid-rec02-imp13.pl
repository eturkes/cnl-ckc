% cdc2022-opioid-rec02-imp13.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp13',ace_sha256(ef20f5ac48c9c48a7717ac0daa7d716d13d3f52c7d220c0727c235fb48bef9bd),ulex(sha256(c879b5696dbd7394030ca8105226a5d860b57777791e2f4772f9325c07278683))).
% S1: Rec02-imp13-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp13-clinician').
% S2: Every subacute-chronic-pain-clinician considers-antidepressants-for-cooccurring-pain-and-depression.
'consider-antidepressants-for-cooccurring-pain-and-depression'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-imp13-clinician consider-antidepressants-for-cooccurring-pain-and-depression?
guideline_query(yesno,'consider-antidepressants-for-cooccurring-pain-and-depression'('Rec02-imp13-clinician')).
