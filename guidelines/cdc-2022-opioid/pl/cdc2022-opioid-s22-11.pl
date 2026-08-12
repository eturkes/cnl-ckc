% cdc2022-opioid-s22-11.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
:- multifile(guideline_schema_version/1).
:- discontiguous(guideline_schema_version/1).
:- multifile(guideline_document/3).
:- discontiguous(guideline_document/3).
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
guideline_schema_version(1).
guideline_document('cdc2022-opioid-s22-11',ace_sha256(bed13f5c554d41f56a31f7ba540798ce461dce3797d43ce6c223554086524bb7),ulex(sha256('6bd0df91739d5a42b1a472d866e0dd72897517f8e4652458e4893adbed9cc663'))).
% S1: A patient-education is critical before an outpatient-opioid-therapy.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s22-11',1,ref(1),[]),'patient-education',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s22-11',1,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s22-11',1,ref(2),[]),'outpatient-opioid-therapy',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s22-11',1,ref(2),[]),na,eq,1).
guideline_property(actual,'$guideline_id'(product,'cdc2022-opioid-s22-11',1,ref(3),[]),critical,pos).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s22-11',1,ref(4),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s22-11',1,ref(4),[]),1,'$guideline_id'(product,'cdc2022-opioid-s22-11',1,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s22-11',1,ref(4),[]),2,'$guideline_id'(product,'cdc2022-opioid-s22-11',1,ref(3),[])).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s22-11',1,ref(4),[]),before,'$guideline_id'(product,'cdc2022-opioid-s22-11',1,ref(2),[])).
% S2: A patient-discussion is critical before an outpatient-opioid-therapy.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s22-11',2,ref(1),[]),'patient-discussion',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s22-11',2,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s22-11',2,ref(2),[]),'outpatient-opioid-therapy',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s22-11',2,ref(2),[]),na,eq,1).
guideline_property(actual,'$guideline_id'(product,'cdc2022-opioid-s22-11',2,ref(3),[]),critical,pos).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s22-11',2,ref(4),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s22-11',2,ref(4),[]),1,'$guideline_id'(product,'cdc2022-opioid-s22-11',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s22-11',2,ref(4),[]),2,'$guideline_id'(product,'cdc2022-opioid-s22-11',2,ref(3),[])).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s22-11',2,ref(4),[]),before,'$guideline_id'(product,'cdc2022-opioid-s22-11',2,ref(2),[])).
% S3: A patient-preference can inform a clinical-decision.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s22-11',3,ref(1),[]),'patient-preference',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s22-11',3,ref(1),[]),na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s22-11',3,box(1),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s22-11',3,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s22-11',3,ref(2),[]),'clinical-decision',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s22-11',3,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s22-11',3,ref(2),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s22-11',3,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s22-11',3,ref(3),[]),inform).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s22-11',3,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s22-11',3,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s22-11',3,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s22-11',3,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s22-11',3,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s22-11',3,ref(2),[])).
% S4: A patient-value can inform a clinical-decision.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s22-11',4,ref(1),[]),'patient-value',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s22-11',4,ref(1),[]),na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s22-11',4,box(1),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s22-11',4,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s22-11',4,ref(2),[]),'clinical-decision',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s22-11',4,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s22-11',4,ref(2),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s22-11',4,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s22-11',4,ref(3),[]),inform).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s22-11',4,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s22-11',4,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s22-11',4,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s22-11',4,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s22-11',4,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s22-11',4,ref(2),[])).
