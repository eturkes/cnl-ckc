% cdc2022-opioid-s23-06.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s23-06',ace_sha256('6e8a0ef3b635ec5667ad658cb039d8617f701ad8147dd9b8f10f691dabcd53f9'),ulex(sha256('45608095e3f77853fdabcce148a9eb7633a8ae776353b8f008c753b846886588')),vocabulary(sha256(c36289aa40f75805a83501ca057b29dd1ba7292b025716853f654890b77ec50c))).
% S1: If a patient uses a stool-softener and the patient does not provably use an additional-laxative then every clinician should avoid the stool-softener.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-06',1,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'stool-softener',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), \+ (guideline_entity(actual,E,'additional-laxative',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,use), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,E)), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-06',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s23-06',1,ref(7),[A,B,C,D]),avoid) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'stool-softener',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), \+ (guideline_entity(actual,E,'additional-laxative',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,use), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,E)), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-06',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s23-06',1,ref(7),[A,B,C,D]),1,D) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'stool-softener',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), \+ (guideline_entity(actual,E,'additional-laxative',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,use), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,E)), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-06',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s23-06',1,ref(7),[A,B,C,D]),2,B) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'stool-softener',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), \+ (guideline_entity(actual,E,'additional-laxative',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,use), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,E)), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
% S2: If a patient uses a fiber-laxative and the patient does not provably use an additional-laxative then every clinician should avoid the fiber-laxative.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-06',2,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'fiber-laxative',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), \+ (guideline_entity(actual,E,'additional-laxative',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,use), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,E)), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-06',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s23-06',2,ref(7),[A,B,C,D]),avoid) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'fiber-laxative',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), \+ (guideline_entity(actual,E,'additional-laxative',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,use), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,E)), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-06',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s23-06',2,ref(7),[A,B,C,D]),1,D) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'fiber-laxative',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), \+ (guideline_entity(actual,E,'additional-laxative',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,use), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,E)), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-06',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s23-06',2,ref(7),[A,B,C,D]),2,B) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'fiber-laxative',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), \+ (guideline_entity(actual,E,'additional-laxative',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,use), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,E)), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
