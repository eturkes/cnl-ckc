% cdc2022-opioid-s34-14.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s34-14',ace_sha256('9b6f001b95c301598e54a5f37b49da313fa0b4420b141a3b14825748b49d168b'),ulex(sha256('06c1c26d6b6abe1436b8c94a276937ad072806062117417de2ab8086fdf9a3e9'))).
% S1: Every clinician should review a state-MME-threshold-policy and should review a state-clinical-protocol.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s34-14',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s34-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s34-14',1,ref(2),[A]),'state-MME-threshold-policy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s34-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s34-14',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s34-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s34-14',1,ref(3),[A]),review) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s34-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s34-14',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s34-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s34-14',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s34-14',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s34-14',1,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s34-14',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s34-14',1,ref(4),[A]),'state-clinical-protocol',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s34-14',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s34-14',1,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s34-14',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s34-14',1,ref(5),[A]),review) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s34-14',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s34-14',1,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s34-14',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s34-14',1,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s34-14',1,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
