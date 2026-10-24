% cdc2022-opioid-s31-12.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s31-12',ace_sha256('17408f049739531ff747422c6b1fcd86aabbb411a5d75f8f1bb2453d38fb2792'),ulex(sha256('45608095e3f77853fdabcce148a9eb7633a8ae776353b8f008c753b846886588')),vocabulary(sha256(c36289aa40f75805a83501ca057b29dd1ba7292b025716853f654890b77ec50c))).
% S1: Every transdermal-fentanyl-dosing uses a microgram-per-hour-unit.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s31-12',1,ref(2),[A]),'microgram-per-hour-unit',countable) :- guideline_entity(actual,A,'transdermal-fentanyl-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s31-12',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'transdermal-fentanyl-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s31-12',1,ref(3),[A]),use) :- guideline_entity(actual,A,'transdermal-fentanyl-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s31-12',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'transdermal-fentanyl-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s31-12',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s31-12',1,ref(2),[A])) :- guideline_entity(actual,A,'transdermal-fentanyl-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every microgram-per-hour-unit is an atypical-outpatient-drug-unit.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s31-12',2,ref(2),[A]),'atypical-outpatient-drug-unit',countable) :- guideline_entity(actual,A,'microgram-per-hour-unit',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s31-12',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'microgram-per-hour-unit',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s31-12',2,ref(3),[A]),be) :- guideline_entity(actual,A,'microgram-per-hour-unit',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s31-12',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'microgram-per-hour-unit',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s31-12',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s31-12',2,ref(2),[A])) :- guideline_entity(actual,A,'microgram-per-hour-unit',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every transdermal-fentanyl-dosing can be confusing.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s31-12',3,box(1),[A]),can) :- guideline_entity(actual,A,'transdermal-fentanyl-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_property('$guideline_id'(context,'cdc2022-opioid-s31-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s31-12',3,ref(2),[A]),confusing,pos) :- guideline_entity(actual,A,'transdermal-fentanyl-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s31-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s31-12',3,ref(3),[A]),be) :- guideline_entity(actual,A,'transdermal-fentanyl-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s31-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s31-12',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,'transdermal-fentanyl-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s31-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s31-12',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s31-12',3,ref(2),[A])) :- guideline_entity(actual,A,'transdermal-fentanyl-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
