% cdc2022-opioid-rec03-imp02.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec03-imp02',ace_sha256(a38603e0291032141522e20658de9d4c856468e3d48bc3912265b13851a11444),ulex(sha256('453749766e15fefa9c2dbb81ef07028b238ee2cb462770b45c2c584b7cbc92e7'))).
% S1: Rec03-imp02-clinician is a starting-opioid-clinician.
'starting-opioid-clinician'('Rec03-imp02-clinician').
% S2: Every starting-opioid-clinician reserves-er-la-opioids-for-severe-continuous-pain.
'reserve-er-la-opioids-for-severe-continuous-pain'(A) :- 'starting-opioid-clinician'(A).
% S3: Does Rec03-imp02-clinician reserve-er-la-opioids-for-severe-continuous-pain?
guideline_query(yesno,'reserve-er-la-opioids-for-severe-continuous-pain'('Rec03-imp02-clinician')).
