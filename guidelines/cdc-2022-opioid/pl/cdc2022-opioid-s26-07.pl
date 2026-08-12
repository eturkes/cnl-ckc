% cdc2022-opioid-s26-07.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s26-07',ace_sha256(b4e159c1fe54980ec1a6e49b24edfb4e7400d83e5e14cf50072748a0e62dcd56),ulex(sha256('2a4a3e829c919b3dd3b20356e7a69585cfb305c3c57dfb531e3f2c1eb97637b6'))).
% S1: Every clinician may encourage an active-care-plan-role for a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s26-07',1,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',1,ref(2),[A]),'active-care-plan-role',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s26-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',1,ref(4),[A]),encourage) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s26-07',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s26-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',1,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s26-07',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician may support a beneficial-activity-engagement for a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s26-07',2,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',2,ref(2),[A]),'beneficial-activity-engagement',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',2,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s26-07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',2,ref(4),[A]),support) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s26-07',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s26-07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',2,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s26-07',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician may provide a relaxation-technique-education to a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(2),[A]),'relaxation-technique-education',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(4),[A]),provide) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(4),[A]),to,'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician may provide a coping-strategy-education to a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s26-07',4,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(2),[A]),'coping-strategy-education',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s26-07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(4),[A]),provide) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s26-07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(4),[A]),to,'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
