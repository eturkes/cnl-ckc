% cdc2022-opioid-s12-03.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_schema_version(3).
guideline_document('cdc2022-opioid-s12-03',ace_sha256('2d70ab2b1563bc96d260a9e99691b1927f3a6ff789e97bf00a1ae54fb90fdaae'),ulex(sha256('0b669018d532f506c518a6b0edffa4994f771b930477e7e2b36faf50d2b75d33')),temporal(sha256(d7b0d6c9af49f8d4b3dc12370d6b1cc20e923a69a5ce745ee26a1bc0f21effb6))).
% S1: For every category-B-recommendation every clinician must help a patient with a category-B-decision.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s12-03',1,box(1),[A,B]),must) :- guideline_entity(actual,A,'category-B-recommendation',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s12-03',1,box(1),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s12-03',1,ref(3),[A,B]),patient,countable) :- guideline_entity(actual,A,'category-B-recommendation',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s12-03',1,box(1),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s12-03',1,ref(3),[A,B]),na,eq,1) :- guideline_entity(actual,A,'category-B-recommendation',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s12-03',1,box(1),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s12-03',1,ref(4),[A,B]),'category-B-decision',countable) :- guideline_entity(actual,A,'category-B-recommendation',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s12-03',1,box(1),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s12-03',1,ref(4),[A,B]),na,eq,1) :- guideline_entity(actual,A,'category-B-recommendation',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s12-03',1,box(1),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s12-03',1,ref(5),[A,B]),help) :- guideline_entity(actual,A,'category-B-recommendation',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s12-03',1,box(1),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s12-03',1,ref(5),[A,B]),1,B) :- guideline_entity(actual,A,'category-B-recommendation',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s12-03',1,box(1),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s12-03',1,ref(5),[A,B]),2,'$guideline_id'(product,'cdc2022-opioid-s12-03',1,ref(3),[A,B])) :- guideline_entity(actual,A,'category-B-recommendation',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s12-03',1,box(1),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s12-03',1,ref(5),[A,B]),with,'$guideline_id'(product,'cdc2022-opioid-s12-03',1,ref(4),[A,B])) :- guideline_entity(actual,A,'category-B-recommendation',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
% S2: Every clinician must consider a patient-value during a category-B-decision.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s12-03',2,box(1),[A]),must) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s12-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',2,ref(2),[A]),'patient-value',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s12-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s12-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',2,ref(3),[A]),'category-B-decision',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s12-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s12-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',2,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s12-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s12-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s12-03',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s12-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',2,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s12-03',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician must consider a patient-preference during a category-B-decision.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s12-03',3,box(1),[A]),must) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s12-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',3,ref(2),[A]),'patient-preference',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s12-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s12-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',3,ref(3),[A]),'category-B-decision',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s12-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s12-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',3,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s12-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s12-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s12-03',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s12-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',3,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s12-03',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician must consider a specific-clinical-situation during a category-B-decision.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s12-03',4,box(1),[A]),must) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s12-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',4,ref(2),[A]),'specific-clinical-situation',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s12-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s12-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',4,ref(3),[A]),'category-B-decision',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s12-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s12-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',4,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s12-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s12-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s12-03',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s12-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',4,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s12-03',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every clinician must use a shared-decision-making during a category-B-decision.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s12-03',5,box(1),[A]),must) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s12-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',5,ref(2),[A]),'shared-decision-making',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s12-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s12-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',5,ref(3),[A]),'category-B-decision',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s12-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',5,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s12-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',5,ref(4),[A]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s12-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',5,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s12-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',5,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s12-03',5,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s12-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-03',5,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s12-03',5,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
