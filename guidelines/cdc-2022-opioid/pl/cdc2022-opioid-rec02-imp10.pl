% cdc2022-opioid-rec02-imp10.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp10',ace_sha256('440dea5f4c9904c90a7e6236c055baeaff46b215fcb51d82151029124d200e92'),ulex(sha256('7855e6f992b6a70f49da5dbd1de6620893b143919a2f28e6b50d67c2eabb7dff'))).
% S1: Rec02-imp10-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp10-clinician').
% S2: Every subacute-chronic-pain-clinician considers-selected-antidepressants-anticonvulsants-and-topical-agents-for-neuropathic-pain.
'consider-selected-antidepressants-anticonvulsants-and-topical-agents-for-neuropathic-pain'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-imp10-clinician consider-selected-antidepressants-anticonvulsants-and-topical-agents-for-neuropathic-pain?
guideline_query(yesno,'consider-selected-antidepressants-anticonvulsants-and-topical-agents-for-neuropathic-pain'('Rec02-imp10-clinician')).
