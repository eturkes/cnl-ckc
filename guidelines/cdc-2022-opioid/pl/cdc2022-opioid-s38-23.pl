% cdc2022-opioid-s38-23.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s38-23',ace_sha256('706c10c832b781ea2315dd7954187cbc8badb84032e0de2566374ad1d7056449'),ulex(sha256(de64e41d6ddfdcb89810d9fc156aa0ac54562519e3bd4b94b8f7324f773ceb91)),vocabulary(sha256('06621a3dca592ca4385262a4a36dd8184670cdaef31827cd8b224c8c870c2f7d'))).
% S1: If a patient collaborates in an opioid-taper then the opioid-taper shows a higher-success-likelihood.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s38-23',1,ref(4),[A,B,C]),'higher-success-likelihood',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,collaborate), guideline_arg(actual,C,1,A), guideline_pp(actual,C,in,B).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s38-23',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,collaborate), guideline_arg(actual,C,1,A), guideline_pp(actual,C,in,B).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s38-23',1,ref(5),[A,B,C]),show) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,collaborate), guideline_arg(actual,C,1,A), guideline_pp(actual,C,in,B).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s38-23',1,ref(5),[A,B,C]),1,B) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,collaborate), guideline_arg(actual,C,1,A), guideline_pp(actual,C,in,B).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s38-23',1,ref(5),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s38-23',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,collaborate), guideline_arg(actual,C,1,A), guideline_pp(actual,C,in,B).
