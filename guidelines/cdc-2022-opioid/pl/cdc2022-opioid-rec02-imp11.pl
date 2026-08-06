% cdc2022-opioid-rec02-imp11.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp11',ace_sha256('1cc4d173cb0941b2a903bde5da85d05cdeb387df22da14c4a647757a2f07579b'),ulex(sha256('3ffe6d9f23ab3109d7803e1121935f32d53a7768425248b943679743da1ed101'))).
% S1: Rec02-imp11-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp11-clinician').
% S2: Every subacute-chronic-pain-clinician decides-judiciously-case-by-case-about-tricyclic-antidepressants-in-older-adults.
'decide-judiciously-case-by-case-about-tricyclic-antidepressants-in-older-adults'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-imp11-clinician decide-judiciously-case-by-case-about-tricyclic-antidepressants-in-older-adults?
guideline_query(yesno,'decide-judiciously-case-by-case-about-tricyclic-antidepressants-in-older-adults'('Rec02-imp11-clinician')).
