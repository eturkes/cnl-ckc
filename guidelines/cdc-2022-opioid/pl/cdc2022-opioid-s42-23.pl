% cdc2022-opioid-s42-23.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s42-23',ace_sha256('55950f42e32cd4b1b97cf7d3d0f8511835f229d291db7b26440d5ea879f38669'),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If a patient has a postsurgical-hospital-discharge and the patient receives an opioid-course-and-dosage during a hospitalization before the postsurgical-hospital-discharge then the opioid-course-and-dosage can help an ongoing-pain-management-need-prediction for the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s42-23',1,box(1),[A,B,C,D,E,F]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'postsurgical-hospital-discharge',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'opioid-course-and-dosage',countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,hospitalization,countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,receive), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,D), guideline_pp(actual,F,before,B), guideline_order(actual,F,before,B), guideline_pp(actual,F,during,E).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s42-23',1,box(1),[A,B,C,D,E,F]),'$guideline_id'(product,'cdc2022-opioid-s42-23',1,ref(7),[A,B,C,D,E,F]),'ongoing-pain-management-need-prediction',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'postsurgical-hospital-discharge',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'opioid-course-and-dosage',countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,hospitalization,countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,receive), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,D), guideline_pp(actual,F,before,B), guideline_order(actual,F,before,B), guideline_pp(actual,F,during,E).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s42-23',1,box(1),[A,B,C,D,E,F]),'$guideline_id'(product,'cdc2022-opioid-s42-23',1,ref(7),[A,B,C,D,E,F]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'postsurgical-hospital-discharge',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'opioid-course-and-dosage',countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,hospitalization,countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,receive), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,D), guideline_pp(actual,F,before,B), guideline_order(actual,F,before,B), guideline_pp(actual,F,during,E).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s42-23',1,box(1),[A,B,C,D,E,F]),'$guideline_id'(product,'cdc2022-opioid-s42-23',1,ref(8),[A,B,C,D,E,F]),help) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'postsurgical-hospital-discharge',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'opioid-course-and-dosage',countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,hospitalization,countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,receive), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,D), guideline_pp(actual,F,before,B), guideline_order(actual,F,before,B), guideline_pp(actual,F,during,E).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s42-23',1,box(1),[A,B,C,D,E,F]),'$guideline_id'(product,'cdc2022-opioid-s42-23',1,ref(8),[A,B,C,D,E,F]),1,D) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'postsurgical-hospital-discharge',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'opioid-course-and-dosage',countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,hospitalization,countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,receive), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,D), guideline_pp(actual,F,before,B), guideline_order(actual,F,before,B), guideline_pp(actual,F,during,E).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s42-23',1,box(1),[A,B,C,D,E,F]),'$guideline_id'(product,'cdc2022-opioid-s42-23',1,ref(8),[A,B,C,D,E,F]),2,'$guideline_id'(product,'cdc2022-opioid-s42-23',1,ref(7),[A,B,C,D,E,F])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'postsurgical-hospital-discharge',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'opioid-course-and-dosage',countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,hospitalization,countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,receive), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,D), guideline_pp(actual,F,before,B), guideline_order(actual,F,before,B), guideline_pp(actual,F,during,E).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s42-23',1,box(1),[A,B,C,D,E,F]),'$guideline_id'(product,'cdc2022-opioid-s42-23',1,ref(8),[A,B,C,D,E,F]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'postsurgical-hospital-discharge',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'opioid-course-and-dosage',countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,hospitalization,countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,receive), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,D), guideline_pp(actual,F,before,B), guideline_order(actual,F,before,B), guideline_pp(actual,F,during,E).
