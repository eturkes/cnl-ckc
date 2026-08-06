% cdc2022-opioid-rec02-imp08.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp08',ace_sha256('22a4f02f6b76eb538814d36e7d6ae26c4f1f011b2517543d365f8731d394e237'),ulex(sha256('3ffe6d9f23ab3109d7803e1121935f32d53a7768425248b943679743da1ed101'))).
% S1: Rec02-imp08-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp08-clinician').
% S2: Every subacute-chronic-pain-clinician uses-nsaids-at-lowest-effective-dose-for-shortest-duration-with-caution.
'use-nsaids-at-lowest-effective-dose-for-shortest-duration-with-caution'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-imp08-clinician use-nsaids-at-lowest-effective-dose-for-shortest-duration-with-caution?
guideline_query(yesno,'use-nsaids-at-lowest-effective-dose-for-shortest-duration-with-caution'('Rec02-imp08-clinician')).
