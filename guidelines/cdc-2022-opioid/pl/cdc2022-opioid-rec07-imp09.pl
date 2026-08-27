% cdc2022-opioid-rec07-imp09.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec07-imp09',ace_sha256(ab071c6b4bf0e5d8d5efefe90768e1bda56fa71b85cfc5aa2d51b3813438a5ae),ulex(sha256('4a882a1996ea910df0fbf91fea897c0805e67d48f9e1ce2fcdbbbd9250c193ae'))).
% S1: If a patient has a treatment-course then every clinician should conduct a regular-opioid-risk-screening for the patient during the treatment-course.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec07-imp09',1,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'treatment-course',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec07-imp09',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',1,ref(5),[A,B,C,D]),'regular-opioid-risk-screening',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'treatment-course',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec07-imp09',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',1,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'treatment-course',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec07-imp09',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',1,ref(6),[A,B,C,D]),conduct) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'treatment-course',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec07-imp09',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',1,ref(6),[A,B,C,D]),1,D) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'treatment-course',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec07-imp09',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',1,ref(6),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',1,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'treatment-course',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec07-imp09',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',1,ref(6),[A,B,C,D]),during,B) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'treatment-course',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec07-imp09',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',1,ref(6),[A,B,C,D]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'treatment-course',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
% S2: Every opioid-risk-condition can change during a treatment-course.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec07-imp09',2,box(1),[A]),can) :- guideline_entity(actual,A,'opioid-risk-condition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec07-imp09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',2,ref(2),[A]),'treatment-course',countable) :- guideline_entity(actual,A,'opioid-risk-condition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec07-imp09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-risk-condition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec07-imp09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',2,ref(3),[A]),change) :- guideline_entity(actual,A,'opioid-risk-condition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec07-imp09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-risk-condition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec07-imp09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',2,ref(3),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',2,ref(2),[A])) :- guideline_entity(actual,A,'opioid-risk-condition',countable), guideline_cardinality(actual,A,na,eq,1).
