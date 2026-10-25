% cdc2022-opioid-s55-14.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
:- multifile(guideline_supersession/4).
:- discontiguous(guideline_supersession/4).
:- multifile(guideline_evaluation_date/1).
:- discontiguous(guideline_evaluation_date/1).
guideline_schema_version(1).
guideline_document('cdc2022-opioid-s55-14',ace_sha256(f6fe1e1b5d7307fd6ed8c0276f1bbd40aae59dbb42df684432bc2dfce5944c3b),ulex(sha256('64c086862c2f48d491a9fde0ba9f0fcf090e25e84456b9749332ec3ace211ce4')),vocabulary(sha256('405832ed3f275f091f60ded2e828af610266110b6c9a29536ca799d29ca43d72'))).
% S1: Every benzodiazepine sometimes can be beneficial.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s55-14',1,box(1),[A]),can) :- guideline_entity(actual,A,benzodiazepine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s55-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s55-14',1,ref(3),[A]),usuality,sometimes) :- guideline_entity(actual,A,benzodiazepine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_property('$guideline_id'(context,'cdc2022-opioid-s55-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s55-14',1,ref(2),[A]),beneficial,pos) :- guideline_entity(actual,A,benzodiazepine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s55-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s55-14',1,ref(3),[A]),be) :- guideline_entity(actual,A,benzodiazepine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s55-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s55-14',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,benzodiazepine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s55-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s55-14',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s55-14',1,ref(2),[A])) :- guideline_entity(actual,A,benzodiazepine,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every benzodiazepine-stopping can be destabilizing.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s55-14',2,box(1),[A]),can) :- guideline_entity(actual,A,'benzodiazepine-stopping',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_property('$guideline_id'(context,'cdc2022-opioid-s55-14',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s55-14',2,ref(2),[A]),destabilizing,pos) :- guideline_entity(actual,A,'benzodiazepine-stopping',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s55-14',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s55-14',2,ref(3),[A]),be) :- guideline_entity(actual,A,'benzodiazepine-stopping',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s55-14',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s55-14',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'benzodiazepine-stopping',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s55-14',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s55-14',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s55-14',2,ref(2),[A])) :- guideline_entity(actual,A,'benzodiazepine-stopping',countable), guideline_cardinality(actual,A,na,eq,1).
