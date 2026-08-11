% cdc2022-opioid-rec03-imp04.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec03-imp04',ace_sha256(d57407781050d1b80a1de465ad52d8736f13130ece5a121a834a7a69b4721c63),ulex(sha256(c879b5696dbd7394030ca8105226a5d860b57777791e2f4772f9325c07278683))).
% S1: Rec03-imp04-clinician is a starting-opioid-clinician.
'starting-opioid-clinician'('Rec03-imp04-clinician').
% S2: Every starting-opioid-clinician consults-product-labeling-and-reduces-total-daily-dosage-for-incomplete-cross-tolerance-when-changing-from-different-immediate-release-opioid-to-er-la-opioid.
'consult-product-labeling-and-reduce-total-daily-dosage-for-incomplete-cross-tolerance-when-changing-from-different-immediate-release-opioid-to-er-la-opioid'(A) :- 'starting-opioid-clinician'(A).
% S3: Does Rec03-imp04-clinician consult-product-labeling-and-reduce-total-daily-dosage-for-incomplete-cross-tolerance-when-changing-from-different-immediate-release-opioid-to-er-la-opioid?
guideline_query(yesno,'consult-product-labeling-and-reduce-total-daily-dosage-for-incomplete-cross-tolerance-when-changing-from-different-immediate-release-opioid-to-er-la-opioid'('Rec03-imp04-clinician')).
