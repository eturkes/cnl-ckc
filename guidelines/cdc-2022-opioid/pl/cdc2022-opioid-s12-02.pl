% cdc2022-opioid-s12-02.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_schema_version(1).
guideline_document('cdc2022-opioid-s12-02',ace_sha256('3322bd1746dd66de3fbb29de27046d3551f0065bc40f3e13308184cc68a14ebb'),ulex(sha256(d08a8bd3e80ef6e01301a079bb5496315f35233bf199aa89b2d2ccc93b858776)),vocabulary(sha256('9d7632637554f8cbfe72e7d34a9b9d14c29eb8593aebb1c128a870c5dcf77ead'))).
% S1: If a recommendation is a category-B-recommendation then the recommendation possibly can be inapplicable to an addressed-person.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s12-02',1,box(1),[A,B,C]),can) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'category-B-recommendation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s12-02',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s12-02',1,ref(4),[A,B,C]),'addressed-person',countable) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'category-B-recommendation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s12-02',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s12-02',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'category-B-recommendation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s12-02',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s12-02',1,ref(6),[A,B,C]),possibility,possibly) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'category-B-recommendation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_property('$guideline_id'(context,'cdc2022-opioid-s12-02',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s12-02',1,ref(5),[A,B,C]),inapplicable,pos) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'category-B-recommendation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s12-02',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s12-02',1,ref(6),[A,B,C]),be) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'category-B-recommendation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s12-02',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s12-02',1,ref(6),[A,B,C]),1,A) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'category-B-recommendation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s12-02',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s12-02',1,ref(6),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s12-02',1,ref(5),[A,B,C])) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'category-B-recommendation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s12-02',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s12-02',1,ref(6),[A,B,C]),to,'$guideline_id'(product,'cdc2022-opioid-s12-02',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'category-B-recommendation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S2: Every category-B-recommendation indicates an appropriate-different-choice.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s12-02',2,ref(2),[A]),'appropriate-different-choice',countable) :- guideline_entity(actual,A,'category-B-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s12-02',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'category-B-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s12-02',2,ref(3),[A]),indicate) :- guideline_entity(actual,A,'category-B-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s12-02',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'category-B-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s12-02',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s12-02',2,ref(2),[A])) :- guideline_entity(actual,A,'category-B-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should base a category-B-decision on a patient-circumstance.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s12-02',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s12-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-02',3,ref(2),[A]),'category-B-decision',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s12-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-02',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s12-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-02',3,ref(3),[A]),'patient-circumstance',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s12-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-02',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s12-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-02',3,ref(4),[A]),base) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s12-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-02',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s12-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-02',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s12-02',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s12-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-02',3,ref(4),[A]),on,'$guideline_id'(product,'cdc2022-opioid-s12-02',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
