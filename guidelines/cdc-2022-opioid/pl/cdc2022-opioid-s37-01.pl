% cdc2022-opioid-s37-01.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s37-01',ace_sha256('21ab5f75427e54081e5e3915ed46ef66de6d179f27b524269533fecd9ff25dac'),ulex(sha256(de64e41d6ddfdcb89810d9fc156aa0ac54562519e3bd4b94b8f7324f773ceb91)),vocabulary(sha256('06621a3dca592ca4385262a4a36dd8184670cdaef31827cd8b224c8c870c2f7d'))).
% S1: Every opioid-benefit changes over a time.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s37-01',1,ref(2),[A]),time,countable) :- guideline_entity(actual,A,'opioid-benefit',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s37-01',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-benefit',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s37-01',1,ref(3),[A]),change) :- guideline_entity(actual,A,'opioid-benefit',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s37-01',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-benefit',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s37-01',1,ref(3),[A]),over,'$guideline_id'(product,'cdc2022-opioid-s37-01',1,ref(2),[A])) :- guideline_entity(actual,A,'opioid-benefit',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every opioid-risk changes over a time.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s37-01',2,ref(2),[A]),time,countable) :- guideline_entity(actual,A,'opioid-risk',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s37-01',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-risk',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s37-01',2,ref(3),[A]),change) :- guideline_entity(actual,A,'opioid-risk',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s37-01',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-risk',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s37-01',2,ref(3),[A]),over,'$guideline_id'(product,'cdc2022-opioid-s37-01',2,ref(2),[A])) :- guideline_entity(actual,A,'opioid-risk',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should periodically reevaluate an opioid-benefit and should periodically reevaluate an opioid-risk.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s37-01',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s37-01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-01',3,ref(2),[A]),'opioid-benefit',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s37-01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-01',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s37-01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-01',3,ref(3),[A]),usuality,periodically) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s37-01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-01',3,ref(3),[A]),reevaluate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-01',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-01',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s37-01',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s37-01',3,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s37-01',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-01',3,ref(4),[A]),'opioid-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s37-01',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-01',3,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s37-01',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-01',3,ref(5),[A]),usuality,periodically) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s37-01',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-01',3,ref(5),[A]),reevaluate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-01',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-01',3,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-01',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-01',3,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s37-01',3,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
