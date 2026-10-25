% cdc2022-opioid-s41-08.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s41-08',ace_sha256('29788e04a9ad418ad95ddfdae6e18023ab6b2d477a25c5ff9a976b93199739b2'),ulex(sha256('64c086862c2f48d491a9fde0ba9f0fcf090e25e84456b9749332ec3ace211ce4')),vocabulary(sha256('405832ed3f275f091f60ded2e828af610266110b6c9a29536ca799d29ca43d72'))).
% S1: Every prescribed-opioid-duration-limitation can minimize an opioid-taper-need for a distressing-or-unpleasant-withdrawal-symptom-prevention.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s41-08',1,box(1),[A]),can) :- guideline_entity(actual,A,'prescribed-opioid-duration-limitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s41-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s41-08',1,ref(2),[A]),'opioid-taper-need',countable) :- guideline_entity(actual,A,'prescribed-opioid-duration-limitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s41-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s41-08',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'prescribed-opioid-duration-limitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s41-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s41-08',1,ref(3),[A]),'distressing-or-unpleasant-withdrawal-symptom-prevention',countable) :- guideline_entity(actual,A,'prescribed-opioid-duration-limitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s41-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s41-08',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'prescribed-opioid-duration-limitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s41-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s41-08',1,ref(4),[A]),minimize) :- guideline_entity(actual,A,'prescribed-opioid-duration-limitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s41-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s41-08',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,'prescribed-opioid-duration-limitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s41-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s41-08',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s41-08',1,ref(2),[A])) :- guideline_entity(actual,A,'prescribed-opioid-duration-limitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s41-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s41-08',1,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s41-08',1,ref(3),[A])) :- guideline_entity(actual,A,'prescribed-opioid-duration-limitation',countable), guideline_cardinality(actual,A,na,eq,1).
