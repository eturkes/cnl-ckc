% cdc2022-opioid-rec06.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec06',ace_sha256('4f45424d645b948f4e32145ed673c2718c58129c0325962658bab91b1d7291dd'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
% S1: Rec06-clinician is an acute-opioid-clinician.
'acute-opioid-clinician'('Rec06-clinician').
% S2: Every acute-opioid-clinician limits-prescription-quantity.
'limit-prescription-quantity'(A) :- 'acute-opioid-clinician'(A).
% S3: Does Rec06-clinician limit-prescription-quantity?
guideline_query(yesno,'limit-prescription-quantity'('Rec06-clinician')).
