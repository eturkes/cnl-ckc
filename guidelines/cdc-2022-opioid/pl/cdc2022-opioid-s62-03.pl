% cdc2022-opioid-s62-03.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s62-03',ace_sha256(c96e67e09dc5f308ec3e78b5b8615f2a1f7cf98b86fc2b205a1947d2407b8de6),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If there is a robust-evidence-based-treatment-coverage then it is possible that the robust-evidence-based-treatment-coverage facilitates an evidence-based-default-pain-treatment and encourages the evidence-based-default-pain-treatment.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s62-03',1,box(1),[A]),can) :- guideline_entity(actual,A,'robust-evidence-based-treatment-coverage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s62-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',1,ref(2),[A]),'evidence-based-default-pain-treatment',countable) :- guideline_entity(actual,A,'robust-evidence-based-treatment-coverage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s62-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'robust-evidence-based-treatment-coverage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s62-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',1,ref(3),[A]),facilitate) :- guideline_entity(actual,A,'robust-evidence-based-treatment-coverage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s62-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'robust-evidence-based-treatment-coverage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s62-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s62-03',1,ref(2),[A])) :- guideline_entity(actual,A,'robust-evidence-based-treatment-coverage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s62-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',1,ref(4),[A]),encourage) :- guideline_entity(actual,A,'robust-evidence-based-treatment-coverage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s62-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,'robust-evidence-based-treatment-coverage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s62-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s62-03',1,ref(2),[A])) :- guideline_entity(actual,A,'robust-evidence-based-treatment-coverage',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: If there is a robust-evidence-based-treatment-access then it is possible that the robust-evidence-based-treatment-access facilitates an evidence-based-default-pain-treatment and encourages the evidence-based-default-pain-treatment.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s62-03',2,box(1),[A]),can) :- guideline_entity(actual,A,'robust-evidence-based-treatment-access',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s62-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',2,ref(2),[A]),'evidence-based-default-pain-treatment',countable) :- guideline_entity(actual,A,'robust-evidence-based-treatment-access',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s62-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'robust-evidence-based-treatment-access',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s62-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',2,ref(3),[A]),facilitate) :- guideline_entity(actual,A,'robust-evidence-based-treatment-access',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s62-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'robust-evidence-based-treatment-access',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s62-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s62-03',2,ref(2),[A])) :- guideline_entity(actual,A,'robust-evidence-based-treatment-access',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s62-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',2,ref(4),[A]),encourage) :- guideline_entity(actual,A,'robust-evidence-based-treatment-access',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s62-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,'robust-evidence-based-treatment-access',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s62-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s62-03',2,ref(2),[A])) :- guideline_entity(actual,A,'robust-evidence-based-treatment-access',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: If there is a pain-treatment-decision-support then it is possible that the pain-treatment-decision-support facilitates an evidence-based-default-pain-treatment and encourages the evidence-based-default-pain-treatment.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s62-03',3,box(1),[A]),can) :- guideline_entity(actual,A,'pain-treatment-decision-support',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s62-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',3,ref(2),[A]),'evidence-based-default-pain-treatment',countable) :- guideline_entity(actual,A,'pain-treatment-decision-support',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s62-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'pain-treatment-decision-support',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s62-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',3,ref(3),[A]),facilitate) :- guideline_entity(actual,A,'pain-treatment-decision-support',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s62-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,'pain-treatment-decision-support',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s62-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s62-03',3,ref(2),[A])) :- guideline_entity(actual,A,'pain-treatment-decision-support',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s62-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',3,ref(4),[A]),encourage) :- guideline_entity(actual,A,'pain-treatment-decision-support',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s62-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,'pain-treatment-decision-support',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s62-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s62-03',3,ref(2),[A])) :- guideline_entity(actual,A,'pain-treatment-decision-support',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: A robust-evidence-based-treatment-coverage-example includes a limited-utilization-management-example and includes a limited-cost-sharing-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s62-03',4,ref(1),[]),'robust-evidence-based-treatment-coverage-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s62-03',4,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s62-03',4,ref(2),[]),'limited-utilization-management-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s62-03',4,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s62-03',4,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s62-03',4,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s62-03',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s62-03',4,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s62-03',4,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s62-03',4,ref(4),[]),'limited-cost-sharing-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s62-03',4,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s62-03',4,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s62-03',4,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s62-03',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s62-03',4,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s62-03',4,ref(4),[])).
% S5: A pain-treatment-decision-support-example is an EHR-prescribing-default-adjustment-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s62-03',5,ref(1),[]),'pain-treatment-decision-support-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s62-03',5,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s62-03',5,ref(2),[]),'EHR-prescribing-default-adjustment-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s62-03',5,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s62-03',5,ref(3),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s62-03',5,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s62-03',5,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s62-03',5,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s62-03',5,ref(2),[])).
