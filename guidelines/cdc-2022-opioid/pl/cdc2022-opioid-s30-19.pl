% cdc2022-opioid-s30-19.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s30-19',ace_sha256(e5b7a53fa4daaba268f494448c09d9193fc7ea05baa566b81c1d6e582b07e7fb),ulex(sha256('64c086862c2f48d491a9fde0ba9f0fcf090e25e84456b9749332ec3ace211ce4')),vocabulary(sha256('405832ed3f275f091f60ded2e828af610266110b6c9a29536ca799d29ca43d72'))).
% S1: If a patient uses a nonopioid-pain-medication then it is possible that a nonpharmacologic-therapy provides a synergistic-benefit for the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s30-19',1,box(1),[A,B,C]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'nonopioid-pain-medication',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-19',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s30-19',1,ref(4),[A,B,C]),'nonpharmacologic-therapy',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'nonopioid-pain-medication',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-19',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s30-19',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'nonopioid-pain-medication',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-19',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s30-19',1,ref(5),[A,B,C]),'synergistic-benefit',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'nonopioid-pain-medication',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-19',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s30-19',1,ref(5),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'nonopioid-pain-medication',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s30-19',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s30-19',1,ref(6),[A,B,C]),provide) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'nonopioid-pain-medication',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-19',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s30-19',1,ref(6),[A,B,C]),1,'$guideline_id'(product,'cdc2022-opioid-s30-19',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'nonopioid-pain-medication',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-19',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s30-19',1,ref(6),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s30-19',1,ref(5),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'nonopioid-pain-medication',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s30-19',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s30-19',1,ref(6),[A,B,C]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'nonopioid-pain-medication',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S2: If a patient uses an opioid-pain-medication then it is possible that a nonpharmacologic-therapy provides a synergistic-benefit for the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s30-19',2,box(1),[A,B,C]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-pain-medication',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-19',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s30-19',2,ref(4),[A,B,C]),'nonpharmacologic-therapy',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-pain-medication',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-19',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s30-19',2,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-pain-medication',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-19',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s30-19',2,ref(5),[A,B,C]),'synergistic-benefit',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-pain-medication',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-19',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s30-19',2,ref(5),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-pain-medication',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s30-19',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s30-19',2,ref(6),[A,B,C]),provide) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-pain-medication',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-19',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s30-19',2,ref(6),[A,B,C]),1,'$guideline_id'(product,'cdc2022-opioid-s30-19',2,ref(4),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-pain-medication',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-19',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s30-19',2,ref(6),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s30-19',2,ref(5),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-pain-medication',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s30-19',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s30-19',2,ref(6),[A,B,C]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-pain-medication',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
