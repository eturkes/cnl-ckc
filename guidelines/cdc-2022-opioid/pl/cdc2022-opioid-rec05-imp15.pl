% cdc2022-opioid-rec05-imp15.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec05-imp15',ace_sha256('4663c3a9e955bc357690b470bdf4b9d557031f066de92c8889a33d8f8dbb9828'),ulex(sha256(be24cf56c59049d3c31fba641013faa22821ecda0669443a1fc4a49979ef57ae)),temporal(sha256('29a90281976bb48dfa32d9ef80651076896404c524c9bf1d7a55b06c1880be2a'))).
% S1: Every clinician may pause a taper.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp15',1,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp15',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp15',1,ref(2),[A]),taper,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp15',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp15',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp15',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp15',1,ref(3),[A]),pause) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp15',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp15',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp15',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp15',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp15',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: If a patient is ready then every clinician may restart a taper.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp15',2,box(1),[A,B,C,D]),may) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,ready,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp15',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp15',2,ref(5),[A,B,C,D]),taper,countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,ready,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp15',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp15',2,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,ready,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp15',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp15',2,ref(6),[A,B,C,D]),restart) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,ready,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp15',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp15',2,ref(6),[A,B,C,D]),1,D) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,ready,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp15',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp15',2,ref(6),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp15',2,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,ready,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
% S3: If a patient reaches a low-opioid-dosage then every clinician may slow a taper.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp15',3,box(1),[A,B,C,D]),may) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'low-opioid-dosage',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,reach), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp15',3,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp15',3,ref(5),[A,B,C,D]),taper,countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'low-opioid-dosage',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,reach), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp15',3,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp15',3,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'low-opioid-dosage',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,reach), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp15',3,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp15',3,ref(6),[A,B,C,D]),slow) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'low-opioid-dosage',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,reach), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp15',3,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp15',3,ref(6),[A,B,C,D]),1,D) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'low-opioid-dosage',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,reach), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp15',3,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp15',3,ref(6),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp15',3,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'low-opioid-dosage',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,reach), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
