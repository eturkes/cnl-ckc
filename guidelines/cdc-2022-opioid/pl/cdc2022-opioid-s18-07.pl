% cdc2022-opioid-s18-07.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s18-07',ace_sha256('3587c453a7c2e81d1db3ff5a91322ff4849f1884b24ebf3552869ab65aa6643a'),ulex(sha256('0b669018d532f506c518a6b0edffa4994f771b930477e7e2b36faf50d2b75d33')),temporal(sha256(d7b0d6c9af49f8d4b3dc12370d6b1cc20e923a69a5ce745ee26a1bc0f21effb6))).
% S1: Every clinician should avoid an opioid-pain-guideline-misapplication-beyond-intended-use.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s18-07',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s18-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-07',1,ref(2),[A]),'opioid-pain-guideline-misapplication-beyond-intended-use',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s18-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-07',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s18-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-07',1,ref(3),[A]),avoid) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s18-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-07',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s18-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-07',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s18-07',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every opioid-policy-setter should avoid a purported-guideline-derived-policy-with-unintended-patient-consequence.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s18-07',2,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s18-07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-07',2,ref(2),[A]),'purported-guideline-derived-policy-with-unintended-patient-consequence',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s18-07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-07',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s18-07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-07',2,ref(3),[A]),avoid) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s18-07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-07',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s18-07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-07',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s18-07',2,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every opioid-policy-setter should avoid a purported-guideline-derived-policy-with-potentially-harmful-patient-consequence.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s18-07',3,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s18-07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-07',3,ref(2),[A]),'purported-guideline-derived-policy-with-potentially-harmful-patient-consequence',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s18-07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-07',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s18-07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-07',3,ref(3),[A]),avoid) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s18-07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-07',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s18-07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-07',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s18-07',3,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
