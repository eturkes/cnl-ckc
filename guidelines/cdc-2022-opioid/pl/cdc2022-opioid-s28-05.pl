% cdc2022-opioid-s28-05.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s28-05',ace_sha256('26bc9bf5a9c72759899ef5e8cc1964ea0654f4419b051e665d967e7c8fe8e145'),ulex(sha256(de64e41d6ddfdcb89810d9fc156aa0ac54562519e3bd4b94b8f7324f773ceb91)),vocabulary(sha256('06621a3dca592ca4385262a4a36dd8184670cdaef31827cd8b224c8c870c2f7d'))).
% S1: If a clinician does not provably consider an exit-strategy with a patient then it is recommended that the clinician does not initiate an opioid-therapy.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s28-05',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'exit-strategy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,consider), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C)), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-s28-05',1,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-s28-05',1,box(2),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'exit-strategy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,consider), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C)).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s28-05',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-05',1,ref(5),[A]),'opioid-therapy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'exit-strategy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,consider), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C)).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s28-05',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-05',1,ref(5),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'exit-strategy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,consider), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C)).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s28-05',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-05',1,ref(6),[A]),initiate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'exit-strategy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,consider), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C)).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-05',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-05',1,ref(6),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'exit-strategy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,consider), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C)).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-05',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s28-05',1,ref(6),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s28-05',1,ref(5),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'exit-strategy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,consider), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C)).
% S2: If an opioid-therapy is unsuccessful then every clinician can possibly use an exit-strategy with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s28-05',2,box(1),[A,B,C,D]),can) :- guideline_entity(actual,A,'opioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,unsuccessful,pos), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s28-05',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-05',2,ref(5),[A,B,C,D]),'exit-strategy',countable) :- guideline_entity(actual,A,'opioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,unsuccessful,pos), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s28-05',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-05',2,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,'opioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,unsuccessful,pos), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s28-05',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-05',2,ref(6),[A,B,C,D]),patient,countable) :- guideline_entity(actual,A,'opioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,unsuccessful,pos), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s28-05',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-05',2,ref(6),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,'opioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,unsuccessful,pos), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s28-05',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-05',2,ref(7),[A,B,C,D]),possibility,possibly) :- guideline_entity(actual,A,'opioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,unsuccessful,pos), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s28-05',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-05',2,ref(7),[A,B,C,D]),use) :- guideline_entity(actual,A,'opioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,unsuccessful,pos), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-05',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-05',2,ref(7),[A,B,C,D]),1,D) :- guideline_entity(actual,A,'opioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,unsuccessful,pos), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s28-05',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-05',2,ref(7),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-s28-05',2,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,'opioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,unsuccessful,pos), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s28-05',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s28-05',2,ref(7),[A,B,C,D]),with,'$guideline_id'(product,'cdc2022-opioid-s28-05',2,ref(6),[A,B,C,D])) :- guideline_entity(actual,A,'opioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,unsuccessful,pos), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
