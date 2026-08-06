% cdc2022-opioid-rec02-imp17.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp17',ace_sha256('15ccabc3d758b1ed9e0af6fc485ab1e34a009a8c18b3ddf29b889664fa3084f0'),ulex(sha256('453749766e15fefa9c2dbb81ef07028b238ee2cb462770b45c2c584b7cbc92e7'))).
% S1: Rec02-imp17-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp17-clinician').
% S2: Every subacute-chronic-pain-clinician considers-opioids-for-comfort-focused-or-alternative-limited-contexts.
'consider-opioids-for-comfort-focused-or-alternative-limited-contexts'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-imp17-clinician consider-opioids-for-comfort-focused-or-alternative-limited-contexts?
guideline_query(yesno,'consider-opioids-for-comfort-focused-or-alternative-limited-contexts'('Rec02-imp17-clinician')).
