% cdc2022-opioid-rec02-imp04.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp04',ace_sha256(d84b01436df5ecae0b25f57ff9c21af061e3ebe0da74778f22b431dca901c2f4),ulex(sha256('453749766e15fefa9c2dbb81ef07028b238ee2cb462770b45c2c584b7cbc92e7'))).
% S1: Rec02-imp04-system is a health-insurer-or-health-system.
'health-insurer-or-health-system'('Rec02-imp04-system').
% S2: Every health-insurer-or-health-system increases-reimbursement-and-access-to-effective-noninvasive-therapies.
'increase-reimbursement-and-access-to-effective-noninvasive-therapies'(A) :- 'health-insurer-or-health-system'(A).
% S3: Does Rec02-imp04-system increase-reimbursement-and-access-to-effective-noninvasive-therapies?
guideline_query(yesno,'increase-reimbursement-and-access-to-effective-noninvasive-therapies'('Rec02-imp04-system')).
