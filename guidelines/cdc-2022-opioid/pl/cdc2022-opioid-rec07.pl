% cdc2022-opioid-rec07.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec07',ace_sha256('703b611339c19a38f1eede135c1c9f799f4cdbd81b36d676e614dd8ca42b89d2'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
% S1: Rec07-clinician is a follow-up-clinician.
'follow-up-clinician'('Rec07-clinician').
% S2: Every follow-up-clinician reevaluates-benefits-and-risks.
'reevaluate-benefits-and-risks'(A) :- 'follow-up-clinician'(A).
% S3: Does Rec07-clinician reevaluate-benefits-and-risks?
guideline_query(yesno,'reevaluate-benefits-and-risks'('Rec07-clinician')).
