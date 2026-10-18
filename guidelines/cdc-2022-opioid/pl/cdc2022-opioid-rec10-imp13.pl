% cdc2022-opioid-rec10-imp13.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
:- multifile(guideline_grade/5).
:- discontiguous(guideline_grade/5).
:- multifile(guideline_preference/4).
:- discontiguous(guideline_preference/4).
:- multifile(guideline_qualifier/4).
:- discontiguous(guideline_qualifier/4).
:- multifile(guideline_unit/3).
:- discontiguous(guideline_unit/3).
guideline_schema_version(1).
guideline_document('cdc2022-opioid-rec10-imp13',ace_sha256('49291f97c3524fd50aa30c938d3c0ff9d81b6428de94e85ca319e7c8a74faad4'),ulex(sha256(d08a8bd3e80ef6e01301a079bb5496315f35233bf199aa89b2d2ccc93b858776)),vocabulary(sha256('9d7632637554f8cbfe72e7d34a9b9d14c29eb8593aebb1c128a870c5dcf77ead'))).
% S1: If a patient has an unexplained-unexpected-toxicology-screening-result then a same-sample-selective-confirmatory-test possibly can be warranted for the patient.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',1,ref(4),[A,B,C]),'same-sample-selective-confirmatory-test',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'unexplained-unexpected-toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'unexplained-unexpected-toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp13',1,box(1),[A,B,C]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'unexplained-unexpected-toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-rec10-imp13',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',1,ref(6),[A,B,C]),possibility,possibly) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'unexplained-unexpected-toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_property('$guideline_id'(context,'cdc2022-opioid-rec10-imp13',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',1,ref(5),[A,B,C]),warranted,pos) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'unexplained-unexpected-toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp13',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',1,ref(6),[A,B,C]),be) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'unexplained-unexpected-toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp13',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',1,ref(6),[A,B,C]),1,'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'unexplained-unexpected-toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp13',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',1,ref(6),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',1,ref(5),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'unexplained-unexpected-toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-rec10-imp13',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',1,ref(6),[A,B,C]),applicability,warranted) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'unexplained-unexpected-toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec10-imp13',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp13',1,ref(6),[A,B,C]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'unexplained-unexpected-toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
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
