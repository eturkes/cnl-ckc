% cdc2022-opioid-s30-06.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_schema_version(3).
guideline_document('cdc2022-opioid-s30-06',ace_sha256('337700aad4380438109675df74fd34765ebe2137d21139fef5d7f75119d519e6'),ulex(sha256('0b669018d532f506c518a6b0edffa4994f771b930477e7e2b36faf50d2b75d33')),temporal(sha256(d7b0d6c9af49f8d4b3dc12370d6b1cc20e923a69a5ce745ee26a1bc0f21effb6))).
% S1: If a patient has a single-modality-therapy-nonresponse then every clinician should consider a multimodal-therapy for the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s30-06',1,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'single-modality-therapy-nonresponse',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-06',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-06',1,ref(5),[A,B,C,D]),'multimodal-therapy',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'single-modality-therapy-nonresponse',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-06',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-06',1,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'single-modality-therapy-nonresponse',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s30-06',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-06',1,ref(6),[A,B,C,D]),consider) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'single-modality-therapy-nonresponse',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-06',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-06',1,ref(6),[A,B,C,D]),1,D) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'single-modality-therapy-nonresponse',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-06',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-06',1,ref(6),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-s30-06',1,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'single-modality-therapy-nonresponse',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s30-06',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-06',1,ref(6),[A,B,C,D]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'single-modality-therapy-nonresponse',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
% S2: Every clinician should individualize a multimodal-therapy with a patient-need.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s30-06',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',2,ref(2),[A]),'multimodal-therapy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',2,ref(3),[A]),'patient-need',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s30-06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',2,ref(4),[A]),individualize) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s30-06',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s30-06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',2,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s30-06',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should individualize a multimodal-therapy with a treatment-cost.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s30-06',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-06',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',3,ref(2),[A]),'multimodal-therapy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-06',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-06',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',3,ref(3),[A]),'treatment-cost',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-06',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s30-06',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',3,ref(4),[A]),individualize) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-06',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-06',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s30-06',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s30-06',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',3,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s30-06',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician should individualize a multimodal-therapy with a treatment-convenience.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s30-06',4,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-06',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',4,ref(2),[A]),'multimodal-therapy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-06',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-06',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',4,ref(3),[A]),'treatment-convenience',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-06',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s30-06',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',4,ref(4),[A]),individualize) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-06',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-06',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s30-06',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s30-06',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',4,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s30-06',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every clinician should individualize a multimodal-therapy with an individual-factor.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s30-06',5,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-06',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',5,ref(2),[A]),'multimodal-therapy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-06',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-06',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',5,ref(3),[A]),'individual-factor',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-06',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',5,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s30-06',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',5,ref(4),[A]),individualize) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-06',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',5,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-06',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',5,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s30-06',5,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s30-06',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-06',5,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s30-06',5,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
