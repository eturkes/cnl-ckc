% cdc2022-opioid-s7-02.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s7-02',ace_sha256(c471e56c9c9e2a4e4d5204e2f856d60dae81f2d937850a4d5765c0ed4ebb7c4e),ulex(sha256(ef57ba7180864095bab88b95f11ddb1a2c858593eba715dbfbe874db09506c5b))).
% S1: Every recommendation does not apply during a hospital-care.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s7-02',1,box(1),[A]),-) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s7-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s7-02',1,ref(2),[A]),'hospital-care',countable) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s7-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s7-02',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s7-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s7-02',1,ref(3),[A]),apply) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s7-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s7-02',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s7-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s7-02',1,ref(3),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s7-02',1,ref(2),[A])) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every recommendation does not apply during an emergency-department-care.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s7-02',2,box(1),[A]),-) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s7-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s7-02',2,ref(2),[A]),'emergency-department-care',countable) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s7-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s7-02',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s7-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s7-02',2,ref(3),[A]),apply) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s7-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s7-02',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s7-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s7-02',2,ref(3),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s7-02',2,ref(2),[A])) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every recommendation does not apply during an observational-setting-care with a possible-inpatient-admission.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s7-02',3,box(1),[A]),-) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s7-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s7-02',3,ref(2),[A]),'observational-setting-care',countable) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s7-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s7-02',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s7-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s7-02',3,ref(3),[A]),'possible-inpatient-admission',countable) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s7-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s7-02',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s7-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s7-02',3,ref(4),[A]),apply) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s7-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s7-02',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s7-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s7-02',3,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s7-02',3,ref(3),[A])) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s7-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s7-02',3,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s7-02',3,ref(2),[A])) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
