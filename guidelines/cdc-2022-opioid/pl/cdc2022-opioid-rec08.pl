% cdc2022-opioid-rec08.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
:- multifile(guideline_exclusion/1).
:- discontiguous(guideline_exclusion/1).
:- multifile(guideline_supersession/4).
:- discontiguous(guideline_supersession/4).
:- multifile(guideline_evaluation_date/1).
:- discontiguous(guideline_evaluation_date/1).
guideline_schema_version(1).
guideline_document('cdc2022-opioid-rec08',ace_sha256('6efccfe8c3125fdf7b43c111599f57ea42e8934b0fc2cf78598d2cca8f1d1e3b'),ulex(sha256(de64e41d6ddfdcb89810d9fc156aa0ac54562519e3bd4b94b8f7324f773ceb91)),vocabulary(sha256('06621a3dca592ca4385262a4a36dd8184670cdaef31827cd8b224c8c870c2f7d'))).
% S1: A recommendation is a category-A-recommendation and is an evidence-type-4-recommendation.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(1),[]),recommendation,countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(2),[]),'category-A-recommendation',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(3),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(2),[])).
guideline_grade(actual,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(1),[]),strength,strong,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(4),[]),'evidence-type-4-recommendation',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(5),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(4),[])).
guideline_grade(actual,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(1),[]),evidence,very_low,'$guideline_id'(product,'cdc2022-opioid-rec08',1,ref(4),[])).
% S2: Every clinician should evaluate a risk for an opioid-related-harm before an opioid-initiation and should discuss a risk for an opioid-related-harm with a patient before an opioid-initiation.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec08',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
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
guideline_order('$guideline_id'(context,'cdc2022-opioid-rec08',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(5),[A]),before,'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec08',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(5),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec08',2,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(6),[A]),risk,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(6),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(7),[A]),'opioid-related-harm',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(7),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(8),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(8),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(9),[A]),'opioid-initiation',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(9),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec08',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(10),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(10),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(10),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(6),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec08',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(10),[A]),before,'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(9),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_order('$guideline_id'(context,'cdc2022-opioid-rec08',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(10),[A]),before,'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(9),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec08',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(10),[A]),with,'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(8),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec08',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(10),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec08',2,ref(7),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should periodically evaluate a risk for an opioid-related-harm during a continued-opioid-therapy and should periodically discuss a risk for an opioid-related-harm with a patient during a continued-opioid-therapy.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec08',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(2),[A]),risk,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(3),[A]),'opioid-related-harm',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(4),[A]),'continued-opioid-therapy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(5),[A]),usuality,periodically) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(5),[A]),evaluate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(5),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(5),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec08',3,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(6),[A]),risk,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(6),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(7),[A]),'opioid-related-harm',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(7),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(8),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(8),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(9),[A]),'continued-opioid-therapy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(9),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(10),[A]),usuality,periodically) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(10),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(10),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(10),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(6),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(10),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(9),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(10),[A]),with,'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(8),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec08',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(10),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec08',3,ref(7),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician should incorporate a risk-mitigation-strategy in a management-plan with a patient and should offer a naloxone-dose.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec08',4,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(2),[A]),'risk-mitigation-strategy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(3),[A]),'management-plan',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(4),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec08',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(5),[A]),incorporate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec08',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(5),[A]),with,'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec08',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(5),[A]),in,'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec08',4,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(6),[A]),'naloxone-dose',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(6),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec08',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(7),[A]),offer) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(7),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(7),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec08',4,ref(6),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
