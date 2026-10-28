% cdc2022-opioid-rec07-imp09.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec07-imp09',ace_sha256(a32772aaebfc24d353bab8ba31f2912c1f993452fe7453cdf5a94f34f15074ae),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If a patient receives an opioid-therapy then every clinician should regularly screen the patient for an opioid-risk-condition during the opioid-therapy.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec07-imp09',1,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec07-imp09',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',1,ref(5),[A,B,C,D]),'opioid-risk-condition',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec07-imp09',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',1,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-rec07-imp09',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',1,ref(6),[A,B,C,D]),usuality,regularly) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec07-imp09',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',1,ref(6),[A,B,C,D]),screen) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec07-imp09',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',1,ref(6),[A,B,C,D]),1,D) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec07-imp09',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',1,ref(6),[A,B,C,D]),2,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec07-imp09',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',1,ref(6),[A,B,C,D]),during,B) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec07-imp09',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',1,ref(6),[A,B,C,D]),for,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',1,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
% S2: If a patient receives an opioid-therapy and has an opioid-risk-condition then the opioid-risk-condition can change during the opioid-therapy.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec07-imp09',2,box(1),[A,B,C,D,E]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'opioid-risk-condition',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec07-imp09',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',2,ref(6),[A,B,C,D,E]),change) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'opioid-risk-condition',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec07-imp09',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',2,ref(6),[A,B,C,D,E]),1,D) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'opioid-risk-condition',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec07-imp09',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',2,ref(6),[A,B,C,D,E]),during,B) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'opioid-risk-condition',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
% S3: An opioid-risk-condition-example includes a depression-example and includes a mental-health-condition-example and includes a substance-use-disorder-history-example and includes an overdose-history-example and includes an at-least-50-MME-per-day-dosage-example and includes a concurrent-central-nervous-system-depressant-use-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(1),[]),'opioid-risk-condition-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(2),[]),'depression-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(4),[]),'mental-health-condition-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(4),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(6),[]),'substance-use-disorder-history-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(6),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(7),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(7),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(7),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(6),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(8),[]),'overdose-history-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(8),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(9),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(9),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(9),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(8),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(10),[]),'at-least-50-MME-per-day-dosage-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(10),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(11),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(11),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(11),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(10),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(12),[]),'concurrent-central-nervous-system-depressant-use-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(12),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(13),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(13),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(13),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec07-imp09',3,ref(12),[])).
