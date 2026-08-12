% cdc2022-opioid-s26-08.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s26-08',ace_sha256('60ab539095021ce53ec0e10dcd30c00d08b3d4a5f2ef79059efc3641a48e0ecd'),ulex(sha256('06c1c26d6b6abe1436b8c94a276937ad072806062117417de2ab8086fdf9a3e9'))).
% S1: Every clinician should understand a community-nonpharmacologic-option.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s26-08',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-08',1,ref(2),[A]),'community-nonpharmacologic-option',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-08',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s26-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-08',1,ref(3),[A]),understand) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-08',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-08',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s26-08',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: If a clinician understands a community-nonpharmacologic-option then the clinician may refer a patient to a low-cost-service.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s26-08',2,box(1),[A,B,C]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'community-nonpharmacologic-option',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-08',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s26-08',2,ref(4),[A,B,C]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'community-nonpharmacologic-option',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-08',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s26-08',2,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'community-nonpharmacologic-option',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-08',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s26-08',2,ref(5),[A,B,C]),'low-cost-service',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'community-nonpharmacologic-option',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-08',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s26-08',2,ref(5),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'community-nonpharmacologic-option',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s26-08',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s26-08',2,ref(6),[A,B,C]),refer) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'community-nonpharmacologic-option',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-08',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s26-08',2,ref(6),[A,B,C]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'community-nonpharmacologic-option',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-08',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s26-08',2,ref(6),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s26-08',2,ref(4),[A,B,C])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'community-nonpharmacologic-option',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s26-08',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s26-08',2,ref(6),[A,B,C]),to,'$guideline_id'(product,'cdc2022-opioid-s26-08',2,ref(5),[A,B,C])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'community-nonpharmacologic-option',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
