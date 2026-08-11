% cdc2022-opioid-rec03-imp01.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec03-imp01',ace_sha256(c10658606475d9226b5454b8663de5eba74e8b1ec711e4a57b316c622be332b5),ulex(sha256(c879b5696dbd7394030ca8105226a5d860b57777791e2f4772f9325c07278683))).
% S1: Rec03-imp01-clinician is a starting-opioid-clinician.
'starting-opioid-clinician'('Rec03-imp01-clinician').
% S2: Every starting-opioid-clinician avoids-er-la-opioids-for-acute-pain-and-for-subacute-or-chronic-treatment-initiation-and-intermittent-or-as-needed-use.
'avoid-er-la-opioids-for-acute-pain-and-for-subacute-or-chronic-treatment-initiation-and-intermittent-or-as-needed-use'(A) :- 'starting-opioid-clinician'(A).
% S3: Does Rec03-imp01-clinician avoid-er-la-opioids-for-acute-pain-and-for-subacute-or-chronic-treatment-initiation-and-intermittent-or-as-needed-use?
guideline_query(yesno,'avoid-er-la-opioids-for-acute-pain-and-for-subacute-or-chronic-treatment-initiation-and-intermittent-or-as-needed-use'('Rec03-imp01-clinician')).
