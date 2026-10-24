% cdc2022-opioid-s10-15.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s10-15',ace_sha256('3441c1c95e510163ef6565b03034736d3558265663b4bf652415f2c6592778b3'),ulex(sha256('64c086862c2f48d491a9fde0ba9f0fcf090e25e84456b9749332ec3ace211ce4')),vocabulary(sha256(c36289aa40f75805a83501ca057b29dd1ba7292b025716853f654890b77ec50c))).
% S1: If a patient receives a hospice-care then the patient receives an end-of-life-care.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s10-15',1,ref(4),[A,B,C]),'end-of-life-care',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'hospice-care',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s10-15',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'hospice-care',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s10-15',1,ref(5),[A,B,C]),receive) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'hospice-care',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s10-15',1,ref(5),[A,B,C]),1,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'hospice-care',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s10-15',1,ref(5),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s10-15',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'hospice-care',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S2: If a patient has a terminal-illness and receives a care then the patient receives an end-of-life-care.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s10-15',2,ref(6),[A,B,C,D,E]),'end-of-life-care',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'terminal-illness',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,care,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,receive), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s10-15',2,ref(6),[A,B,C,D,E]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'terminal-illness',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,care,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,receive), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s10-15',2,ref(7),[A,B,C,D,E]),receive) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'terminal-illness',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,care,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,receive), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s10-15',2,ref(7),[A,B,C,D,E]),1,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'terminal-illness',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,care,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,receive), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s10-15',2,ref(7),[A,B,C,D,E]),2,'$guideline_id'(product,'cdc2022-opioid-s10-15',2,ref(6),[A,B,C,D,E])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'terminal-illness',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,care,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,receive), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
% S3: If a patient has a high-near-term-dying-risk and receives a care then the patient receives an end-of-life-care.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s10-15',3,ref(6),[A,B,C,D,E]),'end-of-life-care',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'high-near-term-dying-risk',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,care,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,receive), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s10-15',3,ref(6),[A,B,C,D,E]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'high-near-term-dying-risk',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,care,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,receive), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s10-15',3,ref(7),[A,B,C,D,E]),receive) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'high-near-term-dying-risk',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,care,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,receive), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s10-15',3,ref(7),[A,B,C,D,E]),1,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'high-near-term-dying-risk',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,care,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,receive), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s10-15',3,ref(7),[A,B,C,D,E]),2,'$guideline_id'(product,'cdc2022-opioid-s10-15',3,ref(6),[A,B,C,D,E])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'high-near-term-dying-risk',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,care,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,receive), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
