% cdc2022-opioid-s28-10.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s28-10',ace_sha256(a3857b73f3786232da4bf9634e893bd16c588143241fe0ddec757485aee5ee5d),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: Every clinician may assess a patient-function with a PEG-assessment-scale.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s28-10',1,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s28-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',1,ref(2),[A]),'patient-function',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s28-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s28-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',1,ref(3),[A]),'PEG-assessment-scale',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s28-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s28-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',1,ref(4),[A]),assess) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s28-10',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s28-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',1,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s28-10',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician may assess a pain-severity with a PEG-assessment-scale.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s28-10',2,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s28-10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',2,ref(2),[A]),'pain-severity',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s28-10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s28-10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',2,ref(3),[A]),'PEG-assessment-scale',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s28-10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s28-10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',2,ref(4),[A]),assess) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s28-10',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s28-10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',2,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s28-10',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician may assess a quality-of-life with a PEG-assessment-scale.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s28-10',3,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s28-10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',3,ref(2),[A]),'quality-of-life',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s28-10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s28-10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',3,ref(3),[A]),'PEG-assessment-scale',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s28-10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s28-10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',3,ref(4),[A]),assess) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s28-10',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s28-10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-10',3,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s28-10',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: If a clinician assesses a patient-function with a PEG-assessment-scale then the clinician may monitor the patient-function with the PEG-assessment-scale.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s28-10',4,box(1),[A,B,C,D]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'patient-function',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PEG-assessment-scale',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,assess), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C), \+ guideline_exclusion('cdc-2022-opioid').
guideline_event('$guideline_id'(context,'cdc2022-opioid-s28-10',4,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-10',4,ref(5),[A,B,C,D]),monitor) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'patient-function',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PEG-assessment-scale',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,assess), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-10',4,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-10',4,ref(5),[A,B,C,D]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'patient-function',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PEG-assessment-scale',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,assess), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-10',4,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-10',4,ref(5),[A,B,C,D]),2,B) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'patient-function',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PEG-assessment-scale',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,assess), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s28-10',4,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-10',4,ref(5),[A,B,C,D]),with,C) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'patient-function',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PEG-assessment-scale',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,assess), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
% S5: If a clinician assesses a pain-severity with a PEG-assessment-scale then the clinician may monitor the pain-severity with the PEG-assessment-scale.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s28-10',5,box(1),[A,B,C,D]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'pain-severity',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PEG-assessment-scale',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,assess), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C), \+ guideline_exclusion('cdc-2022-opioid').
guideline_event('$guideline_id'(context,'cdc2022-opioid-s28-10',5,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-10',5,ref(5),[A,B,C,D]),monitor) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'pain-severity',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PEG-assessment-scale',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,assess), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-10',5,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-10',5,ref(5),[A,B,C,D]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'pain-severity',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PEG-assessment-scale',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,assess), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-10',5,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-10',5,ref(5),[A,B,C,D]),2,B) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'pain-severity',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PEG-assessment-scale',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,assess), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s28-10',5,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-10',5,ref(5),[A,B,C,D]),with,C) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'pain-severity',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PEG-assessment-scale',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,assess), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
% S6: If a clinician assesses a quality-of-life with a PEG-assessment-scale then the clinician may monitor the quality-of-life with the PEG-assessment-scale.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s28-10',6,box(1),[A,B,C,D]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'quality-of-life',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PEG-assessment-scale',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,assess), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C), \+ guideline_exclusion('cdc-2022-opioid').
guideline_event('$guideline_id'(context,'cdc2022-opioid-s28-10',6,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-10',6,ref(5),[A,B,C,D]),monitor) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'quality-of-life',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PEG-assessment-scale',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,assess), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-10',6,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-10',6,ref(5),[A,B,C,D]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'quality-of-life',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PEG-assessment-scale',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,assess), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-10',6,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-10',6,ref(5),[A,B,C,D]),2,B) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'quality-of-life',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PEG-assessment-scale',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,assess), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s28-10',6,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-10',6,ref(5),[A,B,C,D]),with,C) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'quality-of-life',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PEG-assessment-scale',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,assess), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
% S7: Every PEG-assessment-scale includes a pain-average-item and includes an enjoyment-of-life-interference-item and includes a general-activity-interference-item.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s28-10',7,ref(2),[A]),'pain-average-item',countable) :- guideline_entity(actual,A,'PEG-assessment-scale',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s28-10',7,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'PEG-assessment-scale',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s28-10',7,ref(3),[A]),include) :- guideline_entity(actual,A,'PEG-assessment-scale',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s28-10',7,ref(3),[A]),1,A) :- guideline_entity(actual,A,'PEG-assessment-scale',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s28-10',7,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s28-10',7,ref(2),[A])) :- guideline_entity(actual,A,'PEG-assessment-scale',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s28-10',7,ref(4),[A]),'enjoyment-of-life-interference-item',countable) :- guideline_entity(actual,A,'PEG-assessment-scale',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s28-10',7,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'PEG-assessment-scale',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s28-10',7,ref(5),[A]),include) :- guideline_entity(actual,A,'PEG-assessment-scale',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s28-10',7,ref(5),[A]),1,A) :- guideline_entity(actual,A,'PEG-assessment-scale',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s28-10',7,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s28-10',7,ref(4),[A])) :- guideline_entity(actual,A,'PEG-assessment-scale',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s28-10',7,ref(6),[A]),'general-activity-interference-item',countable) :- guideline_entity(actual,A,'PEG-assessment-scale',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s28-10',7,ref(6),[A]),na,eq,1) :- guideline_entity(actual,A,'PEG-assessment-scale',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s28-10',7,ref(7),[A]),include) :- guideline_entity(actual,A,'PEG-assessment-scale',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s28-10',7,ref(7),[A]),1,A) :- guideline_entity(actual,A,'PEG-assessment-scale',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s28-10',7,ref(7),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s28-10',7,ref(6),[A])) :- guideline_entity(actual,A,'PEG-assessment-scale',countable), guideline_cardinality(actual,A,na,eq,1).
