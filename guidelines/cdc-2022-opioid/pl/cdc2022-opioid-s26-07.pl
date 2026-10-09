% cdc2022-opioid-s26-07.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s26-07',ace_sha256('342d9b7ecfac7876e4d34792c351df111e22eac4f52de209eb4c76fb7cac390e'),ulex(sha256(be24cf56c59049d3c31fba641013faa22821ecda0669443a1fc4a49979ef57ae)),temporal(sha256('29a90281976bb48dfa32d9ef80651076896404c524c9bf1d7a55b06c1880be2a'))).
% S1: Every primary-care-clinician may encourage an active-care-plan-role for a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s26-07',1,box(1),[A]),may) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',1,ref(2),[A]),'active-care-plan-role',countable) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s26-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',1,ref(4),[A]),encourage) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s26-07',1,ref(2),[A])) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s26-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',1,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s26-07',1,ref(3),[A])) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every primary-care-clinician may support a beneficial-activity-engagement for a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s26-07',2,box(1),[A]),may) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',2,ref(2),[A]),'beneficial-activity-engagement',countable) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',2,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s26-07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',2,ref(4),[A]),support) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s26-07',2,ref(2),[A])) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s26-07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',2,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s26-07',2,ref(3),[A])) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every primary-care-clinician may provide a relaxation-technique-education to a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(1),[A]),may) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(2),[A]),'relaxation-technique-education',countable) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(4),[A]),provide) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(2),[A])) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(4),[A]),3,'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(3),[A])) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every primary-care-clinician may provide a coping-strategy-education to a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s26-07',4,box(1),[A]),may) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(2),[A]),'coping-strategy-education',countable) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s26-07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(4),[A]),provide) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(2),[A])) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(4),[A]),3,'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(3),[A])) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
