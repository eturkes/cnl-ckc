% cdc2022-opioid-s21-05.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s21-05',ace_sha256(b0bbe6137c98493d79eee6388d610a44fcd1b87def071a037d28b07e70070e25),ulex(sha256('0b669018d532f506c518a6b0edffa4994f771b930477e7e2b36faf50d2b75d33')),temporal(sha256(d7b0d6c9af49f8d4b3dc12370d6b1cc20e923a69a5ce745ee26a1bc0f21effb6))).
% S1: Every clinician should review an FDA-approved-labeling before a pharmacologic-therapy and should review a boxed-warning before a pharmacologic-therapy.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s21-05',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s21-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s21-05',1,ref(2),[A]),'FDA-approved-labeling',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s21-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s21-05',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s21-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s21-05',1,ref(3),[A]),'pharmacologic-therapy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s21-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s21-05',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s21-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s21-05',1,ref(4),[A]),review) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s21-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s21-05',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s21-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s21-05',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s21-05',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s21-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s21-05',1,ref(4),[A]),before,'$guideline_id'(product,'cdc2022-opioid-s21-05',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_order('$guideline_id'(context,'cdc2022-opioid-s21-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s21-05',1,ref(4),[A]),before,'$guideline_id'(product,'cdc2022-opioid-s21-05',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s21-05',1,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s21-05',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s21-05',1,ref(5),[A]),'boxed-warning',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s21-05',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s21-05',1,ref(5),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s21-05',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s21-05',1,ref(6),[A]),'pharmacologic-therapy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s21-05',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s21-05',1,ref(6),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s21-05',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s21-05',1,ref(7),[A]),review) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s21-05',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s21-05',1,ref(7),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s21-05',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s21-05',1,ref(7),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s21-05',1,ref(5),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s21-05',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s21-05',1,ref(7),[A]),before,'$guideline_id'(product,'cdc2022-opioid-s21-05',1,ref(6),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_order('$guideline_id'(context,'cdc2022-opioid-s21-05',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s21-05',1,ref(7),[A]),before,'$guideline_id'(product,'cdc2022-opioid-s21-05',1,ref(6),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
