% cdc2022-opioid-rec01-imp03.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec01-imp03',ace_sha256(e6f9ebe538985276a2671062e81f075b3d30ad7fb5a09c58db5f2376b0588675),ulex(sha256('3ffe6d9f23ab3109d7803e1121935f32d53a7768425248b943679743da1ed101'))).
% S1: Rec01-imp03-clinician is an acute-pain-clinician.
'acute-pain-clinician'('Rec01-imp03-clinician').
% S2: Every acute-pain-clinician prescribes-opioids-only-as-needed-and-encourages-taper-after-around-the-clock-use.
'prescribe-opioids-only-as-needed-and-encourage-taper-after-around-the-clock-use'(A) :- 'acute-pain-clinician'(A).
% S3: Does Rec01-imp03-clinician prescribe-opioids-only-as-needed-and-encourage-taper-after-around-the-clock-use?
guideline_query(yesno,'prescribe-opioids-only-as-needed-and-encourage-taper-after-around-the-clock-use'('Rec01-imp03-clinician')).
