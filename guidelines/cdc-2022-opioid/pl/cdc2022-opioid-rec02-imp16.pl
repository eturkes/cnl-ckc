% cdc2022-opioid-rec02-imp16.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp16',ace_sha256(c62ea8bd8ca74fe65152cc735e11b4ff203f110f651c9a5bbc3d496fda34fd30),ulex(sha256(c879b5696dbd7394030ca8105226a5d860b57777791e2f4772f9325c07278683))).
% S1: Rec02-imp16-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp16-clinician').
% S2: Every subacute-chronic-pain-clinician weighs-context-specific-opioid-benefits-against-risks-before-initiation.
'weigh-context-specific-opioid-benefits-against-risks-before-initiation'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-imp16-clinician weigh-context-specific-opioid-benefits-against-risks-before-initiation?
guideline_query(yesno,'weigh-context-specific-opioid-benefits-against-risks-before-initiation'('Rec02-imp16-clinician')).
