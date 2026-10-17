% cdc2022-opioid-s9-01.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s9-01',ace_sha256(addabe5536e434ec0afa828f6000e2c9eab0f5f6d642868063e047652f682361),ulex(sha256(d08a8bd3e80ef6e01301a079bb5496315f35233bf199aa89b2d2ccc93b858776)),vocabulary(sha256('9d7632637554f8cbfe72e7d34a9b9d14c29eb8593aebb1c128a870c5dcf77ead'))).
% S1: Every opioid-pain-clinical-practice-guideline is not a clinical-judgment-replacement.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-01',1,box(1),[A]),-) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-01',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',1,ref(2),[A]),'clinical-judgment-replacement',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-01',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-01',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',1,ref(3),[A]),be) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-01',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-01',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-01',1,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every opioid-pain-clinical-practice-guideline is not an individualized-person-centered-care-replacement.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-01',2,box(1),[A]),-) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',2,ref(2),[A]),'individualized-person-centered-care-replacement',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',2,ref(3),[A]),be) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-01',2,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every opioid-pain-clinical-practice-guideline is not an inflexible-standard.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-01',3,box(1),[A]),-) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',3,ref(2),[A]),'inflexible-standard',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',3,ref(3),[A]),be) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-01',3,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: If there is an opioid-pain-clinical-practice-guideline then it is recommended that the opioid-pain-clinical-practice-guideline does not cause a rapid-opioid-taper.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-01',4,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-s9-01',4,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-s9-01',4,box(2),[A]),-) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-01',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',4,ref(2),[A]),'rapid-opioid-taper',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-01',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-01',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',4,ref(3),[A]),cause) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-01',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',4,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-01',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',4,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-01',4,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S5: If there is an opioid-pain-clinical-practice-guideline then it is recommended that the opioid-pain-clinical-practice-guideline does not cause an abrupt-opioid-discontinuation.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-01',5,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-s9-01',5,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-s9-01',5,box(2),[A]),-) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-01',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',5,ref(2),[A]),'abrupt-opioid-discontinuation',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-01',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-01',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',5,ref(3),[A]),cause) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-01',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',5,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-01',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',5,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-01',5,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S6: Every opioid-pain-clinical-practice-guideline is not a clinical-practice-law.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-01',6,box(1),[A]),-) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-01',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',6,ref(2),[A]),'clinical-practice-law',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-01',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',6,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-01',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',6,ref(3),[A]),be) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-01',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',6,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-01',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',6,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-01',6,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S7: Every opioid-pain-clinical-practice-guideline is not a clinical-practice-regulation.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-01',7,box(1),[A]),-) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-01',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',7,ref(2),[A]),'clinical-practice-regulation',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-01',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',7,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-01',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',7,ref(3),[A]),be) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-01',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',7,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-01',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',7,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-01',7,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S8: Every opioid-pain-clinical-practice-guideline is not a clinical-practice-policy.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-01',8,box(1),[A]),-) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-01',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',8,ref(2),[A]),'clinical-practice-policy',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-01',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',8,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-01',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',8,ref(3),[A]),be) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-01',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',8,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-01',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',8,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-01',8,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S9: Every opioid-pain-clinical-practice-guideline is not an FDA-approved-labeling-substitute.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-01',9,box(1),[A]),-) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-01',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',9,ref(2),[A]),'FDA-approved-labeling-substitute',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-01',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',9,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-01',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',9,ref(3),[A]),be) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-01',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',9,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-01',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',9,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-01',9,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S10: Every opioid-pain-clinical-practice-guideline does not apply during a sickle-cell-disease-pain.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-01',10,box(1),[A]),-) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-01',10,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',10,ref(2),[A]),'sickle-cell-disease-pain',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-01',10,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',10,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-01',10,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',10,ref(3),[A]),apply) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-01',10,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',10,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s9-01',10,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',10,ref(3),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s9-01',10,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S11: Every opioid-pain-clinical-practice-guideline does not apply during a cancer-related-pain.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-01',11,box(1),[A]),-) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-01',11,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',11,ref(2),[A]),'cancer-related-pain',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-01',11,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',11,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-01',11,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',11,ref(3),[A]),apply) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-01',11,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',11,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s9-01',11,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',11,ref(3),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s9-01',11,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S12: Every opioid-pain-clinical-practice-guideline does not apply during a palliative-care.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-01',12,box(1),[A]),-) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-01',12,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',12,ref(2),[A]),'palliative-care',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-01',12,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',12,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-01',12,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',12,ref(3),[A]),apply) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-01',12,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',12,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s9-01',12,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',12,ref(3),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s9-01',12,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S13: Every opioid-pain-clinical-practice-guideline does not apply during an end-of-life-care.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-01',13,box(1),[A]),-) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-01',13,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',13,ref(2),[A]),'end-of-life-care',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-01',13,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',13,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-01',13,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',13,ref(3),[A]),apply) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-01',13,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',13,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s9-01',13,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',13,ref(3),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s9-01',13,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S14: Every opioid-pain-clinical-practice-guideline is not an opioid-use-disorder-medication-guideline.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-01',14,box(1),[A]),-) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-01',14,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',14,ref(2),[A]),'opioid-use-disorder-medication-guideline',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-01',14,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',14,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-01',14,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',14,ref(3),[A]),be) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-01',14,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',14,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-01',14,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-01',14,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-01',14,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
