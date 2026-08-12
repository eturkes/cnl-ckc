% cdc2022-opioid-s23-14.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s23-14',ace_sha256('3bebde236bbf71aa1cef00a9d771826baec8b04d163b45f03fe154e5c4b86a23'),ulex(sha256('06c1c26d6b6abe1436b8c94a276937ad072806062117417de2ab8086fdf9a3e9'))).
% S1: Every clinician should discuss an intentional-or-unintentional-opioid-sharing-risk-for-household-or-other-person with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-14',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(2),[A]),'intentional-or-unintentional-opioid-sharing-risk-for-household-or-other-person',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should discuss a shared-opioid-overdose-at-same-or-lower-dosage-risk with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-14',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-14',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(2),[A]),'shared-opioid-overdose-at-same-or-lower-dosage-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-14',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-14',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-14',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-14',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-14',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-14',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-14',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should discuss a young-child-or-pet-unintentional-opioid-ingestion-risk with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-14',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-14',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(2),[A]),'young-child-or-pet-unintentional-opioid-ingestion-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-14',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-14',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-14',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-14',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-14',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-14',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-14',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
