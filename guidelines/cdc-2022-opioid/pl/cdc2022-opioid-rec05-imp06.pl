% cdc2022-opioid-rec05-imp06.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec05-imp06',ace_sha256(ea71897f35d170c46a7c643571a5ae9717061423c2f657caaf1fcc025aba94e9),ulex(sha256(c879b5696dbd7394030ca8105226a5d860b57777791e2f4772f9325c07278683))).
% S1: Rec05-imp06-clinician is a dosage-change-clinician.
'dosage-change-clinician'('Rec05-imp06-clinician').
% S2: Every dosage-change-clinician establishes-continued-opioid-goals-and-maximizes-nonpharmacologic-and-nonopioid-treatment-for-tapering-or-higher-dose-patients.
'establish-continued-opioid-goals-and-maximize-nonpharmacologic-and-nonopioid-treatment-for-tapering-or-higher-dose-patients'(A) :- 'dosage-change-clinician'(A).
% S3: Does Rec05-imp06-clinician establish-continued-opioid-goals-and-maximize-nonpharmacologic-and-nonopioid-treatment-for-tapering-or-higher-dose-patients?
guideline_query(yesno,'establish-continued-opioid-goals-and-maximize-nonpharmacologic-and-nonopioid-treatment-for-tapering-or-higher-dose-patients'('Rec05-imp06-clinician')).
