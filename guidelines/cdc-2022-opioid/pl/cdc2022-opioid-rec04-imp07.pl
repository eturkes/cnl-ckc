% cdc2022-opioid-rec04-imp07.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec04-imp07',ace_sha256(d048755ce1742c6389581d3f18accfdd4220412399d5ee58cd1ce63efe4fa697),ulex(sha256('453749766e15fefa9c2dbb81ef07028b238ee2cb462770b45c2c584b7cbc92e7'))).
% S1: Rec04-imp07-clinician is an opioid-dosage-clinician.
'opioid-dosage-clinician'('Rec04-imp07-clinician').
% S2: Every opioid-dosage-clinician pauses-and-reassesses-benefits-and-risks-before-increasing-total-opioid-dosage-to-at-least-fifty-mme-per-day.
'pause-and-reassess-benefits-and-risks-before-increasing-total-opioid-dosage-to-at-least-fifty-mme-per-day'(A) :- 'opioid-dosage-clinician'(A).
% S3: Does Rec04-imp07-clinician pause-and-reassess-benefits-and-risks-before-increasing-total-opioid-dosage-to-at-least-fifty-mme-per-day?
guideline_query(yesno,'pause-and-reassess-benefits-and-risks-before-increasing-total-opioid-dosage-to-at-least-fifty-mme-per-day'('Rec04-imp07-clinician')).
