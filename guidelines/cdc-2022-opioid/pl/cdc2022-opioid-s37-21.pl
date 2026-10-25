% cdc2022-opioid-s37-21.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s37-21',ace_sha256('0b3bbc9830d24e797faab4c8a302efda95f90b139f43ecf357855d690c98c8be'),ulex(sha256('64c086862c2f48d491a9fde0ba9f0fcf090e25e84456b9749332ec3ace211ce4')),vocabulary(sha256('405832ed3f275f091f60ded2e828af610266110b6c9a29536ca799d29ca43d72'))).
% S1: Every opioid-taper sometimes can be harmful.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s37-21',1,box(1),[A]),can) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s37-21',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-21',1,ref(3),[A]),usuality,sometimes) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_property('$guideline_id'(context,'cdc2022-opioid-s37-21',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-21',1,ref(2),[A]),harmful,pos) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s37-21',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-21',1,ref(3),[A]),be) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-21',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-21',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-21',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-21',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s37-21',1,ref(2),[A])) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: If a patient receives a long-term-opioid-therapy then a continued-opioid-therapy-benefit can possibly include a taper-and-discontinuation-risk-avoidance.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s37-21',2,ref(4),[A,B,C]),'continued-opioid-therapy-benefit',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s37-21',2,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s37-21',2,box(1),[A,B,C]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s37-21',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s37-21',2,ref(5),[A,B,C]),'taper-and-discontinuation-risk-avoidance',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s37-21',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s37-21',2,ref(5),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s37-21',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s37-21',2,ref(6),[A,B,C]),possibility,possibly) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s37-21',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s37-21',2,ref(6),[A,B,C]),include) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-21',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s37-21',2,ref(6),[A,B,C]),1,'$guideline_id'(product,'cdc2022-opioid-s37-21',2,ref(4),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-21',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s37-21',2,ref(6),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s37-21',2,ref(5),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
