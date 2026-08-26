% cdc2022-opioid-s62-03.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
:- multifile(guideline_schema_version/1).
:- discontiguous(guideline_schema_version/1).
:- multifile(guideline_document/3).
:- discontiguous(guideline_document/3).
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
guideline_schema_version(1).
guideline_document('cdc2022-opioid-s62-03',ace_sha256('9d7d09b930d9b6d8a94b6d4d19fdf0f99400d5f8e48095def64e1927f4dffa5c'),ulex(sha256('4c55bed7525c11e80d870254a4f876c23b6aa3c2a077242833ec32997784b01c'))).
% S1: Every robust-evidence-based-treatment-coverage may facilitate an evidence-based-default-pain-treatment and may encourage an evidence-based-default-pain-treatment.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s62-03',1,box(1),[A]),may) :- guideline_entity(actual,A,'robust-evidence-based-treatment-coverage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s62-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',1,ref(2),[A]),'evidence-based-default-pain-treatment',countable) :- guideline_entity(actual,A,'robust-evidence-based-treatment-coverage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s62-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'robust-evidence-based-treatment-coverage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s62-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',1,ref(3),[A]),facilitate) :- guideline_entity(actual,A,'robust-evidence-based-treatment-coverage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s62-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'robust-evidence-based-treatment-coverage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s62-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s62-03',1,ref(2),[A])) :- guideline_entity(actual,A,'robust-evidence-based-treatment-coverage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s62-03',1,box(2),[A]),may) :- guideline_entity(actual,A,'robust-evidence-based-treatment-coverage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s62-03',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',1,ref(4),[A]),'evidence-based-default-pain-treatment',countable) :- guideline_entity(actual,A,'robust-evidence-based-treatment-coverage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s62-03',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',1,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'robust-evidence-based-treatment-coverage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s62-03',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',1,ref(5),[A]),encourage) :- guideline_entity(actual,A,'robust-evidence-based-treatment-coverage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s62-03',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',1,ref(5),[A]),1,A) :- guideline_entity(actual,A,'robust-evidence-based-treatment-coverage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s62-03',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',1,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s62-03',1,ref(4),[A])) :- guideline_entity(actual,A,'robust-evidence-based-treatment-coverage',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every robust-evidence-based-treatment-access may facilitate an evidence-based-default-pain-treatment and may encourage an evidence-based-default-pain-treatment.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s62-03',2,box(1),[A]),may) :- guideline_entity(actual,A,'robust-evidence-based-treatment-access',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s62-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',2,ref(2),[A]),'evidence-based-default-pain-treatment',countable) :- guideline_entity(actual,A,'robust-evidence-based-treatment-access',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s62-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'robust-evidence-based-treatment-access',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s62-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',2,ref(3),[A]),facilitate) :- guideline_entity(actual,A,'robust-evidence-based-treatment-access',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s62-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'robust-evidence-based-treatment-access',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s62-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s62-03',2,ref(2),[A])) :- guideline_entity(actual,A,'robust-evidence-based-treatment-access',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s62-03',2,box(2),[A]),may) :- guideline_entity(actual,A,'robust-evidence-based-treatment-access',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s62-03',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',2,ref(4),[A]),'evidence-based-default-pain-treatment',countable) :- guideline_entity(actual,A,'robust-evidence-based-treatment-access',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s62-03',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',2,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'robust-evidence-based-treatment-access',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s62-03',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',2,ref(5),[A]),encourage) :- guideline_entity(actual,A,'robust-evidence-based-treatment-access',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s62-03',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',2,ref(5),[A]),1,A) :- guideline_entity(actual,A,'robust-evidence-based-treatment-access',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s62-03',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',2,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s62-03',2,ref(4),[A])) :- guideline_entity(actual,A,'robust-evidence-based-treatment-access',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every pain-treatment-decision-support may facilitate an evidence-based-default-pain-treatment and may encourage an evidence-based-default-pain-treatment.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s62-03',3,box(1),[A]),may) :- guideline_entity(actual,A,'pain-treatment-decision-support',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s62-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',3,ref(2),[A]),'evidence-based-default-pain-treatment',countable) :- guideline_entity(actual,A,'pain-treatment-decision-support',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s62-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'pain-treatment-decision-support',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s62-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',3,ref(3),[A]),facilitate) :- guideline_entity(actual,A,'pain-treatment-decision-support',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s62-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,'pain-treatment-decision-support',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s62-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s62-03',3,ref(2),[A])) :- guideline_entity(actual,A,'pain-treatment-decision-support',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s62-03',3,box(2),[A]),may) :- guideline_entity(actual,A,'pain-treatment-decision-support',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s62-03',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',3,ref(4),[A]),'evidence-based-default-pain-treatment',countable) :- guideline_entity(actual,A,'pain-treatment-decision-support',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s62-03',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',3,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'pain-treatment-decision-support',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s62-03',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',3,ref(5),[A]),encourage) :- guideline_entity(actual,A,'pain-treatment-decision-support',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s62-03',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',3,ref(5),[A]),1,A) :- guideline_entity(actual,A,'pain-treatment-decision-support',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s62-03',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s62-03',3,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s62-03',3,ref(4),[A])) :- guideline_entity(actual,A,'pain-treatment-decision-support',countable), guideline_cardinality(actual,A,na,eq,1).
