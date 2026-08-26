% cdc2022-opioid-rec02-imp10.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec02-imp10',ace_sha256('97da0e85f7e622b048c265d2b9d67662082d987f01ae10af1b1288220f20ade2'),ulex(sha256('4c55bed7525c11e80d870254a4f876c23b6aa3c2a077242833ec32997784b01c'))).
% S1: Every clinician can consider a tricyclic-antidepressant for a neuropathic-pain.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp10',1,box(1),[A]),can) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',1,ref(2),[A]),'tricyclic-antidepressant',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',1,ref(3),[A]),'neuropathic-pain',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',1,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',1,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician can consider a tetracyclic-antidepressant for a neuropathic-pain.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp10',2,box(1),[A]),can) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',2,ref(2),[A]),'tetracyclic-antidepressant',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',2,ref(3),[A]),'neuropathic-pain',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',2,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',2,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician can consider an SNRI-antidepressant for a neuropathic-pain.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp10',3,box(1),[A]),can) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',3,ref(2),[A]),'SNRI-antidepressant',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',3,ref(3),[A]),'neuropathic-pain',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',3,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',3,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician can consider a selected-anticonvulsant for a neuropathic-pain.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp10',4,box(1),[A]),can) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',4,ref(2),[A]),'selected-anticonvulsant',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',4,ref(3),[A]),'neuropathic-pain',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',4,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',4,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every clinician can consider a capsaicin-patch for a neuropathic-pain.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp10',5,box(1),[A]),can) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',5,ref(2),[A]),'capsaicin-patch',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',5,ref(3),[A]),'neuropathic-pain',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',5,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',5,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',5,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',5,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',5,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',5,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',5,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S6: Every clinician can consider a lidocaine-patch for a neuropathic-pain.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp10',6,box(1),[A]),can) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',6,ref(2),[A]),'lidocaine-patch',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',6,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',6,ref(3),[A]),'neuropathic-pain',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',6,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',6,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',6,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',6,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',6,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',6,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',6,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
