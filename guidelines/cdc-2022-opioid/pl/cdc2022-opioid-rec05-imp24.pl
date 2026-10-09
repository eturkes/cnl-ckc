% cdc2022-opioid-rec05-imp24.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec05-imp24',ace_sha256('6d6b3149138e48dc1a005efad6d77d170ab939c9c92a97deadc17393368e3fc4'),ulex(sha256('0b669018d532f506c518a6b0edffa4994f771b930477e7e2b36faf50d2b75d33')),temporal(sha256(d7b0d6c9af49f8d4b3dc12370d6b1cc20e923a69a5ce745ee26a1bc0f21effb6))).
% S1: Every clinician must ensure a coordinated-pain-management.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp24',1,box(1),[A]),must) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp24',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp24',1,ref(2),[A]),'coordinated-pain-management',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp24',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp24',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp24',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp24',1,ref(3),[A]),ensure) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp24',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp24',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp24',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp24',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp24',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician must ensure a coordinated-opioid-problem-management.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp24',2,box(1),[A]),must) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp24',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp24',2,ref(2),[A]),'coordinated-opioid-problem-management',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp24',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp24',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp24',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp24',2,ref(3),[A]),ensure) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp24',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp24',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp24',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp24',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp24',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician must ensure an opioid-use-disorder-management.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp24',3,box(1),[A]),must) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp24',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp24',3,ref(2),[A]),'opioid-use-disorder-management',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp24',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp24',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp24',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp24',3,ref(3),[A]),ensure) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp24',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp24',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp24',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp24',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp24',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
