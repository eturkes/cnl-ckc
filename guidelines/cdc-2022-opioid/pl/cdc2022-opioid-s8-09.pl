% cdc2022-opioid-s8-09.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s8-09',ace_sha256('30c4a289151646e231e0a798e1f2fcd331025e96a2340e6dac3ba745b2cdc504'),ulex(sha256('4a882a1996ea910df0fbf91fea897c0805e67d48f9e1ce2fcdbbbd9250c193ae'))).
% S1: Every clinician should collaborate with a patient during a treatment-decision.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s8-09',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s8-09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',1,ref(2),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s8-09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s8-09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',1,ref(3),[A]),'treatment-decision',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s8-09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s8-09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',1,ref(4),[A]),collaborate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s8-09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s8-09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',1,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s8-09',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s8-09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',1,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s8-09',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should collaborate with a patient during a treatment-plan-design.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s8-09',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s8-09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',2,ref(2),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s8-09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s8-09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',2,ref(3),[A]),'treatment-plan-design',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s8-09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s8-09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',2,ref(4),[A]),collaborate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s8-09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s8-09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',2,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s8-09',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s8-09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',2,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s8-09',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should collaborate with a patient during a pain-management-strategy-initiation.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s8-09',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s8-09',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',3,ref(2),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s8-09',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s8-09',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',3,ref(3),[A]),'pain-management-strategy-initiation',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s8-09',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s8-09',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',3,ref(4),[A]),collaborate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s8-09',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s8-09',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',3,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s8-09',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s8-09',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',3,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s8-09',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician should collaborate with a patient during a pain-management-strategy-change.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s8-09',4,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s8-09',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',4,ref(2),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s8-09',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s8-09',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',4,ref(3),[A]),'pain-management-strategy-change',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s8-09',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s8-09',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',4,ref(4),[A]),collaborate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s8-09',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s8-09',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',4,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s8-09',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s8-09',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',4,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s8-09',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every clinician should collaborate with a patient during an opioid-initiation.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s8-09',5,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s8-09',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',5,ref(2),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s8-09',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s8-09',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',5,ref(3),[A]),'opioid-initiation',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s8-09',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',5,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s8-09',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',5,ref(4),[A]),collaborate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s8-09',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',5,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s8-09',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',5,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s8-09',5,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s8-09',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',5,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s8-09',5,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S6: Every clinician should collaborate with a patient during an opioid-dosage-increase.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s8-09',6,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s8-09',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',6,ref(2),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s8-09',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',6,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s8-09',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',6,ref(3),[A]),'opioid-dosage-increase',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s8-09',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',6,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s8-09',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',6,ref(4),[A]),collaborate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s8-09',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',6,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s8-09',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',6,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s8-09',6,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s8-09',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',6,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s8-09',6,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S7: Every clinician should collaborate with a patient during an opioid-taper.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s8-09',7,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s8-09',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',7,ref(2),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s8-09',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',7,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s8-09',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',7,ref(3),[A]),'opioid-taper',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s8-09',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',7,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s8-09',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',7,ref(4),[A]),collaborate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s8-09',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',7,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s8-09',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',7,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s8-09',7,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s8-09',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',7,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s8-09',7,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S8: Every clinician should collaborate with a patient during an opioid-discontinuation.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s8-09',8,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s8-09',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',8,ref(2),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s8-09',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',8,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s8-09',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',8,ref(3),[A]),'opioid-discontinuation',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s8-09',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',8,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s8-09',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',8,ref(4),[A]),collaborate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s8-09',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',8,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s8-09',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',8,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s8-09',8,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s8-09',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-09',8,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s8-09',8,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
