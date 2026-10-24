% cdc2022-opioid-s51-21.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_schema_version(1).
guideline_document('cdc2022-opioid-s51-21',ace_sha256('9dd277c3d0a69ecbaeec61a81e98dc679f637ed8fc358e85083241d602549043'),ulex(sha256('64c086862c2f48d491a9fde0ba9f0fcf090e25e84456b9749332ec3ace211ce4')),vocabulary(sha256(c36289aa40f75805a83501ca057b29dd1ba7292b025716853f654890b77ec50c))).
% S1: Every PDMP-risk-score-algorithm can reportedly generate a PDMP-risk-score.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s51-21',1,box(1),[A]),can) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s51-21',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-21',1,ref(2),[A]),'PDMP-risk-score',countable) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s51-21',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-21',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s51-21',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-21',1,ref(3),[A]),possibility,reportedly) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s51-21',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-21',1,ref(3),[A]),generate) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s51-21',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-21',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s51-21',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-21',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s51-21',1,ref(2),[A])) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every PDMP-risk-score-algorithm uses a patient-EHR-information and uses an other-source-information.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s51-21',2,ref(2),[A]),'patient-EHR-information',countable) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s51-21',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s51-21',2,ref(3),[A]),use) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s51-21',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s51-21',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s51-21',2,ref(2),[A])) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s51-21',2,ref(4),[A]),'other-source-information',countable) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s51-21',2,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s51-21',2,ref(5),[A]),use) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s51-21',2,ref(5),[A]),1,A) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s51-21',2,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s51-21',2,ref(4),[A])) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: An other-source-example includes a court-record-example and includes a criminal-history-example and includes a sexual-trauma-history-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s51-21',3,ref(1),[]),'other-source-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s51-21',3,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s51-21',3,ref(2),[]),'court-record-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s51-21',3,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s51-21',3,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s51-21',3,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s51-21',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s51-21',3,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s51-21',3,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s51-21',3,ref(4),[]),'criminal-history-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s51-21',3,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s51-21',3,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s51-21',3,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s51-21',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s51-21',3,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s51-21',3,ref(4),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s51-21',3,ref(6),[]),'sexual-trauma-history-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s51-21',3,ref(6),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s51-21',3,ref(7),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s51-21',3,ref(7),[]),1,'$guideline_id'(product,'cdc2022-opioid-s51-21',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s51-21',3,ref(7),[]),2,'$guideline_id'(product,'cdc2022-opioid-s51-21',3,ref(6),[])).
% S4: Every PDMP-risk-score-algorithm can possibly affect a woman disparately.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s51-21',4,box(1),[A]),can) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s51-21',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-21',4,ref(2),[A]),woman,countable) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s51-21',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-21',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s51-21',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-21',4,ref(3),[A]),possibility,possibly) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s51-21',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-21',4,ref(3),[A]),affect) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s51-21',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-21',4,ref(3),[A]),1,A) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s51-21',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-21',4,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s51-21',4,ref(2),[A])) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s51-21',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-21',4,ref(3),[A]),manner,disparately) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every PDMP-risk-score-algorithm can possibly affect a person-of-color disparately.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s51-21',5,box(1),[A]),can) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s51-21',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-21',5,ref(2),[A]),'person-of-color',countable) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s51-21',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-21',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s51-21',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-21',5,ref(3),[A]),possibility,possibly) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s51-21',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-21',5,ref(3),[A]),affect) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s51-21',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-21',5,ref(3),[A]),1,A) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s51-21',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-21',5,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s51-21',5,ref(2),[A])) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s51-21',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-21',5,ref(3),[A]),manner,disparately) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
% S6: Every PDMP-risk-score-algorithm can possibly affect a person-living-in-poverty disparately.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s51-21',6,box(1),[A]),can) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s51-21',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-21',6,ref(2),[A]),'person-living-in-poverty',countable) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s51-21',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-21',6,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s51-21',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-21',6,ref(3),[A]),possibility,possibly) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s51-21',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-21',6,ref(3),[A]),affect) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s51-21',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-21',6,ref(3),[A]),1,A) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s51-21',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-21',6,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s51-21',6,ref(2),[A])) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s51-21',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-21',6,ref(3),[A]),manner,disparately) :- guideline_entity(actual,A,'PDMP-risk-score-algorithm',countable), guideline_cardinality(actual,A,na,eq,1).
