% cdc2022-opioid-s48-16.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s48-16',ace_sha256('89e431fda577a7e3e08ee278939dab9e7b0cd6e66d31dd3894daffc700552021'),ulex(sha256('45608095e3f77853fdabcce148a9eb7633a8ae776353b8f008c753b846886588')),vocabulary(sha256(c36289aa40f75805a83501ca057b29dd1ba7292b025716853f654890b77ec50c))).
% S1: If a patient has an anxiety-disorder then the patient carries a higher-benzodiazepine-receipt-likelihood.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s48-16',1,ref(4),[A,B,C]),'higher-benzodiazepine-receipt-likelihood',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'anxiety-disorder',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s48-16',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'anxiety-disorder',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s48-16',1,ref(5),[A,B,C]),carry) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'anxiety-disorder',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s48-16',1,ref(5),[A,B,C]),1,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'anxiety-disorder',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s48-16',1,ref(5),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s48-16',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'anxiety-disorder',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S2: If a patient has an other-mental-health-condition then the patient carries a higher-benzodiazepine-receipt-likelihood.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s48-16',2,ref(4),[A,B,C]),'higher-benzodiazepine-receipt-likelihood',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'other-mental-health-condition',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s48-16',2,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'other-mental-health-condition',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s48-16',2,ref(5),[A,B,C]),carry) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'other-mental-health-condition',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s48-16',2,ref(5),[A,B,C]),1,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'other-mental-health-condition',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s48-16',2,ref(5),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s48-16',2,ref(4),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'other-mental-health-condition',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S3: Every benzodiazepine can exacerbate an opioid-induced-respiratory-depression and can increase an overdose-risk.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s48-16',3,box(1),[A]),can) :- guideline_entity(actual,A,benzodiazepine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s48-16',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s48-16',3,ref(2),[A]),'opioid-induced-respiratory-depression',countable) :- guideline_entity(actual,A,benzodiazepine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s48-16',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s48-16',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,benzodiazepine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s48-16',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s48-16',3,ref(3),[A]),exacerbate) :- guideline_entity(actual,A,benzodiazepine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s48-16',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s48-16',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,benzodiazepine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s48-16',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s48-16',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s48-16',3,ref(2),[A])) :- guideline_entity(actual,A,benzodiazepine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s48-16',3,box(2),[A]),can) :- guideline_entity(actual,A,benzodiazepine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s48-16',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s48-16',3,ref(4),[A]),'overdose-risk',countable) :- guideline_entity(actual,A,benzodiazepine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s48-16',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s48-16',3,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,benzodiazepine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s48-16',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s48-16',3,ref(5),[A]),increase) :- guideline_entity(actual,A,benzodiazepine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s48-16',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s48-16',3,ref(5),[A]),1,A) :- guideline_entity(actual,A,benzodiazepine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s48-16',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s48-16',3,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s48-16',3,ref(4),[A])) :- guideline_entity(actual,A,benzodiazepine,countable), guideline_cardinality(actual,A,na,eq,1).
