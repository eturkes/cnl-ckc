% cdc2022-opioid-rec10-imp04.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec10-imp04',ace_sha256(acbaca25f98cb4e264b1886877020168f3fc7b8f9fd2879f2f66922fbe117f56),ulex(sha256('4c55bed7525c11e80d870254a4f876c23b6aa3c2a077242833ec32997784b01c'))).
% S1: Every clinician should pursue a toxicology-testing-bias-minimization.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp04',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',1,ref(2),[A]),'toxicology-testing-bias-minimization',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',1,ref(3),[A]),pursue) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinical-practice-organization should pursue a toxicology-testing-bias-minimization.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp04',2,box(1),[A]),should) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',2,ref(2),[A]),'toxicology-testing-bias-minimization',countable) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',2,ref(3),[A]),pursue) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',2,ref(2),[A])) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every health-system should pursue a toxicology-testing-bias-minimization.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp04',3,box(1),[A]),should) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',3,ref(2),[A]),'toxicology-testing-bias-minimization',countable) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',3,ref(3),[A]),pursue) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',3,ref(2),[A])) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician should avoid an assumption-based-differential-toxicology-testing-application for a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp04',4,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',4,ref(2),[A]),'assumption-based-differential-toxicology-testing-application',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',4,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',4,ref(4),[A]),avoid) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',4,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every clinical-practice-organization should avoid an assumption-based-differential-toxicology-testing-application for a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp04',5,box(1),[A]),should) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',5,ref(2),[A]),'assumption-based-differential-toxicology-testing-application',countable) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',5,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',5,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',5,ref(4),[A]),avoid) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',5,ref(4),[A]),1,A) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',5,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',5,ref(2),[A])) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',5,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',5,ref(3),[A])) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1).
% S6: Every health-system should avoid an assumption-based-differential-toxicology-testing-application for a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp04',6,box(1),[A]),should) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',6,ref(2),[A]),'assumption-based-differential-toxicology-testing-application',countable) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',6,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',6,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',6,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',6,ref(4),[A]),avoid) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',6,ref(4),[A]),1,A) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',6,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',6,ref(2),[A])) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec10-imp04',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',6,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec10-imp04',6,ref(3),[A])) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
