% cdc2022-opioid-rec02-imp01.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec02-imp01',ace_sha256('4312bb7803bb55888e91780443426d1cfccd67005f740ba3984dcc09004c17ad'),ulex(sha256('06c1c26d6b6abe1436b8c94a276937ad072806062117417de2ab8086fdf9a3e9'))).
% S1: Every clinician should evaluate a patient and should determine a diagnosis during a patient-specific-therapy-selection.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp01',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp01',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp01',1,ref(2),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp01',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp01',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp01',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp01',1,ref(3),[A]),evaluate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp01',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp01',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp01',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp01',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp01',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp01',1,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp01',1,ref(4),[A]),diagnosis,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp01',1,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp01',1,ref(5),[A]),'patient-specific-therapy-selection',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp01',1,ref(5),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp01',1,ref(6),[A]),determine) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp01',1,ref(6),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp01',1,ref(6),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp01',1,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec02-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp01',1,ref(6),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec02-imp01',1,ref(5),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
