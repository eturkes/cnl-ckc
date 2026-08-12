% cdc2022-opioid-s23-13.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s23-13',ace_sha256('11b474196848851ab8c7b4a12a0226c4f1b5591b8bc99e6f7ae21395da3cc35e'),ulex(sha256('2a4a3e829c919b3dd3b20356e7a69585cfb305c3c57dfb531e3f2c1eb97637b6'))).
% S1: Every clinician should review an opioid-benzodiazepine-respiratory-depression-risk with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-13',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-13',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',1,ref(2),[A]),'opioid-benzodiazepine-respiratory-depression-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-13',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-13',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-13',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-13',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',1,ref(4),[A]),review) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-13',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-13',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-13',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-13',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',1,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-13',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should review an opioid-other-sedative-respiratory-depression-risk with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-13',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-13',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',2,ref(2),[A]),'opioid-other-sedative-respiratory-depression-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-13',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-13',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',2,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-13',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-13',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',2,ref(4),[A]),review) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-13',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-13',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-13',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-13',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',2,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-13',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should review an opioid-alcohol-respiratory-depression-risk with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-13',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-13',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',3,ref(2),[A]),'opioid-alcohol-respiratory-depression-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-13',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-13',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',3,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-13',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-13',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',3,ref(4),[A]),review) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-13',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-13',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-13',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-13',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',3,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-13',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician should review an opioid-nonprescribed-or-illicit-drug-respiratory-depression-risk with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-13',4,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-13',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',4,ref(2),[A]),'opioid-nonprescribed-or-illicit-drug-respiratory-depression-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-13',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-13',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',4,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-13',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-13',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',4,ref(4),[A]),review) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-13',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-13',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-13',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-13',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',4,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-13',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every clinician should review an opioid-other-opioid-respiratory-depression-risk with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-13',5,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-13',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',5,ref(2),[A]),'opioid-other-opioid-respiratory-depression-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-13',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-13',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',5,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-13',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',5,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-13',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',5,ref(4),[A]),review) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-13',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',5,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-13',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',5,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-13',5,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-13',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-13',5,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-13',5,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
