% cdc2022-opioid-s56-21.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
:- multifile(guideline_grade/5).
:- discontiguous(guideline_grade/5).
:- multifile(guideline_preference/4).
:- discontiguous(guideline_preference/4).
:- multifile(guideline_qualifier/4).
:- discontiguous(guideline_qualifier/4).
:- multifile(guideline_unit/3).
:- discontiguous(guideline_unit/3).
:- multifile(guideline_exclusion/1).
:- discontiguous(guideline_exclusion/1).
guideline_schema_version(1).
guideline_document('cdc2022-opioid-s56-21',ace_sha256(ddfdede9c5788f82b168c15e43ba27696dc8de1c206f9681a1391ae819bdc920),ulex(sha256('45608095e3f77853fdabcce148a9eb7633a8ae776353b8f008c753b846886588')),vocabulary(sha256(c36289aa40f75805a83501ca057b29dd1ba7292b025716853f654890b77ec50c))).
% S1: Every cognitive-behavioral-therapy increases a tapering-success-rate.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s56-21',1,ref(2),[A]),'tapering-success-rate',countable) :- guideline_entity(actual,A,'cognitive-behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s56-21',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'cognitive-behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s56-21',1,ref(3),[A]),increase) :- guideline_entity(actual,A,'cognitive-behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s56-21',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'cognitive-behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s56-21',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s56-21',1,ref(2),[A])) :- guideline_entity(actual,A,'cognitive-behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: If a patient struggles with a benzodiazepine-taper then it is possible that a cognitive-behavioral-therapy particularly helps the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s56-21',2,box(1),[A,B,C]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'benzodiazepine-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,struggle), guideline_arg(actual,C,1,A), guideline_pp(actual,C,with,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s56-21',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s56-21',2,ref(4),[A,B,C]),'cognitive-behavioral-therapy',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'benzodiazepine-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,struggle), guideline_arg(actual,C,1,A), guideline_pp(actual,C,with,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s56-21',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s56-21',2,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'benzodiazepine-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,struggle), guideline_arg(actual,C,1,A), guideline_pp(actual,C,with,B).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s56-21',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s56-21',2,ref(5),[A,B,C]),focus,particularly) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'benzodiazepine-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,struggle), guideline_arg(actual,C,1,A), guideline_pp(actual,C,with,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s56-21',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s56-21',2,ref(5),[A,B,C]),help) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'benzodiazepine-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,struggle), guideline_arg(actual,C,1,A), guideline_pp(actual,C,with,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s56-21',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s56-21',2,ref(5),[A,B,C]),1,'$guideline_id'(product,'cdc2022-opioid-s56-21',2,ref(4),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'benzodiazepine-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,struggle), guideline_arg(actual,C,1,A), guideline_pp(actual,C,with,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s56-21',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s56-21',2,ref(5),[A,B,C]),2,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'benzodiazepine-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,struggle), guideline_arg(actual,C,1,A), guideline_pp(actual,C,with,B).
