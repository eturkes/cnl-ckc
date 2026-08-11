% cdc2022-opioid-rec04-imp05.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec04-imp05',ace_sha256('76e8e94c1f2128bfef70a91c95b10c2999f18ffdf49272833f04ab80b075eb0b'),ulex(sha256('26c94eb4c3e425db0b6c278b6861f816b856b8629608554394634e35ee8e1f3c'))).
% S1: Rec04-imp05-clinician is an opioid-dosage-clinician.
'opioid-dosage-clinician'('Rec04-imp05-clinician').
% S2: Every opioid-dosage-clinician considers-using-product-labeling-as-starting-point-and-calibrating-for-pain-severity-and-renal-or-hepatic-insufficiency-to-determine-lowest-effective-dose-for-opioid-naive-patients.
'consider-using-product-labeling-as-starting-point-and-calibrating-for-pain-severity-and-renal-or-hepatic-insufficiency-to-determine-lowest-effective-dose-for-opioid-naive-patients'(A) :- 'opioid-dosage-clinician'(A).
% S3: Does Rec04-imp05-clinician consider-using-product-labeling-as-starting-point-and-calibrating-for-pain-severity-and-renal-or-hepatic-insufficiency-to-determine-lowest-effective-dose-for-opioid-naive-patients?
guideline_query(yesno,'consider-using-product-labeling-as-starting-point-and-calibrating-for-pain-severity-and-renal-or-hepatic-insufficiency-to-determine-lowest-effective-dose-for-opioid-naive-patients'('Rec04-imp05-clinician')).
