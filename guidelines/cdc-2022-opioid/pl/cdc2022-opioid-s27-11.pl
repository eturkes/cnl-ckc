% cdc2022-opioid-s27-11.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_schema_version(1).
guideline_document('cdc2022-opioid-s27-11',ace_sha256('01157906d62f438026525777e64fc30391e039ada891543b7228d700f46f9392'),ulex(sha256(ebe2d3db8559958f9ed44ec7a76cb2fb6ea75017f9f9437c3ea03b07092b0e75)),vocabulary(sha256('9d7632637554f8cbfe72e7d34a9b9d14c29eb8593aebb1c128a870c5dcf77ead'))).
% S1: If a patient has a chronic-pain then the patient can possibly experience a concurrent-depression.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s27-11',1,box(1),[A,B,C]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'chronic-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s27-11',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s27-11',1,ref(4),[A,B,C]),'concurrent-depression',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'chronic-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s27-11',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s27-11',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'chronic-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s27-11',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s27-11',1,ref(5),[A,B,C]),possibility,possibly) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'chronic-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s27-11',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s27-11',1,ref(5),[A,B,C]),experience) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'chronic-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s27-11',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s27-11',1,ref(5),[A,B,C]),1,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'chronic-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s27-11',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s27-11',1,ref(5),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s27-11',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'chronic-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S2: Every depression can exacerbate a physical-symptom and can exacerbate a pain.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s27-11',2,box(1),[A]),can) :- guideline_entity(actual,A,depression,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s27-11',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s27-11',2,ref(2),[A]),'physical-symptom',countable) :- guideline_entity(actual,A,depression,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s27-11',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s27-11',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,depression,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s27-11',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s27-11',2,ref(3),[A]),exacerbate) :- guideline_entity(actual,A,depression,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s27-11',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s27-11',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,depression,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s27-11',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s27-11',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s27-11',2,ref(2),[A])) :- guideline_entity(actual,A,depression,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s27-11',2,box(2),[A]),can) :- guideline_entity(actual,A,depression,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s27-11',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s27-11',2,ref(4),[A]),pain,countable) :- guideline_entity(actual,A,depression,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s27-11',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s27-11',2,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,depression,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s27-11',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s27-11',2,ref(5),[A]),exacerbate) :- guideline_entity(actual,A,depression,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s27-11',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s27-11',2,ref(5),[A]),1,A) :- guideline_entity(actual,A,depression,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s27-11',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s27-11',2,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s27-11',2,ref(4),[A])) :- guideline_entity(actual,A,depression,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: If a patient has a pain and has a depression then an antidepressant can especially and likely benefit the patient.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s27-11',3,ref(6),[A,B,C,D,E]),antidepressant,countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,depression,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s27-11',3,ref(6),[A,B,C,D,E]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,depression,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s27-11',3,box(1),[A,B,C,D,E]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,depression,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s27-11',3,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s27-11',3,ref(7),[A,B,C,D,E]),possibility,likely) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,depression,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s27-11',3,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s27-11',3,ref(7),[A,B,C,D,E]),focus,especially) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,depression,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s27-11',3,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s27-11',3,ref(7),[A,B,C,D,E]),benefit) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,depression,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s27-11',3,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s27-11',3,ref(7),[A,B,C,D,E]),1,'$guideline_id'(product,'cdc2022-opioid-s27-11',3,ref(6),[A,B,C,D,E])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,depression,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s27-11',3,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s27-11',3,ref(7),[A,B,C,D,E]),2,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,depression,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
