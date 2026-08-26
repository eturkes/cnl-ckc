% cdc2022-opioid-rec08-imp07.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec08-imp07',ace_sha256(d7993e37a33b12588a98d8460521a3b60e2b2e07f29fcfb79ddd1d019be17490),ulex(sha256('4c55bed7525c11e80d870254a4f876c23b6aa3c2a077242833ec32997784b01c'))).
% S1: Every clinician may facilitate a naloxone-access for a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec08-imp07',1,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',1,ref(2),[A]),'naloxone-access',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec08-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',1,ref(4),[A]),facilitate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec08-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',1,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every health-system may facilitate a naloxone-access for a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec08-imp07',2,box(1),[A]),may) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08-imp07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',2,ref(2),[A]),'naloxone-access',countable) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08-imp07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08-imp07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',2,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08-imp07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec08-imp07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',2,ref(4),[A]),facilitate) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',2,ref(2),[A])) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec08-imp07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',2,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',2,ref(3),[A])) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every payer may facilitate a naloxone-access for a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec08-imp07',3,box(1),[A]),may) :- guideline_entity(actual,A,payer,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08-imp07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',3,ref(2),[A]),'naloxone-access',countable) :- guideline_entity(actual,A,payer,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08-imp07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,payer,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08-imp07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',3,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,payer,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08-imp07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,payer,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec08-imp07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',3,ref(4),[A]),facilitate) :- guideline_entity(actual,A,payer,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,payer,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',3,ref(2),[A])) :- guideline_entity(actual,A,payer,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec08-imp07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',3,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',3,ref(3),[A])) :- guideline_entity(actual,A,payer,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every naloxone-dose is a potentially-lifesaving-treatment.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',4,ref(2),[A]),'potentially-lifesaving-treatment',countable) :- guideline_entity(actual,A,'naloxone-dose',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'naloxone-dose',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',4,ref(3),[A]),be) :- guideline_entity(actual,A,'naloxone-dose',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',4,ref(3),[A]),1,A) :- guideline_entity(actual,A,'naloxone-dose',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',4,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec08-imp07',4,ref(2),[A])) :- guideline_entity(actual,A,'naloxone-dose',countable), guideline_cardinality(actual,A,na,eq,1).
