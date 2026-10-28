% cdc2022-opioid-rec04-imp08.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec04-imp08',ace_sha256(f22f87ed5cb3266ec67e797c0be9db540f48779f7881e7ee786d2f65a91991c6),ulex(sha256('64c086862c2f48d491a9fde0ba9f0fcf090e25e84456b9749332ec3ace211ce4')),vocabulary(sha256('06621a3dca592ca4385262a4a36dd8184670cdaef31827cd8b224c8c870c2f7d'))).
% S1: If a clinician decides a dosage-increase then the clinician should cautiously increase an opioid-dosage and should use a smallest-practical-increase.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(1),[A,B,C]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(4),[A,B,C]),'opioid-dosage',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(5),[A,B,C]),manner,cautiously) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(5),[A,B,C]),increase) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(5),[A,B,C]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(5),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(2),[A,B,C]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(6),[A,B,C]),'smallest-practical-increase',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(6),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(7),[A,B,C]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(7),[A,B,C]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(7),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(6),[A,B,C])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
