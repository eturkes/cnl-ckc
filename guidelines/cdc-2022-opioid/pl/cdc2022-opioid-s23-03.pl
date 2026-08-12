% cdc2022-opioid-s23-03.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s23-03',ace_sha256('47a8470d35f1ef28ca6a89f78a5bb2667f2d1315d8393064cdf90a6e71aa2fc6'),ulex(sha256('06c1c26d6b6abe1436b8c94a276937ad072806062117417de2ab8086fdf9a3e9'))).
% S1: Every clinician should advise a constipation with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-03',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',1,ref(2),[A]),constipation,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',1,ref(4),[A]),advise) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-03',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',1,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-03',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should advise a dry-mouth with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-03',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',2,ref(2),[A]),'dry-mouth',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',2,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',2,ref(4),[A]),advise) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-03',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',2,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-03',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should advise a nausea with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-03',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',3,ref(2),[A]),nausea,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',3,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',3,ref(4),[A]),advise) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-03',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',3,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-03',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician should advise a vomiting with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-03',4,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',4,ref(2),[A]),vomiting,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',4,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',4,ref(4),[A]),advise) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-03',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',4,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-03',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every clinician should advise a drowsiness with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-03',5,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',5,ref(2),[A]),drowsiness,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',5,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',5,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',5,ref(4),[A]),advise) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',5,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',5,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-03',5,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',5,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-03',5,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S6: Every clinician should advise a confusion with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-03',6,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',6,ref(2),[A]),confusion,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',6,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',6,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',6,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-03',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',6,ref(4),[A]),advise) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',6,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',6,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-03',6,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-03',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',6,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-03',6,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S7: Every clinician should advise a tolerance with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-03',7,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',7,ref(2),[A]),tolerance,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',7,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',7,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',7,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-03',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',7,ref(4),[A]),advise) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',7,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',7,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-03',7,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-03',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',7,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-03',7,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S8: Every clinician should advise a physical-dependence with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-03',8,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',8,ref(2),[A]),'physical-dependence',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',8,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',8,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',8,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-03',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',8,ref(4),[A]),advise) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',8,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',8,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-03',8,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-03',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',8,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-03',8,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S9: Every clinician should advise an opioid-stopping-withdrawal-symptom with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-03',9,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',9,ref(2),[A]),'opioid-stopping-withdrawal-symptom',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',9,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',9,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',9,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-03',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',9,ref(4),[A]),advise) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',9,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',9,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-03',9,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-03',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',9,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-03',9,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
