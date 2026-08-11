% cdc2022-opioid-rec05-imp21.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec05-imp21',ace_sha256('0efd081fec7c2db9981dab797d36a11888cfde7a3790d38c285d1f9e97a766c1'),ulex(sha256(bac3441c3cba94d1d496ba3b04390eae38a0826cd30a0a0f713112b2172476ce))).
% S1: Every clinician should screen an anxiety and should screen a depression during an opioid-taper.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp21',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',1,ref(2),[A]),anxiety,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',1,ref(3),[A]),screen) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp21',1,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',1,ref(4),[A]),depression,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',1,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',1,ref(5),[A]),'opioid-taper',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',1,ref(5),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',1,ref(6),[A]),screen) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',1,ref(6),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',1,ref(6),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',1,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',1,ref(6),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',1,ref(5),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should screen an opioid-misuse and should screen an opioid-use-disorder during an opioid-taper.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp21',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',2,ref(2),[A]),'opioid-misuse',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',2,ref(3),[A]),screen) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp21',2,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',2,ref(4),[A]),'opioid-use-disorder',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',2,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',2,ref(5),[A]),'opioid-taper',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',2,ref(5),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',2,ref(6),[A]),screen) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',2,ref(6),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',2,ref(6),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',2,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',2,ref(6),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',2,ref(5),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should manage a taper-emergent-comorbidity and should provide a treatment.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp21',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',3,ref(2),[A]),'taper-emergent-comorbidity',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',3,ref(3),[A]),manage) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp21',3,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',3,ref(4),[A]),treatment,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',3,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',3,ref(5),[A]),provide) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',3,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp21',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',3,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp21',3,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
