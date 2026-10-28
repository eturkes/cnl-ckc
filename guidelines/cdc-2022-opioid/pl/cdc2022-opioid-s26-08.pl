% cdc2022-opioid-s26-08.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s26-08',ace_sha256('9299ed55539c4005c818f61085f6a0090c0b15da56eddb38ac4da9870b49bec4'),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: Every clinician should understand a community-nonpharmacologic-option for a low-cost-service-referral.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s26-08',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-08',1,ref(2),[A]),'community-nonpharmacologic-option',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-08',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-08',1,ref(3),[A]),'low-cost-service-referral',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-08',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s26-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-08',1,ref(4),[A]),understand) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-08',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-08',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s26-08',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s26-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-08',1,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s26-08',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: If a clinician understands a community-nonpharmacologic-option then the clinician can refer a patient to a low-cost-service.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s26-08',2,box(1),[A,B,C]),can) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'community-nonpharmacologic-option',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-08',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s26-08',2,ref(4),[A,B,C]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'community-nonpharmacologic-option',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-08',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s26-08',2,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'community-nonpharmacologic-option',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-08',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s26-08',2,ref(5),[A,B,C]),'low-cost-service',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'community-nonpharmacologic-option',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-08',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s26-08',2,ref(5),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'community-nonpharmacologic-option',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s26-08',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s26-08',2,ref(6),[A,B,C]),refer) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'community-nonpharmacologic-option',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-08',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s26-08',2,ref(6),[A,B,C]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'community-nonpharmacologic-option',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-08',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s26-08',2,ref(6),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s26-08',2,ref(4),[A,B,C])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'community-nonpharmacologic-option',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s26-08',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s26-08',2,ref(6),[A,B,C]),to,'$guideline_id'(product,'cdc2022-opioid-s26-08',2,ref(5),[A,B,C])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'community-nonpharmacologic-option',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
