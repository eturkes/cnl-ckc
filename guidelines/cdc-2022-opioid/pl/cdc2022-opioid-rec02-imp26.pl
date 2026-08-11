% cdc2022-opioid-rec02-imp26.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp26',ace_sha256(d2727450070d8a01faf37af6a7280d9cfc7235355bc144a8c3fea12886f77a66),ulex(sha256(c879b5696dbd7394030ca8105226a5d860b57777791e2f4772f9325c07278683))).
% S1: Rec02-imp26-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp26-clinician').
% S2: Every subacute-chronic-pain-clinician explains-opioid-benefits-risks-and-alternatives-and-involves-patients-in-start-decisions.
'explain-opioid-benefits-risks-and-alternatives-and-involve-patients-in-start-decisions'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-imp26-clinician explain-opioid-benefits-risks-and-alternatives-and-involve-patients-in-start-decisions?
guideline_query(yesno,'explain-opioid-benefits-risks-and-alternatives-and-involve-patients-in-start-decisions'('Rec02-imp26-clinician')).
