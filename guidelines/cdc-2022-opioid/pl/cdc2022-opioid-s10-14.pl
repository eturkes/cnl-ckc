% cdc2022-opioid-s10-14.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_schema_version(1).
guideline_document('cdc2022-opioid-s10-14',ace_sha256(be0cf0d428712820d4d3c305d99ff93e112663e7fbfc403baf7858b4347fb741),ulex(sha256('45608095e3f77853fdabcce148a9eb7633a8ae776353b8f008c753b846886588')),vocabulary(sha256(c36289aa40f75805a83501ca057b29dd1ba7292b025716853f654890b77ec50c))).
% S1: If a serious-illness requires an advanced-pain-or-distressing-symptom-management then it is possible that a palliative-care begins during an early-treatment-phase for the serious-illness.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s10-14',1,box(1),[A,B,C]),can) :- guideline_entity(actual,A,'serious-illness',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'advanced-pain-or-distressing-symptom-management',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,require), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s10-14',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s10-14',1,ref(4),[A,B,C]),'palliative-care',countable) :- guideline_entity(actual,A,'serious-illness',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'advanced-pain-or-distressing-symptom-management',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,require), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s10-14',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s10-14',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,'serious-illness',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'advanced-pain-or-distressing-symptom-management',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,require), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s10-14',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s10-14',1,ref(5),[A,B,C]),'early-treatment-phase',countable) :- guideline_entity(actual,A,'serious-illness',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'advanced-pain-or-distressing-symptom-management',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,require), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s10-14',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s10-14',1,ref(5),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,'serious-illness',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'advanced-pain-or-distressing-symptom-management',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,require), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s10-14',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s10-14',1,ref(6),[A,B,C]),begin) :- guideline_entity(actual,A,'serious-illness',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'advanced-pain-or-distressing-symptom-management',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,require), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s10-14',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s10-14',1,ref(6),[A,B,C]),1,'$guideline_id'(product,'cdc2022-opioid-s10-14',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,'serious-illness',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'advanced-pain-or-distressing-symptom-management',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,require), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s10-14',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s10-14',1,ref(6),[A,B,C]),for,A) :- guideline_entity(actual,A,'serious-illness',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'advanced-pain-or-distressing-symptom-management',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,require), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s10-14',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s10-14',1,ref(6),[A,B,C]),during,'$guideline_id'(product,'cdc2022-opioid-s10-14',1,ref(5),[A,B,C])) :- guideline_entity(actual,A,'serious-illness',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'advanced-pain-or-distressing-symptom-management',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,require), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
