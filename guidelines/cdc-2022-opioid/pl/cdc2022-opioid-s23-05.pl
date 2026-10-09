% cdc2022-opioid-s23-05.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s23-05',ace_sha256('458b102f4d44e5b752a6defe5fc97f811ae10c0f38d8ee5c19c5ce4606b352b3'),ulex(sha256('0b669018d532f506c518a6b0edffa4994f771b930477e7e2b36faf50d2b75d33')),temporal(sha256(d7b0d6c9af49f8d4b3dc12370d6b1cc20e923a69a5ce745ee26a1bc0f21effb6))).
% S1: If a patient has a prolonged-opioid-use for a few-day-period then every clinician may consider a prophylactic-pharmacologic-bowel-therapy for the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-05',1,box(1),[A,B,C,D,E]),may) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-05',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s23-05',1,ref(6),[A,B,C,D,E]),'prophylactic-pharmacologic-bowel-therapy',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-05',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s23-05',1,ref(6),[A,B,C,D,E]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-05',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s23-05',1,ref(7),[A,B,C,D,E]),consider) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-05',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s23-05',1,ref(7),[A,B,C,D,E]),1,E) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-05',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s23-05',1,ref(7),[A,B,C,D,E]),2,'$guideline_id'(product,'cdc2022-opioid-s23-05',1,ref(6),[A,B,C,D,E])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-05',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s23-05',1,ref(7),[A,B,C,D,E]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
