% cdc2022-opioid-rec02-imp06.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp06',ace_sha256('1adf34a6d2521c31e306b14e6f602d5ffe915f91f239539494cf87f17b4458e2'),ulex(sha256('7855e6f992b6a70f49da5dbd1de6620893b143919a2f28e6b50d67c2eabb7dff'))).
% S1: Rec02-imp06-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp06-clinician').
% S2: Every subacute-chronic-pain-clinician uses-topical-nsaids-for-superficial-joint-osteoarthritis-after-insufficient-nonpharmacologic-response.
'use-topical-nsaids-for-superficial-joint-osteoarthritis-after-insufficient-nonpharmacologic-response'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-imp06-clinician use-topical-nsaids-for-superficial-joint-osteoarthritis-after-insufficient-nonpharmacologic-response?
guideline_query(yesno,'use-topical-nsaids-for-superficial-joint-osteoarthritis-after-insufficient-nonpharmacologic-response'('Rec02-imp06-clinician')).
