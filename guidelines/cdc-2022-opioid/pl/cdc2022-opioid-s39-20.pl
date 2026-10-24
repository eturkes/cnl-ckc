% cdc2022-opioid-s39-20.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s39-20',ace_sha256(ef2732cc91386f18a0ce2ac3a09df612100cd694f199c8cd5fecb798fe0a748b),ulex(sha256('64c086862c2f48d491a9fde0ba9f0fcf090e25e84456b9749332ec3ace211ce4')),vocabulary(sha256(c36289aa40f75805a83501ca057b29dd1ba7292b025716853f654890b77ec50c))).
% S1: If a patient has an abrupt-heroin-or-methadone-discontinuation then a clonidine outperforms a placebo in a withdrawal-severity-reduction.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',1,ref(4),[A,B,C]),clonidine,countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'abrupt-heroin-or-methadone-discontinuation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'abrupt-heroin-or-methadone-discontinuation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',1,ref(5),[A,B,C]),placebo,countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'abrupt-heroin-or-methadone-discontinuation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',1,ref(5),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'abrupt-heroin-or-methadone-discontinuation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',1,ref(6),[A,B,C]),'withdrawal-severity-reduction',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'abrupt-heroin-or-methadone-discontinuation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',1,ref(6),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'abrupt-heroin-or-methadone-discontinuation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',1,ref(7),[A,B,C]),outperform) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'abrupt-heroin-or-methadone-discontinuation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',1,ref(7),[A,B,C]),1,'$guideline_id'(product,'cdc2022-opioid-s39-20',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'abrupt-heroin-or-methadone-discontinuation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',1,ref(7),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s39-20',1,ref(5),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'abrupt-heroin-or-methadone-discontinuation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',1,ref(7),[A,B,C]),in,'$guideline_id'(product,'cdc2022-opioid-s39-20',1,ref(6),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'abrupt-heroin-or-methadone-discontinuation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S2: If a patient has an abrupt-heroin-or-methadone-discontinuation then a lofexidine outperforms a placebo in a withdrawal-severity-reduction.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',2,ref(4),[A,B,C]),lofexidine,countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'abrupt-heroin-or-methadone-discontinuation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',2,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'abrupt-heroin-or-methadone-discontinuation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',2,ref(5),[A,B,C]),placebo,countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'abrupt-heroin-or-methadone-discontinuation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',2,ref(5),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'abrupt-heroin-or-methadone-discontinuation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',2,ref(6),[A,B,C]),'withdrawal-severity-reduction',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'abrupt-heroin-or-methadone-discontinuation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',2,ref(6),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'abrupt-heroin-or-methadone-discontinuation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',2,ref(7),[A,B,C]),outperform) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'abrupt-heroin-or-methadone-discontinuation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',2,ref(7),[A,B,C]),1,'$guideline_id'(product,'cdc2022-opioid-s39-20',2,ref(4),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'abrupt-heroin-or-methadone-discontinuation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',2,ref(7),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s39-20',2,ref(5),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'abrupt-heroin-or-methadone-discontinuation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',2,ref(7),[A,B,C]),in,'$guideline_id'(product,'cdc2022-opioid-s39-20',2,ref(6),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'abrupt-heroin-or-methadone-discontinuation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S3: Every clonidine is an alpha-2-agonist.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',3,ref(2),[A]),'alpha-2-agonist',countable) :- guideline_entity(actual,A,clonidine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clonidine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',3,ref(3),[A]),be) :- guideline_entity(actual,A,clonidine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,clonidine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s39-20',3,ref(2),[A])) :- guideline_entity(actual,A,clonidine,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every lofexidine is an alpha-2-agonist.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',4,ref(2),[A]),'alpha-2-agonist',countable) :- guideline_entity(actual,A,lofexidine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,lofexidine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',4,ref(3),[A]),be) :- guideline_entity(actual,A,lofexidine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',4,ref(3),[A]),1,A) :- guideline_entity(actual,A,lofexidine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s39-20',4,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s39-20',4,ref(2),[A])) :- guideline_entity(actual,A,lofexidine,countable), guideline_cardinality(actual,A,na,eq,1).
