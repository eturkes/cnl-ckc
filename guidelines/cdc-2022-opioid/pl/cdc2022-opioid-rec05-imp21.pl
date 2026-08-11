% cdc2022-opioid-rec05-imp21.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec05-imp21',ace_sha256('4b307fa29d53b248eb5131c113d3c72af7b0f76bb3de4a9f60ce564c710ea6a2'),ulex(sha256('26c94eb4c3e425db0b6c278b6861f816b856b8629608554394634e35ee8e1f3c'))).
% S1: Rec05-imp21-clinician is a dosage-change-clinician.
'dosage-change-clinician'('Rec05-imp21-clinician').
% S2: Every dosage-change-clinician remains-alert-to-and-screens-for-anxiety-depression-opioid-misuse-or-opioid-use-disorder-during-taper-and-provides-or-arranges-management.
'remain-alert-to-and-screen-for-anxiety-depression-opioid-misuse-or-opioid-use-disorder-during-taper-and-provide-or-arrange-management'(A) :- 'dosage-change-clinician'(A).
% S3: Does Rec05-imp21-clinician remain-alert-to-and-screen-for-anxiety-depression-opioid-misuse-or-opioid-use-disorder-during-taper-and-provide-or-arrange-management?
guideline_query(yesno,'remain-alert-to-and-screen-for-anxiety-depression-opioid-misuse-or-opioid-use-disorder-during-taper-and-provide-or-arrange-management'('Rec05-imp21-clinician')).
