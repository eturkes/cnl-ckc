% cdc2022-opioid-rec01-imp06.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec01-imp06',ace_sha256(ab0d38f0ce3a14ed76d9aef510fed3d39f501ff98e6c5a54d7675d2aa0762d5f),ulex(sha256('453749766e15fefa9c2dbb81ef07028b238ee2cb462770b45c2c584b7cbc92e7'))).
% S1: Rec01-imp06-clinician is an acute-pain-clinician.
'acute-pain-clinician'('Rec01-imp06-clinician').
% S2: Every acute-pain-clinician considers-opioid-therapy-for-severe-traumatic-injuries-invasive-surgeries-and-other-severe-acute-pain-when-nsaids-and-other-therapies-are-contraindicated-or-likely-ineffective.
'consider-opioid-therapy-for-severe-traumatic-injuries-invasive-surgeries-and-other-severe-acute-pain-when-nsaids-and-other-therapies-are-contraindicated-or-likely-ineffective'(A) :- 'acute-pain-clinician'(A).
% S3: Does Rec01-imp06-clinician consider-opioid-therapy-for-severe-traumatic-injuries-invasive-surgeries-and-other-severe-acute-pain-when-nsaids-and-other-therapies-are-contraindicated-or-likely-ineffective?
guideline_query(yesno,'consider-opioid-therapy-for-severe-traumatic-injuries-invasive-surgeries-and-other-severe-acute-pain-when-nsaids-and-other-therapies-are-contraindicated-or-likely-ineffective'('Rec01-imp06-clinician')).
