% cdc2022-opioid-rec04-imp08.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec04-imp08',ace_sha256('404f275fb1f5a734c29f6b9c5984b55ce365450ebc5e1594b3a64907b0b0183c'),ulex(sha256(c879b5696dbd7394030ca8105226a5d860b57777791e2f4772f9325c07278683))).
% S1: Rec04-imp08-clinician is an opioid-dosage-clinician.
'opioid-dosage-clinician'('Rec04-imp08-clinician').
% S2: Every opioid-dosage-clinician uses-caution-and-smallest-practical-increase-after-deciding-to-increase-dosage.
'use-caution-and-smallest-practical-increase-after-deciding-to-increase-dosage'(A) :- 'opioid-dosage-clinician'(A).
% S3: Does Rec04-imp08-clinician use-caution-and-smallest-practical-increase-after-deciding-to-increase-dosage?
guideline_query(yesno,'use-caution-and-smallest-practical-increase-after-deciding-to-increase-dosage'('Rec04-imp08-clinician')).
