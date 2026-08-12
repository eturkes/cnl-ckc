% cdc2022-opioid-s38-16.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s38-16',ace_sha256(e726ed7c920a94ba89663b22a4918b23bb46472a26755c83273f798322471954),ulex(sha256('06c1c26d6b6abe1436b8c94a276937ad072806062117417de2ab8086fdf9a3e9'))).
% S1: Every clinician may acknowledge a patient-taper-fear.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s38-16',1,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-16',1,ref(2),[A]),'patient-taper-fear',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-16',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s38-16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-16',1,ref(3),[A]),acknowledge) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-16',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-16',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s38-16',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician may ask a patient-support-question.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s38-16',2,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-16',2,ref(2),[A]),'patient-support-question',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-16',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s38-16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-16',2,ref(3),[A]),ask) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-16',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-16',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s38-16',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician may ensure an appropriate-and-accessible-psychosocial-support for a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s38-16',3,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-16',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-16',3,ref(2),[A]),'appropriate-and-accessible-psychosocial-support',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-16',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-16',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-16',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-16',3,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-16',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-16',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s38-16',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-16',3,ref(4),[A]),ensure) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-16',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-16',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-16',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-16',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s38-16',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s38-16',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-16',3,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s38-16',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
