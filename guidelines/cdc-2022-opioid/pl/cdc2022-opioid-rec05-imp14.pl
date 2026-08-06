% cdc2022-opioid-rec05-imp14.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec05-imp14',ace_sha256('13a734557adcecb7534734cd0e4da8b7533bdf72a894971c83b32b049ba69ff6'),ulex(sha256('453749766e15fefa9c2dbb81ef07028b238ee2cb462770b45c2c584b7cbc92e7'))).
% S1: Rec05-imp14-clinician is a dosage-change-clinician.
'dosage-change-clinician'('Rec05-imp14-clinician').
% S2: Every dosage-change-clinician considers-clinically-significant-withdrawal-symptoms-as-signal-to-further-slow-taper.
'consider-clinically-significant-withdrawal-symptoms-as-signal-to-further-slow-taper'(A) :- 'dosage-change-clinician'(A).
% S3: Does Rec05-imp14-clinician consider-clinically-significant-withdrawal-symptoms-as-signal-to-further-slow-taper?
guideline_query(yesno,'consider-clinically-significant-withdrawal-symptoms-as-signal-to-further-slow-taper'('Rec05-imp14-clinician')).
