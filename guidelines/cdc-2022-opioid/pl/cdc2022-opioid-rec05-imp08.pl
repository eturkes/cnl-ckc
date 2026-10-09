% cdc2022-opioid-rec05-imp08.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec05-imp08',ace_sha256('5bd40141a0815ff24a1d15eb2c7864f7f23f9921d0f0630f0d3e9c8584dc5abf'),ulex(sha256('0b669018d532f506c518a6b0edffa4994f771b930477e7e2b36faf50d2b75d33')),temporal(sha256(d7b0d6c9af49f8d4b3dc12370d6b1cc20e923a69a5ce745ee26a1bc0f21effb6))).
% S1: If a patient engages in an opioid-taper then every clinician should conduct a follow-up with the patient at an interval of at most 1 month.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp08',1,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,engage), guideline_arg(actual,C,1,A), guideline_pp(actual,C,in,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp08',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp08',1,ref(5),[A,B,C,D]),'follow-up',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,engage), guideline_arg(actual,C,1,A), guideline_pp(actual,C,in,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp08',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp08',1,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,engage), guideline_arg(actual,C,1,A), guideline_pp(actual,C,in,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp08',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp08',1,ref(6),[A,B,C,D]),interval,countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,engage), guideline_arg(actual,C,1,A), guideline_pp(actual,C,in,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp08',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp08',1,ref(6),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,engage), guideline_arg(actual,C,1,A), guideline_pp(actual,C,in,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp08',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp08',1,ref(7),[A,B,C,D]),month,countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,engage), guideline_arg(actual,C,1,A), guideline_pp(actual,C,in,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp08',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp08',1,ref(7),[A,B,C,D]),na,leq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,engage), guideline_arg(actual,C,1,A), guideline_pp(actual,C,in,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp08',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp08',1,ref(8),[A,B,C,D]),conduct) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,engage), guideline_arg(actual,C,1,A), guideline_pp(actual,C,in,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp08',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp08',1,ref(8),[A,B,C,D]),1,D) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,engage), guideline_arg(actual,C,1,A), guideline_pp(actual,C,in,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp08',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp08',1,ref(8),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp08',1,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,engage), guideline_arg(actual,C,1,A), guideline_pp(actual,C,in,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp08',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp08',1,ref(8),[A,B,C,D]),at,'$guideline_id'(product,'cdc2022-opioid-rec05-imp08',1,ref(6),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,engage), guideline_arg(actual,C,1,A), guideline_pp(actual,C,in,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_recurrence('$guideline_id'(context,'cdc2022-opioid-rec05-imp08',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp08',1,ref(8),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp08',1,ref(7),[A,B,C,D]),month) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,engage), guideline_arg(actual,C,1,A), guideline_pp(actual,C,in,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp08',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp08',1,ref(8),[A,B,C,D]),with,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,engage), guideline_arg(actual,C,1,A), guideline_pp(actual,C,in,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
