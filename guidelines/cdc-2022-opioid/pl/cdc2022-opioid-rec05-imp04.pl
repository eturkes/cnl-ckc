% cdc2022-opioid-rec05-imp04.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec05-imp04',ace_sha256('3456a6ba19c574df360b61682da784d635fc8d19523bd9155ea226239d392014'),ulex(sha256('26c94eb4c3e425db0b6c278b6861f816b856b8629608554394634e35ee8e1f3c'))).
% S1: Rec05-imp04-clinician is a dosage-change-clinician.
'dosage-change-clinician'('Rec05-imp04-clinician').
% S2: Every dosage-change-clinician prioritizes-shared-decision-making-with-patients-when-continuing-opioid-benefits-and-risks-are-close-or-unclear.
'prioritize-shared-decision-making-with-patients-when-continuing-opioid-benefits-and-risks-are-close-or-unclear'(A) :- 'dosage-change-clinician'(A).
% S3: Does Rec05-imp04-clinician prioritize-shared-decision-making-with-patients-when-continuing-opioid-benefits-and-risks-are-close-or-unclear?
guideline_query(yesno,'prioritize-shared-decision-making-with-patients-when-continuing-opioid-benefits-and-risks-are-close-or-unclear'('Rec05-imp04-clinician')).
