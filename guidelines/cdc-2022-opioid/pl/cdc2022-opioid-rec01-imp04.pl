% cdc2022-opioid-rec01-imp04.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec01-imp04',ace_sha256(b06ed2ec3fb2db3b4479d3c6079b14676ba6bcfdc4fc03184a98f38439bbe218),ulex(sha256('7855e6f992b6a70f49da5dbd1de6620893b143919a2f28e6b50d67c2eabb7dff'))).
% S1: Rec01-imp04-clinician is an acute-pain-clinician.
'acute-pain-clinician'('Rec01-imp04-clinician').
% S2: Every acute-pain-clinician uses-nonopioids-when-possible-and-limits-additional-opioids-to-severe-pain-duration-for-long-term-opioid-patients-with-acute-pain.
'use-nonopioids-when-possible-and-limit-additional-opioids-to-severe-pain-duration-for-long-term-opioid-patients-with-acute-pain'(A) :- 'acute-pain-clinician'(A).
% S3: Does Rec01-imp04-clinician use-nonopioids-when-possible-and-limit-additional-opioids-to-severe-pain-duration-for-long-term-opioid-patients-with-acute-pain?
guideline_query(yesno,'use-nonopioids-when-possible-and-limit-additional-opioids-to-severe-pain-duration-for-long-term-opioid-patients-with-acute-pain'('Rec01-imp04-clinician')).
