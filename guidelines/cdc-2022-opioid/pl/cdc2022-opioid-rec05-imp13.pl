% cdc2022-opioid-rec05-imp13.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec05-imp13',ace_sha256(ab6f6c5b6d90ecef0e051c8fdefe4adc3b5481856efe7354bad400fd07edc0a8),ulex(sha256('26c94eb4c3e425db0b6c278b6861f816b856b8629608554394634e35ee8e1f3c'))).
% S1: Rec05-imp13-clinician is a dosage-change-clinician.
'dosage-change-clinician'('Rec05-imp13-clinician').
% S2: Every dosage-change-clinician maximizes-nonopioid-pain-treatment-and-addresses-behavioral-distress-for-patients-struggling-to-tolerate-taper.
'maximize-nonopioid-pain-treatment-and-address-behavioral-distress-for-patients-struggling-to-tolerate-taper'(A) :- 'dosage-change-clinician'(A).
% S3: Does Rec05-imp13-clinician maximize-nonopioid-pain-treatment-and-address-behavioral-distress-for-patients-struggling-to-tolerate-taper?
guideline_query(yesno,'maximize-nonopioid-pain-treatment-and-address-behavioral-distress-for-patients-struggling-to-tolerate-taper'('Rec05-imp13-clinician')).
