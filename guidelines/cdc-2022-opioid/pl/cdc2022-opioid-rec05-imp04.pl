% cdc2022-opioid-rec05-imp04.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec05-imp04',ace_sha256('3456a6ba19c574df360b61682da784d635fc8d19523bd9155ea226239d392014'),ulex(sha256(c879b5696dbd7394030ca8105226a5d860b57777791e2f4772f9325c07278683))).
% S1: Rec05-imp04-clinician is a dosage-change-clinician.
'dosage-change-clinician'('Rec05-imp04-clinician').
% S2: Every dosage-change-clinician prioritizes-shared-decision-making-with-patients-when-continuing-opioid-benefits-and-risks-are-close-or-unclear.
'prioritize-shared-decision-making-with-patients-when-continuing-opioid-benefits-and-risks-are-close-or-unclear'(A) :- 'dosage-change-clinician'(A).
% S3: Does Rec05-imp04-clinician prioritize-shared-decision-making-with-patients-when-continuing-opioid-benefits-and-risks-are-close-or-unclear?
guideline_query(yesno,'prioritize-shared-decision-making-with-patients-when-continuing-opioid-benefits-and-risks-are-close-or-unclear'('Rec05-imp04-clinician')).
