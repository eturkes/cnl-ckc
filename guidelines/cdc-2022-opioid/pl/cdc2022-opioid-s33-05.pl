% cdc2022-opioid-s33-05.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s33-05',ace_sha256(e6f6ec7a6415cfcce825430de29e48b17830cc36d9f095c0a62f70947fe13f95),ulex(sha256('6bd0df91739d5a42b1a472d866e0dd72897517f8e4652458e4893adbed9cc663'))).
% S1: Every clinician should heed a particular-caution during a transdermal-fentanyl-prescription.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s33-05',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s33-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-05',1,ref(2),[A]),'particular-caution',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s33-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-05',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s33-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-05',1,ref(3),[A]),'transdermal-fentanyl-prescription',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s33-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-05',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s33-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-05',1,ref(4),[A]),heed) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s33-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-05',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s33-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-05',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s33-05',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s33-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-05',1,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s33-05',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should consider a transdermal-fentanyl-dosing during a transdermal-fentanyl-prescription.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s33-05',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s33-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-05',2,ref(2),[A]),'transdermal-fentanyl-dosing',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s33-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-05',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s33-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-05',2,ref(3),[A]),'transdermal-fentanyl-prescription',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s33-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-05',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s33-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-05',2,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s33-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-05',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s33-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-05',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s33-05',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s33-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-05',2,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s33-05',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should consider a transdermal-fentanyl-absorption during a transdermal-fentanyl-prescription.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s33-05',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s33-05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-05',3,ref(2),[A]),'transdermal-fentanyl-absorption',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s33-05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-05',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s33-05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-05',3,ref(3),[A]),'transdermal-fentanyl-prescription',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s33-05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-05',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s33-05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-05',3,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s33-05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-05',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s33-05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-05',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s33-05',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s33-05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-05',3,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s33-05',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
