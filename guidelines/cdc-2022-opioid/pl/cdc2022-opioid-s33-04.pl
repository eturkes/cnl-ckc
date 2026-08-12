% cdc2022-opioid-s33-04.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s33-04',ace_sha256(b3e71bd332ee82342bdc92186e57d92a3aa95e0b5f85bc74917c6f796c1e17f2),ulex(sha256('0e8a9a04fd387b8eab12faf03224671330b94c03dff5f0bf39f23a8d14af88bd'))).
% S1: Every clinician should heed a particular-caution during a methadone-dose-conversion.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s33-04',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s33-04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-04',1,ref(2),[A]),'particular-caution',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s33-04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-04',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s33-04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-04',1,ref(3),[A]),'methadone-dose-conversion',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s33-04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-04',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s33-04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-04',1,ref(4),[A]),heed) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s33-04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-04',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s33-04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-04',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s33-04',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s33-04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-04',1,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s33-04',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
