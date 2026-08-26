% cdc2022-opioid-rec10-imp06.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec10-imp06',ace_sha256(e4b323770bd9afec213d3e66323fa58f76316aa34dceba8702371fb5ffa51d66),ulex(sha256('4c55bed7525c11e80d870254a4f876c23b6aa3c2a077242833ec32997784b01c'))).
% S1: Every clinician should explain a toxicology-testing-nondismissal-assurance to a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp06',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp06',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp06',1,ref(2),[A]),'toxicology-testing-nondismissal-assurance',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp06',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp06',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp06',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp06',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp06',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp06',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp06',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp06',1,ref(4),[A]),explain) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp06',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp06',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp06',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp06',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp06',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec10-imp06',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp06',1,ref(4),[A]),to,'$guideline_id'(product,'cdc2022-opioid-rec10-imp06',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should explain a toxicology-testing-patient-safety-purpose to a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp06',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp06',2,ref(2),[A]),'toxicology-testing-patient-safety-purpose',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp06',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp06',2,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp06',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp06',2,ref(4),[A]),explain) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp06',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp06',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp06',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec10-imp06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp06',2,ref(4),[A]),to,'$guideline_id'(product,'cdc2022-opioid-rec10-imp06',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
