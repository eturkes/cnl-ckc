% cdc2022-opioid-s62-03.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_schema_version(3).
guideline_document('cdc2022-opioid-s62-03',ace_sha256('9d7d09b930d9b6d8a94b6d4d19fdf0f99400d5f8e48095def64e1927f4dffa5c'),ulex(sha256(be24cf56c59049d3c31fba641013faa22821ecda0669443a1fc4a49979ef57ae)),temporal(sha256('29a90281976bb48dfa32d9ef80651076896404c524c9bf1d7a55b06c1880be2a'))).
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
