% cdc2022-opioid-s60-03.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s60-03',ace_sha256('561b1920a8fd7a3ba088192134f99bf84f3be7800d8cf2b43ad3d9f7baa34b0b'),ulex(sha256('3e37dcfbb36dbdd0830d644235c88914b28def633a7ac43d2b5bb53faf8ba507')),vocabulary(sha256('9d7632637554f8cbfe72e7d34a9b9d14c29eb8593aebb1c128a870c5dcf77ead'))).
% S1: Every clinician should strongly provide an opioid-use-disorder-medication-treatment for a patient who has an opioid-use-disorder.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s60-03',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s60-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-03',1,ref(2),[A]),'opioid-use-disorder-medication-treatment',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s60-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-03',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s60-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-03',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s60-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-03',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s60-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-03',1,ref(4),[A]),'opioid-use-disorder',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s60-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-03',1,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s60-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-03',1,ref(5),[A]),have) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s60-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-03',1,ref(5),[A]),1,'$guideline_id'(product,'cdc2022-opioid-s60-03',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s60-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-03',1,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s60-03',1,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s60-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-03',1,ref(6),[A]),force,strongly) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s60-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-03',1,ref(6),[A]),provide) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s60-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-03',1,ref(6),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s60-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-03',1,ref(6),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s60-03',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s60-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-03',1,ref(6),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s60-03',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
