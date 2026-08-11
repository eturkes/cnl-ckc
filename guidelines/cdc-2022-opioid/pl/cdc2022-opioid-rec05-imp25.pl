% cdc2022-opioid-rec05-imp25.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec05-imp25',ace_sha256('906c6f19a47099743a42a81c1f2bc738896c06ba86111ce87dcecd84cfff8126'),ulex(sha256(c879b5696dbd7394030ca8105226a5d860b57777791e2f4772f9325c07278683))).
% S1: Rec05-imp25-policy-setter is a payer-health-system-or-state-medical-board.
'payer-health-system-or-state-medical-board'('Rec05-imp25-policy-setter').
% S2: Every payer-health-system-or-state-medical-board avoids-rigid-opioid-dose-or-duration-standards-and-incentives-ensures-threshold-policies-prevent-rapid-tapers-or-abrupt-discontinuation-and-avoids-penalizing-clinicians-for-accepting-or-not-rapidly-tapering-long-term-opioid-patients.
'avoid-rigid-opioid-dose-or-duration-standards-and-incentives-ensure-threshold-policies-prevent-rapid-tapers-or-abrupt-discontinuation-and-avoid-penalizing-clinicians-for-accepting-or-not-rapidly-tapering-long-term-opioid-patients'(A) :- 'payer-health-system-or-state-medical-board'(A).
% S3: Does Rec05-imp25-policy-setter avoid-rigid-opioid-dose-or-duration-standards-and-incentives-ensure-threshold-policies-prevent-rapid-tapers-or-abrupt-discontinuation-and-avoid-penalizing-clinicians-for-accepting-or-not-rapidly-tapering-long-term-opioid-patients?
guideline_query(yesno,'avoid-rigid-opioid-dose-or-duration-standards-and-incentives-ensure-threshold-policies-prevent-rapid-tapers-or-abrupt-discontinuation-and-avoid-penalizing-clinicians-for-accepting-or-not-rapidly-tapering-long-term-opioid-patients'('Rec05-imp25-policy-setter')).
