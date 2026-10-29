% cdc2022-opioid-s25-17.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s25-17',ace_sha256('7fc3945b47039dfd0cc688f5ebd4e6b6ef1dd18b87a8cffdbd5ac63377ebfb48'),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: Every physical-therapy can improve a pain without a serious-harm-risk and can improve a function without a serious-harm-risk.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s25-17',1,box(1),[A]),can) :- guideline_entity(actual,A,'physical-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s25-17',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',1,ref(2),[A]),pain,countable) :- guideline_entity(actual,A,'physical-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s25-17',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'physical-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s25-17',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',1,ref(3),[A]),'serious-harm-risk',countable) :- guideline_entity(actual,A,'physical-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s25-17',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'physical-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s25-17',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',1,ref(4),[A]),improve) :- guideline_entity(actual,A,'physical-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s25-17',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,'physical-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s25-17',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s25-17',1,ref(2),[A])) :- guideline_entity(actual,A,'physical-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s25-17',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',1,ref(4),[A]),without,'$guideline_id'(product,'cdc2022-opioid-s25-17',1,ref(3),[A])) :- guideline_entity(actual,A,'physical-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s25-17',1,box(2),[A]),can) :- guideline_entity(actual,A,'physical-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s25-17',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',1,ref(5),[A]),function,countable) :- guideline_entity(actual,A,'physical-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s25-17',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',1,ref(5),[A]),na,eq,1) :- guideline_entity(actual,A,'physical-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s25-17',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',1,ref(6),[A]),'serious-harm-risk',countable) :- guideline_entity(actual,A,'physical-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s25-17',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',1,ref(6),[A]),na,eq,1) :- guideline_entity(actual,A,'physical-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s25-17',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',1,ref(7),[A]),improve) :- guideline_entity(actual,A,'physical-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s25-17',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',1,ref(7),[A]),1,A) :- guideline_entity(actual,A,'physical-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s25-17',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',1,ref(7),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s25-17',1,ref(5),[A])) :- guideline_entity(actual,A,'physical-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s25-17',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',1,ref(7),[A]),without,'$guideline_id'(product,'cdc2022-opioid-s25-17',1,ref(6),[A])) :- guideline_entity(actual,A,'physical-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: If a patient has a knee-osteoarthritis then it is possible that a weight-loss improves a pain without a serious-harm-risk and improves a function without a serious-harm-risk.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s25-17',2,box(1),[A,B,C]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'knee-osteoarthritis',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s25-17',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s25-17',2,ref(4),[A,B,C]),'weight-loss',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'knee-osteoarthritis',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s25-17',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s25-17',2,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'knee-osteoarthritis',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s25-17',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s25-17',2,ref(5),[A,B,C]),pain,countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'knee-osteoarthritis',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s25-17',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s25-17',2,ref(5),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'knee-osteoarthritis',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s25-17',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s25-17',2,ref(6),[A,B,C]),'serious-harm-risk',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'knee-osteoarthritis',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s25-17',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s25-17',2,ref(6),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'knee-osteoarthritis',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s25-17',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s25-17',2,ref(7),[A,B,C]),improve) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'knee-osteoarthritis',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s25-17',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s25-17',2,ref(7),[A,B,C]),1,'$guideline_id'(product,'cdc2022-opioid-s25-17',2,ref(4),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'knee-osteoarthritis',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s25-17',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s25-17',2,ref(7),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s25-17',2,ref(5),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'knee-osteoarthritis',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s25-17',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s25-17',2,ref(7),[A,B,C]),without,'$guideline_id'(product,'cdc2022-opioid-s25-17',2,ref(6),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'knee-osteoarthritis',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s25-17',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s25-17',2,ref(8),[A,B,C]),function,countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'knee-osteoarthritis',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s25-17',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s25-17',2,ref(8),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'knee-osteoarthritis',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s25-17',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s25-17',2,ref(9),[A,B,C]),'serious-harm-risk',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'knee-osteoarthritis',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s25-17',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s25-17',2,ref(9),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'knee-osteoarthritis',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s25-17',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s25-17',2,ref(10),[A,B,C]),improve) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'knee-osteoarthritis',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s25-17',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s25-17',2,ref(10),[A,B,C]),1,'$guideline_id'(product,'cdc2022-opioid-s25-17',2,ref(4),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'knee-osteoarthritis',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s25-17',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s25-17',2,ref(10),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s25-17',2,ref(8),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'knee-osteoarthritis',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s25-17',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s25-17',2,ref(10),[A,B,C]),without,'$guideline_id'(product,'cdc2022-opioid-s25-17',2,ref(9),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'knee-osteoarthritis',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S3: Every behavioral-therapy can improve a pain without a serious-harm-risk and can improve a function without a serious-harm-risk.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s25-17',3,box(1),[A]),can) :- guideline_entity(actual,A,'behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s25-17',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',3,ref(2),[A]),pain,countable) :- guideline_entity(actual,A,'behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s25-17',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s25-17',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',3,ref(3),[A]),'serious-harm-risk',countable) :- guideline_entity(actual,A,'behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s25-17',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s25-17',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',3,ref(4),[A]),improve) :- guideline_entity(actual,A,'behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s25-17',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,'behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s25-17',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s25-17',3,ref(2),[A])) :- guideline_entity(actual,A,'behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s25-17',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',3,ref(4),[A]),without,'$guideline_id'(product,'cdc2022-opioid-s25-17',3,ref(3),[A])) :- guideline_entity(actual,A,'behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s25-17',3,box(2),[A]),can) :- guideline_entity(actual,A,'behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s25-17',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',3,ref(5),[A]),function,countable) :- guideline_entity(actual,A,'behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s25-17',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',3,ref(5),[A]),na,eq,1) :- guideline_entity(actual,A,'behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s25-17',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',3,ref(6),[A]),'serious-harm-risk',countable) :- guideline_entity(actual,A,'behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s25-17',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',3,ref(6),[A]),na,eq,1) :- guideline_entity(actual,A,'behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s25-17',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',3,ref(7),[A]),improve) :- guideline_entity(actual,A,'behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s25-17',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',3,ref(7),[A]),1,A) :- guideline_entity(actual,A,'behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s25-17',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',3,ref(7),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s25-17',3,ref(5),[A])) :- guideline_entity(actual,A,'behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s25-17',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-17',3,ref(7),[A]),without,'$guideline_id'(product,'cdc2022-opioid-s25-17',3,ref(6),[A])) :- guideline_entity(actual,A,'behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: A behavioral-therapy-example includes a cognitive-behavioral-therapy-example and includes a mindfulness-based-stress-reduction-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s25-17',4,ref(1),[]),'behavioral-therapy-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s25-17',4,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s25-17',4,ref(2),[]),'cognitive-behavioral-therapy-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s25-17',4,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s25-17',4,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s25-17',4,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s25-17',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s25-17',4,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s25-17',4,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s25-17',4,ref(4),[]),'mindfulness-based-stress-reduction-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s25-17',4,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s25-17',4,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s25-17',4,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s25-17',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s25-17',4,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s25-17',4,ref(4),[])).
