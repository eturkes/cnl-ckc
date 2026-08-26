% cdc2022-opioid-s18-06.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s18-06',ace_sha256(ce7bef5f0494748e2e3e73f9e50949557022dac69936dcd9365578b54034e1cf),ulex(sha256('4c55bed7525c11e80d870254a4f876c23b6aa3c2a077242833ec32997784b01c'))).
% S1: A multimodal-and-multidisciplinary-pain-management-approach is critical.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s18-06',1,ref(1),[]),'multimodal-and-multidisciplinary-pain-management-approach',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s18-06',1,ref(1),[]),na,eq,1).
guideline_property(actual,'$guideline_id'(product,'cdc2022-opioid-s18-06',1,ref(2),[]),critical,pos).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s18-06',1,ref(3),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s18-06',1,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s18-06',1,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s18-06',1,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s18-06',1,ref(2),[])).
% S2: Every clinician should use a multimodal-and-multidisciplinary-pain-management-approach.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s18-06',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s18-06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',2,ref(2),[A]),'multimodal-and-multidisciplinary-pain-management-approach',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s18-06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s18-06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',2,ref(3),[A]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s18-06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s18-06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s18-06',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should consider a physical-health during a multimodal-and-multidisciplinary-pain-management-approach.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s18-06',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s18-06',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',3,ref(2),[A]),'physical-health',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s18-06',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s18-06',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',3,ref(3),[A]),'multimodal-and-multidisciplinary-pain-management-approach',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s18-06',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s18-06',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',3,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s18-06',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s18-06',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s18-06',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s18-06',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',3,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s18-06',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician should consider a behavioral-health during a multimodal-and-multidisciplinary-pain-management-approach.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s18-06',4,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s18-06',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',4,ref(2),[A]),'behavioral-health',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s18-06',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s18-06',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',4,ref(3),[A]),'multimodal-and-multidisciplinary-pain-management-approach',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s18-06',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s18-06',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',4,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s18-06',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s18-06',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s18-06',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s18-06',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',4,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s18-06',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every clinician should consider a long-term-services-and-supports during a multimodal-and-multidisciplinary-pain-management-approach.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s18-06',5,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s18-06',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',5,ref(2),[A]),'long-term-services-and-supports',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s18-06',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s18-06',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',5,ref(3),[A]),'multimodal-and-multidisciplinary-pain-management-approach',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s18-06',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',5,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s18-06',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',5,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s18-06',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',5,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s18-06',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',5,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s18-06',5,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s18-06',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',5,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s18-06',5,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S6: Every clinician should consider an expected-health-outcome during a multimodal-and-multidisciplinary-pain-management-approach.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s18-06',6,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s18-06',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',6,ref(2),[A]),'expected-health-outcome',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s18-06',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',6,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s18-06',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',6,ref(3),[A]),'multimodal-and-multidisciplinary-pain-management-approach',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s18-06',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',6,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s18-06',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',6,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s18-06',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',6,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s18-06',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',6,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s18-06',6,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s18-06',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',6,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s18-06',6,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S7: Every clinician should consider a well-being during a multimodal-and-multidisciplinary-pain-management-approach.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s18-06',7,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s18-06',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',7,ref(2),[A]),'well-being',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s18-06',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',7,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s18-06',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',7,ref(3),[A]),'multimodal-and-multidisciplinary-pain-management-approach',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s18-06',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',7,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s18-06',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',7,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s18-06',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',7,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s18-06',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',7,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s18-06',7,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s18-06',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-06',7,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s18-06',7,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
