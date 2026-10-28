% cdc2022-opioid-s28-11.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s28-11',ace_sha256('92c5f3cbe396d30cba597d022e9d479bb1e641680f5e92b62dd0fd9492c3daef'),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: Every clinician may elicit a meaningful-functional-goal from a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s28-11',1,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s28-11',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-11',1,ref(2),[A]),'meaningful-functional-goal',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s28-11',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-11',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s28-11',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-11',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s28-11',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-11',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s28-11',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-11',1,ref(4),[A]),elicit) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-11',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-11',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-11',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-11',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s28-11',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s28-11',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-11',1,ref(4),[A]),from,'$guideline_id'(product,'cdc2022-opioid-s28-11',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: A meaningful-functional-goal-example includes a dog-walking-example and includes a block-walking-example and includes a part-time-work-return-example and includes a family-event-attendance-example and includes a recreational-activity-attendance-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(1),[]),'meaningful-functional-goal-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(2),[]),'dog-walking-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(4),[]),'block-walking-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(4),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(6),[]),'part-time-work-return-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(6),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(7),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(7),[]),1,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(7),[]),2,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(6),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(8),[]),'family-event-attendance-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(8),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(9),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(9),[]),1,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(9),[]),2,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(8),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(10),[]),'recreational-activity-attendance-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(10),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(11),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(11),[]),1,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(11),[]),2,'$guideline_id'(product,'cdc2022-opioid-s28-11',2,ref(10),[])).
% S3: If a clinician elicits a meaningful-functional-goal from a patient then the clinician may use the meaningful-functional-goal during an opioid-benefit-assessment for the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s28-11',3,box(1),[A,B,C,D]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'meaningful-functional-goal',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,elicit), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,from,C), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s28-11',3,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-11',3,ref(5),[A,B,C,D]),'opioid-benefit-assessment',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'meaningful-functional-goal',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,elicit), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,from,C).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s28-11',3,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-11',3,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'meaningful-functional-goal',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,elicit), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,from,C).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s28-11',3,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-11',3,ref(6),[A,B,C,D]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'meaningful-functional-goal',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,elicit), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,from,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-11',3,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-11',3,ref(6),[A,B,C,D]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'meaningful-functional-goal',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,elicit), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,from,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-11',3,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-11',3,ref(6),[A,B,C,D]),2,B) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'meaningful-functional-goal',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,elicit), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,from,C).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s28-11',3,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-11',3,ref(6),[A,B,C,D]),for,C) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'meaningful-functional-goal',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,elicit), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,from,C).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s28-11',3,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-11',3,ref(6),[A,B,C,D]),during,'$guideline_id'(product,'cdc2022-opioid-s28-11',3,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'meaningful-functional-goal',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,elicit), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,from,C).
% S4: If a clinician elicits a meaningful-functional-goal from a patient then the clinician may use the meaningful-functional-goal during a continued-opioid-benefit-risk-weighing for the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s28-11',4,box(1),[A,B,C,D]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'meaningful-functional-goal',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,elicit), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,from,C), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s28-11',4,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-11',4,ref(5),[A,B,C,D]),'continued-opioid-benefit-risk-weighing',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'meaningful-functional-goal',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,elicit), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,from,C).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s28-11',4,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-11',4,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'meaningful-functional-goal',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,elicit), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,from,C).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s28-11',4,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-11',4,ref(6),[A,B,C,D]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'meaningful-functional-goal',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,elicit), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,from,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-11',4,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-11',4,ref(6),[A,B,C,D]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'meaningful-functional-goal',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,elicit), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,from,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-11',4,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-11',4,ref(6),[A,B,C,D]),2,B) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'meaningful-functional-goal',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,elicit), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,from,C).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s28-11',4,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-11',4,ref(6),[A,B,C,D]),for,C) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'meaningful-functional-goal',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,elicit), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,from,C).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s28-11',4,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-11',4,ref(6),[A,B,C,D]),during,'$guideline_id'(product,'cdc2022-opioid-s28-11',4,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'meaningful-functional-goal',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,elicit), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,from,C).
