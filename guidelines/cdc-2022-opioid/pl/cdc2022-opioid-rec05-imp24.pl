% cdc2022-opioid-rec05-imp24.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec05-imp24',ace_sha256(e78f9d288a0eaf44bbecbf37dbc9c893c23e789c62f17529da90abbab8627676),ulex(sha256('26c94eb4c3e425db0b6c278b6861f816b856b8629608554394634e35ee8e1f3c'))).
% S1: Rec05-imp24-clinician is a dosage-change-clinician.
'dosage-change-clinician'('Rec05-imp24-clinician').
% S2: Every dosage-change-clinician provides-or-arranges-coordinated-management-of-pain-and-opioid-related-problems-including-opioid-use-disorder.
'provide-or-arrange-coordinated-management-of-pain-and-opioid-related-problems-including-opioid-use-disorder'(A) :- 'dosage-change-clinician'(A).
% S3: Does Rec05-imp24-clinician provide-or-arrange-coordinated-management-of-pain-and-opioid-related-problems-including-opioid-use-disorder?
guideline_query(yesno,'provide-or-arrange-coordinated-management-of-pain-and-opioid-related-problems-including-opioid-use-disorder'('Rec05-imp24-clinician')).
