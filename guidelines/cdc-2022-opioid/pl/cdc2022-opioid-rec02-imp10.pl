% cdc2022-opioid-rec02-imp10.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec02-imp10',ace_sha256('440dea5f4c9904c90a7e6236c055baeaff46b215fcb51d82151029124d200e92'),ulex(sha256(c879b5696dbd7394030ca8105226a5d860b57777791e2f4772f9325c07278683))).
% S1: Rec02-imp10-clinician is a subacute-chronic-pain-clinician.
'subacute-chronic-pain-clinician'('Rec02-imp10-clinician').
% S2: Every subacute-chronic-pain-clinician considers-selected-antidepressants-anticonvulsants-and-topical-agents-for-neuropathic-pain.
'consider-selected-antidepressants-anticonvulsants-and-topical-agents-for-neuropathic-pain'(A) :- 'subacute-chronic-pain-clinician'(A).
% S3: Does Rec02-imp10-clinician consider-selected-antidepressants-anticonvulsants-and-topical-agents-for-neuropathic-pain?
guideline_query(yesno,'consider-selected-antidepressants-anticonvulsants-and-topical-agents-for-neuropathic-pain'('Rec02-imp10-clinician')).
