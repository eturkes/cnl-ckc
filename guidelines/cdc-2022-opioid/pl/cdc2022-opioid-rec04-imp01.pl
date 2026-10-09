% cdc2022-opioid-rec04-imp01.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
:- multifile(guideline_schema_version/1).
:- discontiguous(guideline_schema_version/1).
:- multifile(guideline_document/4).
:- discontiguous(guideline_document/4).
:- multifile(guideline_entity/4).
:- discontiguous(guideline_entity/4).
:- multifile(guideline_cardinality/5).
:- discontiguous(guideline_cardinality/5).
:- multifile(guideline_event/3).
:- discontiguous(guideline_event/3).
:- multifile(guideline_arg/4).
:- discontiguous(guideline_arg/4).
:- multifile(guideline_pp/4).
:- discontiguous(guideline_pp/4).
:- multifile(guideline_property/4).
:- discontiguous(guideline_property/4).
:- multifile(guideline_operator/3).
:- discontiguous(guideline_operator/3).
:- multifile(guideline_interval/6).
:- discontiguous(guideline_interval/6).
:- multifile(guideline_recurrence/4).
:- discontiguous(guideline_recurrence/4).
:- multifile(guideline_frequency/5).
:- discontiguous(guideline_frequency/5).
:- multifile(guideline_order/4).
:- discontiguous(guideline_order/4).
:- multifile(guideline_recurrence_window/7).
:- discontiguous(guideline_recurrence_window/7).
:- multifile(guideline_range/3).
:- discontiguous(guideline_range/3).
guideline_schema_version(3).
guideline_document('cdc2022-opioid-rec04-imp01',ace_sha256('88cff65692bd021f33a1e81725736bb63df0e9ae0d438fc5d5ee4af0cfcd50e7'),ulex(sha256('0b669018d532f506c518a6b0edffa4994f771b930477e7e2b36faf50d2b75d33')),temporal(sha256(d7b0d6c9af49f8d4b3dc12370d6b1cc20e923a69a5ce745ee26a1bc0f21effb6))).
% S1: An opioid-dosage-recommendation is not an inflexible-standard.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',1,ref(1),[]),'opioid-dosage-recommendation',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',1,ref(1),[]),na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec04-imp01',1,box(1),[]),-).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec04-imp01',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',1,ref(2),[]),'inflexible-standard',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec04-imp01',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',1,ref(2),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec04-imp01',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',1,ref(3),[]),be).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp01',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',1,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',1,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp01',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',1,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',1,ref(2),[])).
% S2: An opioid-dosage-recommendation is not a rigid-standard.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',2,ref(1),[]),'opioid-dosage-recommendation',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',2,ref(1),[]),na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec04-imp01',2,box(1),[]),-).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec04-imp01',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',2,ref(2),[]),'rigid-standard',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec04-imp01',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',2,ref(2),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec04-imp01',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',2,ref(3),[]),be).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp01',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',2,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',2,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp01',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',2,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',2,ref(2),[])).
% S3: An opioid-dosage-recommendation is a decision-guidepost.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',3,ref(1),[]),'opioid-dosage-recommendation',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',3,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',3,ref(2),[]),'decision-guidepost',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',3,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',3,ref(3),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',3,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',3,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',3,ref(2),[])).
% S4: A decision-guidepost can inform a clinician-patient-decision.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',4,ref(1),[]),'decision-guidepost',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',4,ref(1),[]),na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec04-imp01',4,box(1),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec04-imp01',4,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',4,ref(2),[]),'clinician-patient-decision',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec04-imp01',4,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',4,ref(2),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec04-imp01',4,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',4,ref(3),[]),inform).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp01',4,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',4,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',4,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp01',4,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',4,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec04-imp01',4,ref(2),[])).
