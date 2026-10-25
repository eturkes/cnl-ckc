% cdc2022-opioid-s55-15.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
:- multifile(guideline_supersession/4).
:- discontiguous(guideline_supersession/4).
:- multifile(guideline_evaluation_date/1).
:- discontiguous(guideline_evaluation_date/1).
guideline_schema_version(1).
guideline_document('cdc2022-opioid-s55-15',ace_sha256('898a0ae35b592021c7d22f5713381cd501e24453957126a352f47d2397d5f3ad'),ulex(sha256('64c086862c2f48d491a9fde0ba9f0fcf090e25e84456b9749332ec3ace211ce4')),vocabulary(sha256(c36289aa40f75805a83501ca057b29dd1ba7292b025716853f654890b77ec50c))).
% S1: Every benzodiazepine causes a central-nervous-system-depression.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s55-15',1,ref(2),[A]),'central-nervous-system-depression',countable) :- guideline_entity(actual,A,benzodiazepine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s55-15',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,benzodiazepine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s55-15',1,ref(3),[A]),cause) :- guideline_entity(actual,A,benzodiazepine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s55-15',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,benzodiazepine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s55-15',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s55-15',1,ref(2),[A])) :- guideline_entity(actual,A,benzodiazepine,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every opioid causes a central-nervous-system-depression.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s55-15',2,ref(2),[A]),'central-nervous-system-depression',countable) :- guideline_entity(actual,A,opioid,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s55-15',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,opioid,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s55-15',2,ref(3),[A]),cause) :- guideline_entity(actual,A,opioid,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s55-15',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,opioid,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s55-15',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s55-15',2,ref(2),[A])) :- guideline_entity(actual,A,opioid,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every benzodiazepine can potentiate an opioid-induced-respiratory-drive-decrease.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s55-15',3,box(1),[A]),can) :- guideline_entity(actual,A,benzodiazepine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s55-15',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s55-15',3,ref(2),[A]),'opioid-induced-respiratory-drive-decrease',countable) :- guideline_entity(actual,A,benzodiazepine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s55-15',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s55-15',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,benzodiazepine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s55-15',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s55-15',3,ref(3),[A]),potentiate) :- guideline_entity(actual,A,benzodiazepine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s55-15',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s55-15',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,benzodiazepine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s55-15',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s55-15',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s55-15',3,ref(2),[A])) :- guideline_entity(actual,A,benzodiazepine,countable), guideline_cardinality(actual,A,na,eq,1).
