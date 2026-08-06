% cdc2022-opioid-rec02-imp05.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp05',ace_sha256('2158640754bd11245daec6df034843fb8b49a23192f532240c88b1e9cb0a9d6d'),ulex(sha256('3ffe6d9f23ab3109d7803e1121935f32d53a7768425248b943679743da1ed101'))).
% S1: Rec02-imp05-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp05-clinician').
% S2: Every subacute-chronic-pain-clinician reviews-drug-labeling-and-weighs-benefits-and-risks-before-pharmacologic-therapy.
'review-drug-labeling-and-weigh-benefits-and-risks-before-pharmacologic-therapy'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-imp05-clinician review-drug-labeling-and-weigh-benefits-and-risks-before-pharmacologic-therapy?
guideline_query(yesno,'review-drug-labeling-and-weigh-benefits-and-risks-before-pharmacologic-therapy'('Rec02-imp05-clinician')).
