% cdc2022-opioid-rec02-imp21.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp21',ace_sha256('002ee498a93383bfcf3b9985bd4e6ef7ed37482c8be3795c8cb7fcde66bd0e79'),ulex(sha256(c879b5696dbd7394030ca8105226a5d860b57777791e2f4772f9325c07278683))).
% S1: Rec02-imp21-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp21-clinician').
% S2: Every subacute-chronic-pain-clinician continues-opioids-long-term-only-after-intentional-informed-benefit-risk-decision.
'continue-opioids-long-term-only-after-intentional-informed-benefit-risk-decision'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-imp21-clinician continue-opioids-long-term-only-after-intentional-informed-benefit-risk-decision?
guideline_query(yesno,'continue-opioids-long-term-only-after-intentional-informed-benefit-risk-decision'('Rec02-imp21-clinician')).
