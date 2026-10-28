% cdc2022-opioid-s48-17.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s48-17',ace_sha256('41ee6e43f76d559107e946bb28d8a9ea63a116bdd7c9c457b1c2bbc75c79ce49'),ulex(sha256('64c086862c2f48d491a9fde0ba9f0fcf090e25e84456b9749332ec3ace211ce4')),vocabulary(sha256('06621a3dca592ca4385262a4a36dd8184670cdaef31827cd8b224c8c870c2f7d'))).
% S1: Every depression-treatment can improve a pain-symptom and can improve a depression.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s48-17',1,box(1),[A]),can) :- guideline_entity(actual,A,'depression-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s48-17',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s48-17',1,ref(2),[A]),'pain-symptom',countable) :- guideline_entity(actual,A,'depression-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s48-17',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s48-17',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'depression-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s48-17',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s48-17',1,ref(3),[A]),improve) :- guideline_entity(actual,A,'depression-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s48-17',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s48-17',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'depression-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s48-17',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s48-17',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s48-17',1,ref(2),[A])) :- guideline_entity(actual,A,'depression-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s48-17',1,box(2),[A]),can) :- guideline_entity(actual,A,'depression-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s48-17',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s48-17',1,ref(4),[A]),depression,countable) :- guideline_entity(actual,A,'depression-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s48-17',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s48-17',1,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'depression-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s48-17',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s48-17',1,ref(5),[A]),improve) :- guideline_entity(actual,A,'depression-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s48-17',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s48-17',1,ref(5),[A]),1,A) :- guideline_entity(actual,A,'depression-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s48-17',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s48-17',1,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s48-17',1,ref(4),[A])) :- guideline_entity(actual,A,'depression-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every depression-treatment can possibly decrease an overdose-risk.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s48-17',2,box(1),[A]),can) :- guideline_entity(actual,A,'depression-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s48-17',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s48-17',2,ref(2),[A]),'overdose-risk',countable) :- guideline_entity(actual,A,'depression-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s48-17',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s48-17',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'depression-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s48-17',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s48-17',2,ref(3),[A]),possibility,possibly) :- guideline_entity(actual,A,'depression-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s48-17',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s48-17',2,ref(3),[A]),decrease) :- guideline_entity(actual,A,'depression-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s48-17',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s48-17',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'depression-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s48-17',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s48-17',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s48-17',2,ref(2),[A])) :- guideline_entity(actual,A,'depression-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
