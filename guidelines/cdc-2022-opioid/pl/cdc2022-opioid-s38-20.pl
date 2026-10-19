% cdc2022-opioid-s38-20.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s38-20',ace_sha256('2bea84859cee3d8666e05aa14479d8015ee2977487a1704c2980de59f27c6277'),ulex(sha256('3210e31a7a1a232faae4aed00d06481f6bef71ebd89b6c884183bc530be9986e')),vocabulary(sha256(b5cff67a2f9de9bee846e9e7650affaf95276ff3a26fc00befd8ceb076b0f4ac))).
% S1: Every opioid-taper can continue during some months and can continue during some years.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s38-20',1,box(1),[A]),can) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-20',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-20',1,ref(2),[A]),month,countable) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-20',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-20',1,ref(2),[A]),na,geq,2) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_unit('$guideline_id'(context,'cdc2022-opioid-s38-20',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-20',1,ref(2),[A]),month) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s38-20',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-20',1,ref(3),[A]),continue) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-20',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-20',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s38-20',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-20',1,ref(3),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s38-20',1,ref(2),[A])) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_interval('$guideline_id'(context,'cdc2022-opioid-s38-20',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-20',1,ref(3),[A]),duration,'$guideline_id'(product,'cdc2022-opioid-s38-20',1,ref(2),[A]),month,none) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s38-20',1,box(2),[A]),can) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-20',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-20',1,ref(4),[A]),year,countable) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-20',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-20',1,ref(4),[A]),na,geq,2) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_unit('$guideline_id'(context,'cdc2022-opioid-s38-20',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-20',1,ref(4),[A]),year) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s38-20',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-20',1,ref(5),[A]),continue) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-20',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-20',1,ref(5),[A]),1,A) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s38-20',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-20',1,ref(5),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s38-20',1,ref(4),[A])) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_interval('$guideline_id'(context,'cdc2022-opioid-s38-20',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-20',1,ref(5),[A]),duration,'$guideline_id'(product,'cdc2022-opioid-s38-20',1,ref(4),[A]),year,none) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every opioid-taper-duration depends on an opioid-dosage.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s38-20',2,ref(2),[A]),'opioid-dosage',countable) :- guideline_entity(actual,A,'opioid-taper-duration',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s38-20',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-taper-duration',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s38-20',2,ref(3),[A]),depend) :- guideline_entity(actual,A,'opioid-taper-duration',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s38-20',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-taper-duration',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s38-20',2,ref(3),[A]),on,'$guideline_id'(product,'cdc2022-opioid-s38-20',2,ref(2),[A])) :- guideline_entity(actual,A,'opioid-taper-duration',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should individualize an opioid-taper with a patient-goal-and-concern.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s38-20',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-20',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-20',3,ref(2),[A]),'opioid-taper',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-20',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-20',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-20',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-20',3,ref(3),[A]),'patient-goal-and-concern',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-20',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-20',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s38-20',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-20',3,ref(4),[A]),individualize) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-20',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-20',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-20',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-20',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s38-20',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s38-20',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-20',3,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s38-20',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
