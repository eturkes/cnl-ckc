% cdc2022-opioid-rec02-imp02.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp02',ace_sha256(fcb6939708cb5ef648d9dfe1ba07f881aabb84f5cdcc86095847d5fb11d2898e),ulex(sha256(c879b5696dbd7394030ca8105226a5d860b57777791e2f4772f9325c07278683))).
% S1: Rec02-imp02-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp02-clinician').
% S2: Every subacute-chronic-pain-clinician recommends-appropriate-noninvasive-nonpharmacologic-approaches.
'recommend-appropriate-noninvasive-nonpharmacologic-approaches'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-imp02-clinician recommend-appropriate-noninvasive-nonpharmacologic-approaches?
guideline_query(yesno,'recommend-appropriate-noninvasive-nonpharmacologic-approaches'('Rec02-imp02-clinician')).
