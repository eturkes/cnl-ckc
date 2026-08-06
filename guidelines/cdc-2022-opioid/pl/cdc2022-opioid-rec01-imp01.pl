% cdc2022-opioid-rec01-imp01.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec01-imp01',ace_sha256(ef1577f37d551e4e75d043d249ea968b9bc2485a461e0f90addbc69d5065bc55),ulex(sha256('453749766e15fefa9c2dbb81ef07028b238ee2cb462770b45c2c584b7cbc92e7'))).
% S1: Rec01-imp01-clinician is an acute-pain-clinician.
'acute-pain-clinician'('Rec01-imp01-clinician').
% S2: Every acute-pain-clinician maximizes-nonopioid-therapy.
'maximize-nonopioid-therapy'(A) :- 'acute-pain-clinician'(A).
% S3: Does Rec01-imp01-clinician maximize-nonopioid-therapy?
guideline_query(yesno,'maximize-nonopioid-therapy'('Rec01-imp01-clinician')).
