% cdc2022-opioid-s59-07.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
:- multifile(guideline_schema_version/1).
:- discontiguous(guideline_schema_version/1).
:- multifile(guideline_document/3).
:- discontiguous(guideline_document/3).
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
guideline_schema_version(1).
guideline_document('cdc2022-opioid-s59-07',ace_sha256('5814b2788e975bad8c79da45f2ecdcd855b129fdb948c19b13df94f33b2857f3'),ulex(sha256('4c55bed7525c11e80d870254a4f876c23b6aa3c2a077242833ec32997784b01c'))).
% S1: If a clinician discontinues a buprenorphine-treatment for a patient then the clinician should use a very-gradual-buprenorphine-taper for the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s59-07',1,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'buprenorphine-treatment',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,discontinue), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s59-07',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s59-07',1,ref(5),[A,B,C,D]),'very-gradual-buprenorphine-taper',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'buprenorphine-treatment',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,discontinue), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s59-07',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s59-07',1,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'buprenorphine-treatment',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,discontinue), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s59-07',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s59-07',1,ref(6),[A,B,C,D]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'buprenorphine-treatment',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,discontinue), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s59-07',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s59-07',1,ref(6),[A,B,C,D]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'buprenorphine-treatment',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,discontinue), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s59-07',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s59-07',1,ref(6),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-s59-07',1,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'buprenorphine-treatment',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,discontinue), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s59-07',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s59-07',1,ref(6),[A,B,C,D]),for,C) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'buprenorphine-treatment',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,discontinue), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
% S2: Every very-gradual-buprenorphine-taper should continue for more than 1 month.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s59-07',2,box(1),[A]),should) :- guideline_entity(actual,A,'very-gradual-buprenorphine-taper',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s59-07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s59-07',2,ref(2),[A]),month,countable) :- guideline_entity(actual,A,'very-gradual-buprenorphine-taper',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s59-07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s59-07',2,ref(2),[A]),na,greater,1) :- guideline_entity(actual,A,'very-gradual-buprenorphine-taper',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s59-07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s59-07',2,ref(3),[A]),continue) :- guideline_entity(actual,A,'very-gradual-buprenorphine-taper',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s59-07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s59-07',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'very-gradual-buprenorphine-taper',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s59-07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s59-07',2,ref(3),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s59-07',2,ref(2),[A])) :- guideline_entity(actual,A,'very-gradual-buprenorphine-taper',countable), guideline_cardinality(actual,A,na,eq,1).
