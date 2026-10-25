% cdc2022-opioid-s39-24.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s39-24',ace_sha256(fdb448aff00cce72841d2d5023e73ec9af7f51759b37175ce8dcf5666198d333),ulex(sha256('64c086862c2f48d491a9fde0ba9f0fcf090e25e84456b9749332ec3ace211ce4')),vocabulary(sha256(c36289aa40f75805a83501ca057b29dd1ba7292b025716853f654890b77ec50c))).
% S1: Every buprenorphine carries a less-than-full-agonist-overdose-likelihood.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s39-24',1,ref(2),[A]),'less-than-full-agonist-overdose-likelihood',countable) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s39-24',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s39-24',1,ref(3),[A]),carry) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s39-24',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s39-24',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s39-24',1,ref(2),[A])) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: If a patient takes a buprenorphine then a buprenorphine-overdose can possibly occur for the patient.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s39-24',2,ref(4),[A,B,C]),'buprenorphine-overdose',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,buprenorphine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,take), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s39-24',2,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,buprenorphine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,take), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s39-24',2,box(1),[A,B,C]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,buprenorphine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,take), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s39-24',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s39-24',2,ref(5),[A,B,C]),possibility,possibly) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,buprenorphine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,take), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s39-24',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s39-24',2,ref(5),[A,B,C]),occur) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,buprenorphine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,take), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-24',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s39-24',2,ref(5),[A,B,C]),1,'$guideline_id'(product,'cdc2022-opioid-s39-24',2,ref(4),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,buprenorphine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,take), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s39-24',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s39-24',2,ref(5),[A,B,C]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,buprenorphine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,take), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S3: If a patient concurrently takes a buprenorphine and concurrently takes an other-respiratory-depressant then a buprenorphine-overdose can particularly occur for the patient.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s39-24',3,ref(6),[A,B,C,D,E]),'buprenorphine-overdose',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,buprenorphine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,take), guideline_qualifier(actual,C,manner,concurrently), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'other-respiratory-depressant',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,take), guideline_qualifier(actual,E,manner,concurrently), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s39-24',3,ref(6),[A,B,C,D,E]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,buprenorphine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,take), guideline_qualifier(actual,C,manner,concurrently), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'other-respiratory-depressant',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,take), guideline_qualifier(actual,E,manner,concurrently), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s39-24',3,box(1),[A,B,C,D,E]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,buprenorphine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,take), guideline_qualifier(actual,C,manner,concurrently), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'other-respiratory-depressant',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,take), guideline_qualifier(actual,E,manner,concurrently), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s39-24',3,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s39-24',3,ref(7),[A,B,C,D,E]),focus,particularly) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,buprenorphine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,take), guideline_qualifier(actual,C,manner,concurrently), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'other-respiratory-depressant',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,take), guideline_qualifier(actual,E,manner,concurrently), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s39-24',3,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s39-24',3,ref(7),[A,B,C,D,E]),occur) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,buprenorphine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,take), guideline_qualifier(actual,C,manner,concurrently), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'other-respiratory-depressant',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,take), guideline_qualifier(actual,E,manner,concurrently), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-24',3,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s39-24',3,ref(7),[A,B,C,D,E]),1,'$guideline_id'(product,'cdc2022-opioid-s39-24',3,ref(6),[A,B,C,D,E])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,buprenorphine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,take), guideline_qualifier(actual,C,manner,concurrently), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'other-respiratory-depressant',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,take), guideline_qualifier(actual,E,manner,concurrently), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s39-24',3,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s39-24',3,ref(7),[A,B,C,D,E]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,buprenorphine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,take), guideline_qualifier(actual,C,manner,concurrently), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'other-respiratory-depressant',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,take), guideline_qualifier(actual,E,manner,concurrently), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
% S4: An other-respiratory-depressant-example includes a full-agonist-opioid-example and includes a benzodiazepine-example and includes an alcohol-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s39-24',4,ref(1),[]),'other-respiratory-depressant-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s39-24',4,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s39-24',4,ref(2),[]),'full-agonist-opioid-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s39-24',4,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s39-24',4,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s39-24',4,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s39-24',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s39-24',4,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s39-24',4,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s39-24',4,ref(4),[]),'benzodiazepine-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s39-24',4,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s39-24',4,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s39-24',4,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s39-24',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s39-24',4,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s39-24',4,ref(4),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s39-24',4,ref(6),[]),'alcohol-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s39-24',4,ref(6),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s39-24',4,ref(7),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s39-24',4,ref(7),[]),1,'$guideline_id'(product,'cdc2022-opioid-s39-24',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s39-24',4,ref(7),[]),2,'$guideline_id'(product,'cdc2022-opioid-s39-24',4,ref(6),[])).
