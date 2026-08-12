% cdc2022-opioid-s23-01.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s23-01',ace_sha256('3f022cdc29e0f0ce711a12a6aba8dbb15a56d045d67227da273cf5d64f9ec02e'),ulex(sha256('2a4a3e829c919b3dd3b20356e7a69585cfb305c3c57dfb531e3f2c1eb97637b6'))).
% S1: Every clinician should review a patient-severe-or-uncontrolled-pain-reporting-mechanism and should review a timely-pain-reassessment-and-management-protocol.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-01',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-01',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-01',1,ref(2),[A]),'patient-severe-or-uncontrolled-pain-reporting-mechanism',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-01',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-01',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-01',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-01',1,ref(3),[A]),review) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-01',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-01',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-01',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-01',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-01',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-01',1,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-01',1,ref(4),[A]),'timely-pain-reassessment-and-management-protocol',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-01',1,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-01',1,ref(5),[A]),review) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-01',1,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-01',1,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-01',1,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
