% cdc2022-opioid-rec02-imp19.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp19',ace_sha256('1da9d822950b9f7d224f9b64638810de41749e0516b9da2f84ce7096d739c9fd'),ulex(sha256(c879b5696dbd7394030ca8105226a5d860b57777791e2f4772f9325c07278683))).
% S1: Rec02-imp19-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp19-clinician').
% S2: Every subacute-chronic-pain-clinician jointly-establishes-functional-evaluation-and-measurable-goals-before-opioids.
'jointly-establish-functional-evaluation-and-measurable-goals-before-opioids'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-imp19-clinician jointly-establish-functional-evaluation-and-measurable-goals-before-opioids?
guideline_query(yesno,'jointly-establish-functional-evaluation-and-measurable-goals-before-opioids'('Rec02-imp19-clinician')).
