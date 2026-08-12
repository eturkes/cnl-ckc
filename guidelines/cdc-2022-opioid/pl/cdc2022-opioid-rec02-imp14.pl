% cdc2022-opioid-rec02-imp14.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec02-imp14',ace_sha256(d31b6554b941607fe2382363d720f5ddfb7b7836d84e78f8f3ce529048d225e9),ulex(sha256(e099ebb206cfb62cc396438935bd812a79998ee738ec876290f34a554836093e))).
% S1: Every clinician should not consider a first-line-opioid-therapy for a subacute-pain.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp14',1,box(1),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',1,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec02-imp14',1,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',1,ref(2),[A]),'first-line-opioid-therapy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',1,ref(3),[A]),'subacute-pain',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',1,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',1,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should not consider a routine-opioid-therapy for a subacute-pain.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp14',2,box(1),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',2,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec02-imp14',2,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',2,ref(2),[A]),'routine-opioid-therapy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',2,ref(3),[A]),'subacute-pain',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',2,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',2,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should not consider a first-line-opioid-therapy for a chronic-pain.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp14',3,box(1),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',3,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec02-imp14',3,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',3,ref(2),[A]),'first-line-opioid-therapy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',3,ref(3),[A]),'chronic-pain',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',3,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',3,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician should not consider a routine-opioid-therapy for a chronic-pain.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp14',4,box(1),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',4,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec02-imp14',4,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',4,ref(2),[A]),'routine-opioid-therapy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',4,ref(3),[A]),'chronic-pain',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',4,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec02-imp14',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',4,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec02-imp14',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
