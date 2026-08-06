% cdc2022-opioid-rec02-imp25.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp25',ace_sha256('7330c193db2331e631621b9e5114bbf96aea120315d739dd5063671799ae7683'),ulex(sha256('453749766e15fefa9c2dbb81ef07028b238ee2cb462770b45c2c584b7cbc92e7'))).
% S1: Rec02-imp25-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp25-clinician').
% S2: Every subacute-chronic-pain-clinician reviews-low-cost-pain-management-options-for-all-patients.
'review-low-cost-pain-management-options-for-all-patients'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-imp25-clinician review-low-cost-pain-management-options-for-all-patients?
guideline_query(yesno,'review-low-cost-pain-management-options-for-all-patients'('Rec02-imp25-clinician')).
