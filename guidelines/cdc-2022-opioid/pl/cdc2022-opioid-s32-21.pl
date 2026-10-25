% cdc2022-opioid-s32-21.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s32-21',ace_sha256(a7f0008bcb5112fcf499c0d04039dae8646aac0a234d0c97072fdc0eeae52d05),ulex(sha256('64c086862c2f48d491a9fde0ba9f0fcf090e25e84456b9749332ec3ace211ce4')),vocabulary(sha256(c36289aa40f75805a83501ca057b29dd1ba7292b025716853f654890b77ec50c))).
% S1: If a patient receives an opioid-dosage-increase to at least 50 MME-units-per-day then the patient can often lack a pain-or-function-benefit from the opioid-dosage-increase.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s32-21',1,box(1),[A,B,C,D]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'MME-unit-per-day',countable), guideline_cardinality(actual,C,na,E,F), guideline_satisfies(E,F,geq,50), guideline_event(actual,D,receive), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,to,C).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s32-21',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s32-21',1,ref(5),[A,B,C,D]),'pain-or-function-benefit',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'MME-unit-per-day',countable), guideline_cardinality(actual,C,na,E,F), guideline_satisfies(E,F,geq,50), guideline_event(actual,D,receive), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,to,C).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s32-21',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s32-21',1,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'MME-unit-per-day',countable), guideline_cardinality(actual,C,na,E,F), guideline_satisfies(E,F,geq,50), guideline_event(actual,D,receive), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,to,C).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s32-21',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s32-21',1,ref(6),[A,B,C,D]),usuality,often) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'MME-unit-per-day',countable), guideline_cardinality(actual,C,na,E,F), guideline_satisfies(E,F,geq,50), guideline_event(actual,D,receive), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,to,C).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s32-21',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s32-21',1,ref(6),[A,B,C,D]),lack) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'MME-unit-per-day',countable), guideline_cardinality(actual,C,na,E,F), guideline_satisfies(E,F,geq,50), guideline_event(actual,D,receive), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,to,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s32-21',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s32-21',1,ref(6),[A,B,C,D]),1,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'MME-unit-per-day',countable), guideline_cardinality(actual,C,na,E,F), guideline_satisfies(E,F,geq,50), guideline_event(actual,D,receive), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,to,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s32-21',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s32-21',1,ref(6),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-s32-21',1,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'MME-unit-per-day',countable), guideline_cardinality(actual,C,na,E,F), guideline_satisfies(E,F,geq,50), guideline_event(actual,D,receive), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,to,C).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s32-21',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s32-21',1,ref(6),[A,B,C,D]),from,B) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'MME-unit-per-day',countable), guideline_cardinality(actual,C,na,E,F), guideline_satisfies(E,F,geq,50), guideline_event(actual,D,receive), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,to,C).
% S2: If an opioid-dosage increases then a patient-risk progressively increases.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s32-21',2,ref(3),[A,B]),'patient-risk',countable) :- guideline_entity(actual,A,'opioid-dosage',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,B,increase), guideline_arg(actual,B,1,A).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s32-21',2,ref(3),[A,B]),na,eq,1) :- guideline_entity(actual,A,'opioid-dosage',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,B,increase), guideline_arg(actual,B,1,A).
guideline_qualifier(actual,'$guideline_id'(product,'cdc2022-opioid-s32-21',2,ref(4),[A,B]),manner,progressively) :- guideline_entity(actual,A,'opioid-dosage',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,B,increase), guideline_arg(actual,B,1,A).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s32-21',2,ref(4),[A,B]),increase) :- guideline_entity(actual,A,'opioid-dosage',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,B,increase), guideline_arg(actual,B,1,A).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s32-21',2,ref(4),[A,B]),1,'$guideline_id'(product,'cdc2022-opioid-s32-21',2,ref(3),[A,B])) :- guideline_entity(actual,A,'opioid-dosage',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,B,increase), guideline_arg(actual,B,1,A).
