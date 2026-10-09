% cdc2022-opioid-s33-05.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
:- multifile(guideline_schema_version/1).
:- discontiguous(guideline_schema_version/1).
:- multifile(guideline_document/4).
:- discontiguous(guideline_document/4).
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
:- multifile(guideline_interval/6).
:- discontiguous(guideline_interval/6).
:- multifile(guideline_recurrence/4).
:- discontiguous(guideline_recurrence/4).
:- multifile(guideline_frequency/5).
:- discontiguous(guideline_frequency/5).
:- multifile(guideline_order/4).
:- discontiguous(guideline_order/4).
:- multifile(guideline_recurrence_window/7).
:- discontiguous(guideline_recurrence_window/7).
:- multifile(guideline_range/3).
:- discontiguous(guideline_range/3).
guideline_schema_version(3).
guideline_document('cdc2022-opioid-s33-05',ace_sha256(e6f6ec7a6415cfcce825430de29e48b17830cc36d9f095c0a62f70947fe13f95),ulex(sha256('0b669018d532f506c518a6b0edffa4994f771b930477e7e2b36faf50d2b75d33')),temporal(sha256(d7b0d6c9af49f8d4b3dc12370d6b1cc20e923a69a5ce745ee26a1bc0f21effb6))).
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
