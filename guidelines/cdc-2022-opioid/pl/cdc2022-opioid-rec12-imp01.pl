% cdc2022-opioid-rec12-imp01.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec12-imp01',ace_sha256('28961a7d36f8328bc6926b7fffcc63d0c891c5b30bead4eab611ef7d87f6f72e'),ulex(sha256(de64e41d6ddfdcb89810d9fc156aa0ac54562519e3bd4b94b8f7324f773ceb91)),vocabulary(sha256('06621a3dca592ca4385262a4a36dd8184670cdaef31827cd8b224c8c870c2f7d'))).
% S1: If a clinician suspects an opioid-use-disorder in a patient then the clinician should nonjudgmentally discuss an opioid-use-disorder-concern with the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec12-imp01',1,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,suspect), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,in,C), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec12-imp01',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec12-imp01',1,ref(5),[A,B,C,D]),'opioid-use-disorder-concern',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,suspect), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,in,C).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec12-imp01',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec12-imp01',1,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,suspect), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,in,C).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-rec12-imp01',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec12-imp01',1,ref(6),[A,B,C,D]),manner,nonjudgmentally) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,suspect), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,in,C).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec12-imp01',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec12-imp01',1,ref(6),[A,B,C,D]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,suspect), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,in,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec12-imp01',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec12-imp01',1,ref(6),[A,B,C,D]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,suspect), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,in,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec12-imp01',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec12-imp01',1,ref(6),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-rec12-imp01',1,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,suspect), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,in,C).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec12-imp01',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec12-imp01',1,ref(6),[A,B,C,D]),with,C) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,suspect), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,in,C).
% S2: If a clinician suspects an opioid-use-disorder in a patient then the clinician should provide an opioid-related-disclosure-opportunity to the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec12-imp01',2,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,suspect), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,in,C), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec12-imp01',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec12-imp01',2,ref(5),[A,B,C,D]),'opioid-related-disclosure-opportunity',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,suspect), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,in,C).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec12-imp01',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec12-imp01',2,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,suspect), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,in,C).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec12-imp01',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec12-imp01',2,ref(6),[A,B,C,D]),provide) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,suspect), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,in,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec12-imp01',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec12-imp01',2,ref(6),[A,B,C,D]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,suspect), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,in,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec12-imp01',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec12-imp01',2,ref(6),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-rec12-imp01',2,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,suspect), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,in,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec12-imp01',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec12-imp01',2,ref(6),[A,B,C,D]),3,C) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,suspect), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,in,C).
