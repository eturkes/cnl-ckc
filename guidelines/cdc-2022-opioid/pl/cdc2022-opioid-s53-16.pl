% cdc2022-opioid-s53-16.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s53-16',ace_sha256('295e11a1d71e7d6ada613910729e03e9123e3b20c6af2338458dbb6ce084d2fc'),ulex(sha256('64c086862c2f48d491a9fde0ba9f0fcf090e25e84456b9749332ec3ace211ce4')),vocabulary(sha256('06621a3dca592ca4385262a4a36dd8184670cdaef31827cd8b224c8c870c2f7d'))).
% S1: Every false-positive-presumptive-result is a not-uncommon-result.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s53-16',1,ref(2),[A]),'not-uncommon-result',countable) :- guideline_entity(actual,A,'false-positive-presumptive-result',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s53-16',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'false-positive-presumptive-result',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s53-16',1,ref(3),[A]),be) :- guideline_entity(actual,A,'false-positive-presumptive-result',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s53-16',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'false-positive-presumptive-result',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s53-16',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s53-16',1,ref(2),[A])) :- guideline_entity(actual,A,'false-positive-presumptive-result',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every false-negative-presumptive-result is a not-uncommon-result.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s53-16',2,ref(2),[A]),'not-uncommon-result',countable) :- guideline_entity(actual,A,'false-negative-presumptive-result',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s53-16',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'false-negative-presumptive-result',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s53-16',2,ref(3),[A]),be) :- guideline_entity(actual,A,'false-negative-presumptive-result',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s53-16',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'false-negative-presumptive-result',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s53-16',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s53-16',2,ref(2),[A])) :- guideline_entity(actual,A,'false-negative-presumptive-result',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician can commonly misinterpret a presumptive-result.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s53-16',3,box(1),[A]),can) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s53-16',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-16',3,ref(2),[A]),'presumptive-result',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s53-16',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-16',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s53-16',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-16',3,ref(3),[A]),usuality,commonly) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s53-16',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-16',3,ref(3),[A]),misinterpret) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s53-16',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-16',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s53-16',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-16',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s53-16',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every presumptive-result-misinterpretation can compound a false-presumptive-result-problem and can lead to an inappropriate-patient-consequence.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s53-16',4,box(1),[A]),can) :- guideline_entity(actual,A,'presumptive-result-misinterpretation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s53-16',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-16',4,ref(2),[A]),'false-presumptive-result-problem',countable) :- guideline_entity(actual,A,'presumptive-result-misinterpretation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s53-16',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-16',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'presumptive-result-misinterpretation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s53-16',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-16',4,ref(3),[A]),compound) :- guideline_entity(actual,A,'presumptive-result-misinterpretation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s53-16',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-16',4,ref(3),[A]),1,A) :- guideline_entity(actual,A,'presumptive-result-misinterpretation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s53-16',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-16',4,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s53-16',4,ref(2),[A])) :- guideline_entity(actual,A,'presumptive-result-misinterpretation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s53-16',4,box(2),[A]),can) :- guideline_entity(actual,A,'presumptive-result-misinterpretation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s53-16',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-16',4,ref(4),[A]),'inappropriate-patient-consequence',countable) :- guideline_entity(actual,A,'presumptive-result-misinterpretation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s53-16',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-16',4,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'presumptive-result-misinterpretation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s53-16',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-16',4,ref(5),[A]),lead) :- guideline_entity(actual,A,'presumptive-result-misinterpretation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s53-16',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-16',4,ref(5),[A]),1,A) :- guideline_entity(actual,A,'presumptive-result-misinterpretation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s53-16',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-16',4,ref(5),[A]),to,'$guideline_id'(product,'cdc2022-opioid-s53-16',4,ref(4),[A])) :- guideline_entity(actual,A,'presumptive-result-misinterpretation',countable), guideline_cardinality(actual,A,na,eq,1).
