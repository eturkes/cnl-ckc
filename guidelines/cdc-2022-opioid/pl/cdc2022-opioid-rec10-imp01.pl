% cdc2022-opioid-rec10-imp01.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec10-imp01',ace_sha256(a95b1ab359718ca2d67d9b2c4d66c504871f76fbf566f6bfd0360ce122a8b533),ulex(sha256('4c55bed7525c11e80d870254a4f876c23b6aa3c2a077242833ec32997784b01c'))).
% S1: Every clinician should not use a toxicology-testing for a punitive-purpose.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp01',1,box(1),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',1,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec10-imp01',1,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',1,ref(2),[A]),'toxicology-testing',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',1,ref(3),[A]),'punitive-purpose',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',1,ref(4),[A]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',1,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should use a toxicology-testing during a clinical-decision.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp01',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',2,ref(2),[A]),'toxicology-testing',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',2,ref(3),[A]),'clinical-decision',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',2,ref(4),[A]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',2,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should consider an other-clinical-information during a clinical-decision.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp01',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',3,ref(2),[A]),'other-clinical-information',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',3,ref(3),[A]),'clinical-decision',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',3,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',3,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician should use a toxicology-testing during a patient-care-improvement.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp01',4,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',4,ref(2),[A]),'toxicology-testing',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',4,ref(3),[A]),'patient-care-improvement',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',4,ref(4),[A]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',4,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every clinician should consider an other-clinical-information during a patient-care-improvement.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp01',5,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',5,ref(2),[A]),'other-clinical-information',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',5,ref(3),[A]),'patient-care-improvement',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',5,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',5,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',5,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',5,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',5,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',5,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',5,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
