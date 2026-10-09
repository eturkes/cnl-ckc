% cdc2022-opioid-s32-02.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s32-02',ace_sha256(f67e8bd47d827344ee8f728cd67d128b93ee814c768c886215ed610c51cb511c),ulex(sha256('0b669018d532f506c518a6b0edffa4994f771b930477e7e2b36faf50d2b75d33')),temporal(sha256(d7b0d6c9af49f8d4b3dc12370d6b1cc20e923a69a5ce745ee26a1bc0f21effb6))).
% S1: If a clinician prescribes an immediate-release-opioid with an ER-LA-opioid then the clinician should consider an increased-combination-overdose-risk and should heed a combination-opioid-caution.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s32-02',1,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'immediate-release-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'ER-LA-opioid',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s32-02',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s32-02',1,ref(5),[A,B,C,D]),'increased-combination-overdose-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'immediate-release-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'ER-LA-opioid',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s32-02',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s32-02',1,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'immediate-release-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'ER-LA-opioid',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s32-02',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s32-02',1,ref(6),[A,B,C,D]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'immediate-release-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'ER-LA-opioid',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s32-02',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s32-02',1,ref(6),[A,B,C,D]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'immediate-release-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'ER-LA-opioid',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s32-02',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s32-02',1,ref(6),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-s32-02',1,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'immediate-release-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'ER-LA-opioid',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s32-02',1,box(2),[A,B,C,D]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'immediate-release-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'ER-LA-opioid',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s32-02',1,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s32-02',1,ref(7),[A,B,C,D]),'combination-opioid-caution',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'immediate-release-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'ER-LA-opioid',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s32-02',1,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s32-02',1,ref(7),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'immediate-release-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'ER-LA-opioid',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s32-02',1,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s32-02',1,ref(8),[A,B,C,D]),heed) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'immediate-release-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'ER-LA-opioid',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s32-02',1,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s32-02',1,ref(8),[A,B,C,D]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'immediate-release-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'ER-LA-opioid',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s32-02',1,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s32-02',1,ref(8),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-s32-02',1,ref(7),[A,B,C,D])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'immediate-release-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'ER-LA-opioid',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
