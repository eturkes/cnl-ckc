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
guideline_document('cdc2022-opioid-s23-05',ace_sha256(b41569f40a4882a4cea1334f55fa35615da84e65ed8e2be4b9f6763da138ec9d),ulex(sha256('45608095e3f77853fdabcce148a9eb7633a8ae776353b8f008c753b846886588')),vocabulary(sha256(c36289aa40f75805a83501ca057b29dd1ba7292b025716853f654890b77ec50c))).
% S1: If a patient has a prolonged-opioid-use for a few-day-period then the patient can possibly need a prophylactic-pharmacologic-bowel-therapy.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-05',1,box(1),[A,B,C,D]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s23-05',1,ref(5),[A,B,C,D]),'prophylactic-pharmacologic-bowel-therapy',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s23-05',1,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s23-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s23-05',1,ref(6),[A,B,C,D]),possibility,possibly) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s23-05',1,ref(6),[A,B,C,D]),need) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s23-05',1,ref(6),[A,B,C,D]),1,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s23-05',1,ref(6),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-s23-05',1,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
