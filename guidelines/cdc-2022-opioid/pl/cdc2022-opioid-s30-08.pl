% cdc2022-opioid-s30-08.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s30-08',ace_sha256('379ec153de6e7b144285e32fc6a37296d79222597f1808cbf3505385473615e1'),ulex(sha256(be24cf56c59049d3c31fba641013faa22821ecda0669443a1fc4a49979ef57ae)),temporal(sha256('29a90281976bb48dfa32d9ef80651076896404c524c9bf1d7a55b06c1880be2a'))).
% S1: Every clinician should heed a medication-combination-caution during a medication-combination-use.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s30-08',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',1,ref(2),[A]),'medication-combination-caution',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',1,ref(3),[A]),'medication-combination-use',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s30-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',1,ref(4),[A]),heed) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s30-08',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s30-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',1,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s30-08',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should avoid a synergistic-medication-risk during a medication-combination-use.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s30-08',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-08',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',2,ref(2),[A]),'synergistic-medication-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-08',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-08',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',2,ref(3),[A]),'medication-combination-use',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-08',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s30-08',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',2,ref(4),[A]),avoid) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-08',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-08',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s30-08',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s30-08',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',2,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s30-08',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
