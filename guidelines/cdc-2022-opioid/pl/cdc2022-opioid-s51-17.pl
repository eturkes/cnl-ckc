% cdc2022-opioid-s51-17.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s51-17',ace_sha256(ad8caf3ea0043c766cf725d0353109161631d5caf09059656a393e052269c831),ulex(sha256('45608095e3f77853fdabcce148a9eb7633a8ae776353b8f008c753b846886588')),vocabulary(sha256(c36289aa40f75805a83501ca057b29dd1ba7292b025716853f654890b77ec50c))).
% S1: If a patient does not provably have an otherwise-available-medication-history then a PDMP-data can be helpful for the patient.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s51-17',1,ref(4),[A]),'PDMP-data',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'otherwise-available-medication-history',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s51-17',1,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'otherwise-available-medication-history',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s51-17',1,box(1),[A]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'otherwise-available-medication-history',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_property('$guideline_id'(context,'cdc2022-opioid-s51-17',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-17',1,ref(5),[A]),helpful,pos) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'otherwise-available-medication-history',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s51-17',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-17',1,ref(6),[A]),be) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'otherwise-available-medication-history',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s51-17',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-17',1,ref(6),[A]),1,'$guideline_id'(product,'cdc2022-opioid-s51-17',1,ref(4),[A])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'otherwise-available-medication-history',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s51-17',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-17',1,ref(6),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s51-17',1,ref(5),[A])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'otherwise-available-medication-history',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s51-17',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s51-17',1,ref(6),[A]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'otherwise-available-medication-history',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
% S2: A medication-history-unavailability-example is a care-transition-to-a-new-clinician-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s51-17',2,ref(1),[]),'medication-history-unavailability-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s51-17',2,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s51-17',2,ref(2),[]),'care-transition-to-a-new-clinician-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s51-17',2,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s51-17',2,ref(3),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s51-17',2,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s51-17',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s51-17',2,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s51-17',2,ref(2),[])).
