% cdc2022-opioid-rec03-imp05.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec03-imp05',ace_sha256('20d47d54dbd5b51fce87daf8a17e2e7762019256a0ce962b7c4e792e41b49883'),ulex(sha256(c879b5696dbd7394030ca8105226a5d860b57777791e2f4772f9325c07278683))).
% S1: Rec03-imp05-clinician is a starting-opioid-clinician.
'starting-opioid-clinician'('Rec03-imp05-clinician').
% S2: Every starting-opioid-clinician uses-additional-er-la-opioid-caution-and-considers-longer-dosing-interval-for-renal-or-hepatic-dysfunction.
'use-additional-er-la-opioid-caution-and-consider-longer-dosing-interval-for-renal-or-hepatic-dysfunction'(A) :- 'starting-opioid-clinician'(A).
% S3: Does Rec03-imp05-clinician use-additional-er-la-opioid-caution-and-consider-longer-dosing-interval-for-renal-or-hepatic-dysfunction?
guideline_query(yesno,'use-additional-er-la-opioid-caution-and-consider-longer-dosing-interval-for-renal-or-hepatic-dysfunction'('Rec03-imp05-clinician')).
