% cdc2022-opioid-s10-11.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s10-11',ace_sha256(b8b15e49160fb7a37d04ca92aa28e5dafb8f1a7db038f5cacb82f6cde3efec63),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: Every opioid-pain-clinical-practice-guideline addresses an opioid-pain-medication-use in a special-population.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',1,ref(2),[A]),'opioid-pain-medication-use',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',1,ref(3),[A]),'special-population',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',1,ref(4),[A]),address) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s10-11',1,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',1,ref(4),[A]),in,'$guideline_id'(product,'cdc2022-opioid-s10-11',1,ref(3),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every opioid-pain-clinical-practice-guideline addresses an opioid-pain-medication-use in a special-risk-condition-population.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',2,ref(2),[A]),'opioid-pain-medication-use',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',2,ref(3),[A]),'special-risk-condition-population',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',2,ref(4),[A]),address) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s10-11',2,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',2,ref(4),[A]),in,'$guideline_id'(product,'cdc2022-opioid-s10-11',2,ref(3),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: A special-population-example includes an older-adult-example and includes a pregnant-person-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',3,ref(1),[]),'special-population-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',3,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',3,ref(2),[]),'older-adult-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',3,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',3,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',3,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s10-11',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',3,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s10-11',3,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',3,ref(4),[]),'pregnant-person-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',3,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',3,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',3,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s10-11',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',3,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s10-11',3,ref(4),[])).
% S4: A special-risk-condition-example includes a substance-use-disorder-history-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',4,ref(1),[]),'special-risk-condition-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',4,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',4,ref(2),[]),'substance-use-disorder-history-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',4,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',4,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',4,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s10-11',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s10-11',4,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s10-11',4,ref(2),[])).
