% cdc2022-opioid-s28-10.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s28-10',ace_sha256(d4e0a3408c1850d33c4641b7644ecc7686f07455cfd86c1ef97900a7be098cac),ulex(sha256('4a882a1996ea910df0fbf91fea897c0805e67d48f9e1ce2fcdbbbd9250c193ae'))).
% S1: Every clinician may assess a patient-function with a PEG-assessment-scale.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s28-10',1,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s28-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',1,ref(2),[A]),'patient-function',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s28-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s28-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',1,ref(3),[A]),'PEG-assessment-scale',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s28-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s28-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',1,ref(4),[A]),assess) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s28-10',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s28-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',1,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s28-10',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician may assess a pain-severity with a PEG-assessment-scale.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s28-10',2,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s28-10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',2,ref(2),[A]),'pain-severity',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s28-10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s28-10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',2,ref(3),[A]),'PEG-assessment-scale',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s28-10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s28-10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',2,ref(4),[A]),assess) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s28-10',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s28-10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',2,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s28-10',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician may assess a quality-of-life with a PEG-assessment-scale.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s28-10',3,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s28-10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',3,ref(2),[A]),'quality-of-life',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s28-10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s28-10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',3,ref(3),[A]),'PEG-assessment-scale',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s28-10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s28-10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',3,ref(4),[A]),assess) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s28-10',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s28-10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',3,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s28-10',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician may monitor a patient-function with a PEG-assessment-scale.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s28-10',4,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s28-10',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',4,ref(2),[A]),'patient-function',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s28-10',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s28-10',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',4,ref(3),[A]),'PEG-assessment-scale',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s28-10',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s28-10',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',4,ref(4),[A]),monitor) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-10',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-10',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s28-10',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s28-10',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',4,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s28-10',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every clinician may monitor a pain-severity with a PEG-assessment-scale.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s28-10',5,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s28-10',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',5,ref(2),[A]),'pain-severity',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s28-10',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s28-10',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',5,ref(3),[A]),'PEG-assessment-scale',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s28-10',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',5,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s28-10',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',5,ref(4),[A]),monitor) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-10',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',5,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-10',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',5,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s28-10',5,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s28-10',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',5,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s28-10',5,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S6: Every clinician may monitor a quality-of-life with a PEG-assessment-scale.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s28-10',6,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s28-10',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',6,ref(2),[A]),'quality-of-life',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s28-10',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',6,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s28-10',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',6,ref(3),[A]),'PEG-assessment-scale',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s28-10',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',6,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s28-10',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',6,ref(4),[A]),monitor) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-10',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',6,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-10',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',6,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s28-10',6,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s28-10',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',6,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s28-10',6,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
