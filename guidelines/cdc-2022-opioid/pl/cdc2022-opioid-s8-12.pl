% cdc2022-opioid-s8-12.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s8-12',ace_sha256('631207d0c755ec46ad2a5d29ed74aff7952e89dd45065060846a0f2706204433'),ulex(sha256('0037ea725bdd10cf000798ce586d3739368b01cba88ac29668dbf1e4c9414552')),vocabulary(sha256('9d7632637554f8cbfe72e7d34a9b9d14c29eb8593aebb1c128a870c5dcf77ead'))).
% S1: Every opioid-pain-clinical-practice-guideline is a voluntary-guideline.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s8-12',1,ref(2),[A]),'voluntary-guideline',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s8-12',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s8-12',1,ref(3),[A]),be) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s8-12',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s8-12',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s8-12',1,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every opioid-pain-clinical-practice-guideline is a recommendation-only-guideline.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s8-12',2,ref(2),[A]),'recommendation-only-guideline',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s8-12',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s8-12',2,ref(3),[A]),be) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s8-12',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s8-12',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s8-12',2,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every opioid-pain-clinical-practice-guideline should support a clinical-judgment.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s8-12',3,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s8-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-12',3,ref(2),[A]),'clinical-judgment',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s8-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-12',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s8-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-12',3,ref(3),[A]),support) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s8-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-12',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s8-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-12',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s8-12',3,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: If there is an opioid-pain-clinical-practice-guideline then it is recommended that the opioid-pain-clinical-practice-guideline does not supplant a clinical-judgment.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s8-12',4,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-s8-12',4,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-s8-12',4,box(2),[A]),-) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s8-12',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-12',4,ref(2),[A]),'clinical-judgment',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s8-12',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-12',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s8-12',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-12',4,ref(3),[A]),supplant) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s8-12',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-12',4,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s8-12',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-12',4,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s8-12',4,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every opioid-pain-clinical-practice-guideline should support an individualized-person-centered-decision.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s8-12',5,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s8-12',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-12',5,ref(2),[A]),'individualized-person-centered-decision',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s8-12',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-12',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s8-12',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-12',5,ref(3),[A]),support) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s8-12',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-12',5,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s8-12',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-12',5,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s8-12',5,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S6: If there is an opioid-pain-clinical-practice-guideline then it is recommended that the opioid-pain-clinical-practice-guideline does not supplant an individualized-person-centered-decision.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s8-12',6,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-s8-12',6,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-s8-12',6,box(2),[A]),-) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s8-12',6,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-12',6,ref(2),[A]),'individualized-person-centered-decision',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s8-12',6,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-12',6,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s8-12',6,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-12',6,ref(3),[A]),supplant) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s8-12',6,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-12',6,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s8-12',6,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-12',6,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s8-12',6,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
