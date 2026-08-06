% cdc2022-opioid-rec08.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec08',ace_sha256('03f8f2ea32e635777708e8e0ada2c8726f8b01a134a7a5c55a1e6bfcf76025fd'),ulex(sha256('7926023c0fffd5d10d6e87f7d250ae2232508730e5a50cdbdef79bb29aa7613b'))).
% S1: Rec08-clinician is a risk-mitigation-clinician.
'risk-mitigation-clinician'('Rec08-clinician').
% S2: Rec08-peer is a risk-mitigation-clinician.
'risk-mitigation-clinician'('Rec08-peer').
% S3: Every risk-mitigation-clinician offers-naloxone.
'offer-naloxone'(A) :- 'risk-mitigation-clinician'(A).
% S4: Who offers-naloxone?
guideline_query(who(A),'offer-naloxone'(A)).
