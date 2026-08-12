% cdc2022-opioid-s8-12.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s8-12',ace_sha256(b379c053fb53d55eddfaac91bdaa71e40cc5a418353315b86330f5fd9031b440),ulex(sha256(ef57ba7180864095bab88b95f11ddb1a2c858593eba715dbfbe874db09506c5b))).
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
% S4: Every opioid-pain-clinical-practice-guideline should not supplant a clinical-judgment.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s8-12',4,box(1),[A]),-) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-s8-12',4,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-s8-12',4,box(2),[A]),should) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
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
% S6: Every opioid-pain-clinical-practice-guideline should not supplant an individualized-person-centered-decision.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s8-12',6,box(1),[A]),-) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-s8-12',6,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-s8-12',6,box(2),[A]),should) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s8-12',6,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-12',6,ref(2),[A]),'individualized-person-centered-decision',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s8-12',6,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-12',6,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s8-12',6,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-12',6,ref(3),[A]),supplant) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s8-12',6,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-12',6,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s8-12',6,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s8-12',6,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s8-12',6,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
