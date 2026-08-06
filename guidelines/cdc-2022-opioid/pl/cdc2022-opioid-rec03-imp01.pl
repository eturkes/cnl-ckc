% cdc2022-opioid-rec03-imp01.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec03-imp01',ace_sha256(c10658606475d9226b5454b8663de5eba74e8b1ec711e4a57b316c622be332b5),ulex(sha256('453749766e15fefa9c2dbb81ef07028b238ee2cb462770b45c2c584b7cbc92e7'))).
% S1: Rec03-imp01-clinician is a starting-opioid-clinician.
'starting-opioid-clinician'('Rec03-imp01-clinician').
% S2: Every starting-opioid-clinician avoids-er-la-opioids-for-acute-pain-and-for-subacute-or-chronic-treatment-initiation-and-intermittent-or-as-needed-use.
'avoid-er-la-opioids-for-acute-pain-and-for-subacute-or-chronic-treatment-initiation-and-intermittent-or-as-needed-use'(A) :- 'starting-opioid-clinician'(A).
% S3: Does Rec03-imp01-clinician avoid-er-la-opioids-for-acute-pain-and-for-subacute-or-chronic-treatment-initiation-and-intermittent-or-as-needed-use?
guideline_query(yesno,'avoid-er-la-opioids-for-acute-pain-and-for-subacute-or-chronic-treatment-initiation-and-intermittent-or-as-needed-use'('Rec03-imp01-clinician')).
