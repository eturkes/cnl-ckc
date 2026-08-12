% cdc2022-opioid-rec08.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec08',ace_sha256('54ea37b21802df4781dd8c6b51271dc6cfcd2b676a25f159d83976ce684ca54d'),ulex(sha256('9d39595833bb1358c343511806091b34e2442561f073b432f531f68ac385a15e'))).
% S1: A recommendation is a category-A-recommendation and is an evidence-type-4-recommendation.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(1),[]),recommendation,countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(2),[]),'category-A-recommendation',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(3),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(4),[]),'evidence-type-4-recommendation',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(5),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(4),[])).
% S2: Every clinician should evaluate a risk for an opioid-related-harm before an opioid-initiation.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec08',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(2),[A]),risk,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(3),[A]),'opioid-related-harm',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(4),[A]),'opioid-initiation',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec08',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(5),[A]),evaluate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec08',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(5),[A]),before,'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec08',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(5),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should evaluate a risk for an opioid-related-harm during a periodic-continuation-review.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec08',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(2),[A]),risk,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(3),[A]),'opioid-related-harm',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(4),[A]),'periodic-continuation-review',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(5),[A]),evaluate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(5),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(5),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician should discuss a risk for an opioid-related-harm with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec08',4,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(2),[A]),risk,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(3),[A]),'opioid-related-harm',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(4),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec08',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(5),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec08',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(5),[A]),with,'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec08',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(5),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every clinician should incorporate a risk-mitigation-strategy in a management-plan with a patient and should offer a naloxone-dose.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec08',5,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',5,ref(2),[A]),'risk-mitigation-strategy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',5,ref(3),[A]),'management-plan',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',5,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',5,ref(4),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',5,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec08',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',5,ref(5),[A]),incorporate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',5,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',5,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec08',5,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec08',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',5,ref(5),[A]),with,'$guideline_id'(product,'cdc2022-opioid-rec08',5,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec08',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',5,ref(5),[A]),in,'$guideline_id'(product,'cdc2022-opioid-rec08',5,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec08',5,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',5,ref(6),[A]),'naloxone-dose',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',5,ref(6),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec08',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',5,ref(7),[A]),offer) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',5,ref(7),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',5,ref(7),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec08',5,ref(6),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
