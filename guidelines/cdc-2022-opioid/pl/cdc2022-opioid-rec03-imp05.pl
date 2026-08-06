% cdc2022-opioid-rec03-imp05.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec03-imp05',ace_sha256('20d47d54dbd5b51fce87daf8a17e2e7762019256a0ce962b7c4e792e41b49883'),ulex(sha256('453749766e15fefa9c2dbb81ef07028b238ee2cb462770b45c2c584b7cbc92e7'))).
% S1: Rec03-imp05-clinician is a starting-opioid-clinician.
'starting-opioid-clinician'('Rec03-imp05-clinician').
% S2: Every starting-opioid-clinician uses-additional-er-la-opioid-caution-and-considers-longer-dosing-interval-for-renal-or-hepatic-dysfunction.
'use-additional-er-la-opioid-caution-and-consider-longer-dosing-interval-for-renal-or-hepatic-dysfunction'(A) :- 'starting-opioid-clinician'(A).
% S3: Does Rec03-imp05-clinician use-additional-er-la-opioid-caution-and-consider-longer-dosing-interval-for-renal-or-hepatic-dysfunction?
guideline_query(yesno,'use-additional-er-la-opioid-caution-and-consider-longer-dosing-interval-for-renal-or-hepatic-dysfunction'('Rec03-imp05-clinician')).
