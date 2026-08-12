% cdc2022-opioid-s23-10.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s23-10',ace_sha256('351c489c17d8324b79a3578aa0825b787b2ca56a308e24d038fe762fc1e24334'),ulex(sha256('6bd0df91739d5a42b1a472d866e0dd72897517f8e4652458e4893adbed9cc663'))).
% S1: Every clinician should discuss an opioid-driving-or-machinery-safety-effect with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-10',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-10',1,ref(2),[A]),'opioid-driving-or-machinery-safety-effect',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-10',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-10',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-10',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-10',1,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-10',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-10',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-10',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-10',1,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-10',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should discuss an opioid-initiation-driving-or-machinery-safety-effect with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-10',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-10',2,ref(2),[A]),'opioid-initiation-driving-or-machinery-safety-effect',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-10',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-10',2,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-10',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-10',2,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-10',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-10',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-10',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-10',2,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-10',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should discuss a concurrent-central-nervous-system-depressant-driving-or-machinery-safety-effect with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-10',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-10',3,ref(2),[A]),'concurrent-central-nervous-system-depressant-driving-or-machinery-safety-effect',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-10',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-10',3,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-10',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-10',3,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-10',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-10',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-10',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-10',3,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-10',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
