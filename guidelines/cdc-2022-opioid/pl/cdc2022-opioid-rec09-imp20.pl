% cdc2022-opioid-rec09-imp20.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_schema_version(1).
guideline_document('cdc2022-opioid-rec09-imp20',ace_sha256('463fd3ed8afde2116b25a8f8df9561106e8c5464cbc25aac3fa827308039fe5d'),ulex(sha256('3e37dcfbb36dbdd0830d644235c88914b28def633a7ac43d2b5bb53faf8ba507')),vocabulary(sha256('9d7632637554f8cbfe72e7d34a9b9d14c29eb8593aebb1c128a870c5dcf77ead'))).
% S1: If a clinician manages a patient then the clinician should screen a substance-use for the patient and should nonjudgmentally discuss a substance-use-concern with the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec09-imp20',1,box(1),[A,B,C]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,manage), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec09-imp20',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp20',1,ref(4),[A,B,C]),'substance-use',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,manage), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec09-imp20',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp20',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,manage), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec09-imp20',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp20',1,ref(5),[A,B,C]),screen) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,manage), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec09-imp20',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp20',1,ref(5),[A,B,C]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,manage), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec09-imp20',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp20',1,ref(5),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-rec09-imp20',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,manage), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec09-imp20',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp20',1,ref(5),[A,B,C]),for,B) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,manage), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec09-imp20',1,box(2),[A,B,C]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,manage), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec09-imp20',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp20',1,ref(6),[A,B,C]),'substance-use-concern',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,manage), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec09-imp20',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp20',1,ref(6),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,manage), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-rec09-imp20',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp20',1,ref(7),[A,B,C]),manner,nonjudgmentally) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,manage), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec09-imp20',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp20',1,ref(7),[A,B,C]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,manage), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec09-imp20',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp20',1,ref(7),[A,B,C]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,manage), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec09-imp20',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp20',1,ref(7),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-rec09-imp20',1,ref(6),[A,B,C])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,manage), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec09-imp20',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp20',1,ref(7),[A,B,C]),with,B) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,manage), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
