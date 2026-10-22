% cdc2022-opioid-s23-14.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s23-14',ace_sha256('3bebde236bbf71aa1cef00a9d771826baec8b04d163b45f03fe154e5c4b86a23'),ulex(sha256(e8b9f2411b72a24bb2298ca9b1abaf93606dc5038829e6a3043fa9931bbc5ef8)),vocabulary(sha256(b5cff67a2f9de9bee846e9e7650affaf95276ff3a26fc00befd8ceb076b0f4ac))).
% S1: Every clinician should discuss an intentional-or-unintentional-opioid-sharing-risk-for-household-or-other-person with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-14',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(2),[A]),'intentional-or-unintentional-opioid-sharing-risk-for-household-or-other-person',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should discuss a shared-opioid-overdose-at-same-or-lower-dosage-risk with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-14',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-14',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(2),[A]),'shared-opioid-overdose-at-same-or-lower-dosage-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-14',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-14',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-14',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-14',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-14',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-14',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-14',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should discuss a young-child-or-pet-unintentional-opioid-ingestion-risk with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-14',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-14',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(2),[A]),'young-child-or-pet-unintentional-opioid-ingestion-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-14',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-14',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-14',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-14',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-14',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-14',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-14',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
