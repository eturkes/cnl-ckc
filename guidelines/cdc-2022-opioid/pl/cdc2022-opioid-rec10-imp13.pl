% cdc2022-opioid-rec10-imp13.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec10-imp13',ace_sha256(f118cec2d1cf3933e8cb6353499f76708026d54a5be8cec759597bafd186147d),ulex(sha256('4c55bed7525c11e80d870254a4f876c23b6aa3c2a077242833ec32997784b01c'))).
% S1: If a patient has an unexplained-unexpected-toxicology-screening-result then every clinician may use a same-sample-selective-confirmatory-test for the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp13',1,box(1),[A,B,C,D]),may) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'unexplained-unexpected-toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp13',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',1,ref(5),[A,B,C,D]),'same-sample-selective-confirmatory-test',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'unexplained-unexpected-toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp13',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',1,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'unexplained-unexpected-toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp13',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',1,ref(6),[A,B,C,D]),use) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'unexplained-unexpected-toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp13',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',1,ref(6),[A,B,C,D]),1,D) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'unexplained-unexpected-toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp13',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',1,ref(6),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',1,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'unexplained-unexpected-toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec10-imp13',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',1,ref(6),[A,B,C,D]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'unexplained-unexpected-toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
% S2: A same-sample-selective-confirmatory-test can differentiate a specific-opioid and can differentiate an opioid-metabolite.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',2,ref(1),[]),'same-sample-selective-confirmatory-test',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',2,ref(1),[]),na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp13',2,box(1),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp13',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',2,ref(2),[]),'specific-opioid',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp13',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',2,ref(2),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp13',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',2,ref(3),[]),differentiate).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp13',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',2,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',2,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp13',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',2,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',2,ref(2),[])).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp13',2,box(2),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp13',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',2,ref(4),[]),'opioid-metabolite',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp13',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',2,ref(4),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp13',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',2,ref(5),[]),differentiate).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp13',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',2,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',2,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp13',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',2,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',2,ref(4),[])).
