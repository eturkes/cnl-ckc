% cdc2022-opioid-rec08-imp04.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec08-imp04',ace_sha256('9d40952af579a37ea2a1f8b71511640d50cadfc5db3e3a3e9cb9124dec4587dc'),ulex(sha256('4c55bed7525c11e80d870254a4f876c23b6aa3c2a077242833ec32997784b01c'))).
% S1: Every clinical-practice-organization should provide an overdose-prevention-education to a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec08-imp04',1,box(1),[A]),should) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',1,ref(2),[A]),'overdose-prevention-education',countable) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',1,ref(4),[A]),provide) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',1,ref(2),[A])) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',1,ref(4),[A]),3,'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',1,ref(3),[A])) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinical-practice-organization should provide a naloxone-use-education to a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec08-imp04',2,box(1),[A]),should) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',2,ref(2),[A]),'naloxone-use-education',countable) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',2,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',2,ref(4),[A]),provide) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',2,ref(2),[A])) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',2,ref(4),[A]),3,'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',2,ref(3),[A])) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinical-practice-organization should offer an overdose-prevention-education to a patient-household-member.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec08-imp04',3,box(1),[A]),should) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',3,ref(2),[A]),'overdose-prevention-education',countable) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',3,ref(3),[A]),'patient-household-member',countable) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',3,ref(4),[A]),offer) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',3,ref(2),[A])) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',3,ref(4),[A]),3,'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',3,ref(3),[A])) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinical-practice-organization should offer a naloxone-use-education to a patient-household-member.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec08-imp04',4,box(1),[A]),should) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',4,ref(2),[A]),'naloxone-use-education',countable) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',4,ref(3),[A]),'patient-household-member',countable) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',4,ref(4),[A]),offer) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',4,ref(2),[A])) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp04',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',4,ref(4),[A]),3,'$guideline_id'(product,'cdc2022-opioid-rec08-imp04',4,ref(3),[A])) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
