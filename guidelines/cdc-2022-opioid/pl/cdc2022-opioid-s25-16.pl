% cdc2022-opioid-s25-16.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s25-16',ace_sha256('738bf49e203d4955d33031156fa62bb61b014f2c67f3139ada92cb022f3f46df'),ulex(sha256(de64e41d6ddfdcb89810d9fc156aa0ac54562519e3bd4b94b8f7324f773ceb91)),vocabulary(sha256('06621a3dca592ca4385262a4a36dd8184670cdaef31827cd8b224c8c870c2f7d'))).
% S1: If a patient is an older-adult and has a chronic-pain then every NSAID-use potentially can be inappropriate for the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s25-16',1,box(1),[A,B,C,D,E,F]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'older-adult',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'chronic-pain',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D), guideline_entity(actual,F,'NSAID-use',countable), guideline_cardinality(actual,F,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s25-16',1,box(1),[A,B,C,D,E,F]),'$guideline_id'(product,'cdc2022-opioid-s25-16',1,ref(8),[A,B,C,D,E,F]),possibility,potentially) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'older-adult',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'chronic-pain',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D), guideline_entity(actual,F,'NSAID-use',countable), guideline_cardinality(actual,F,na,eq,1).
guideline_property('$guideline_id'(context,'cdc2022-opioid-s25-16',1,box(1),[A,B,C,D,E,F]),'$guideline_id'(product,'cdc2022-opioid-s25-16',1,ref(7),[A,B,C,D,E,F]),inappropriate,pos) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'older-adult',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'chronic-pain',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D), guideline_entity(actual,F,'NSAID-use',countable), guideline_cardinality(actual,F,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s25-16',1,box(1),[A,B,C,D,E,F]),'$guideline_id'(product,'cdc2022-opioid-s25-16',1,ref(8),[A,B,C,D,E,F]),be) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'older-adult',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'chronic-pain',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D), guideline_entity(actual,F,'NSAID-use',countable), guideline_cardinality(actual,F,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s25-16',1,box(1),[A,B,C,D,E,F]),'$guideline_id'(product,'cdc2022-opioid-s25-16',1,ref(8),[A,B,C,D,E,F]),1,F) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'older-adult',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'chronic-pain',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D), guideline_entity(actual,F,'NSAID-use',countable), guideline_cardinality(actual,F,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s25-16',1,box(1),[A,B,C,D,E,F]),'$guideline_id'(product,'cdc2022-opioid-s25-16',1,ref(8),[A,B,C,D,E,F]),2,'$guideline_id'(product,'cdc2022-opioid-s25-16',1,ref(7),[A,B,C,D,E,F])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'older-adult',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'chronic-pain',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D), guideline_entity(actual,F,'NSAID-use',countable), guideline_cardinality(actual,F,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s25-16',1,box(1),[A,B,C,D,E,F]),'$guideline_id'(product,'cdc2022-opioid-s25-16',1,ref(8),[A,B,C,D,E,F]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'older-adult',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'chronic-pain',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D), guideline_entity(actual,F,'NSAID-use',countable), guideline_cardinality(actual,F,na,eq,1).
% S2: If a patient is an older-adult and has a chronic-pain then a prolonged-NSAID-use carries a higher-adverse-effect-risk for the patient.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s25-16',2,ref(6),[A,B,C,D,E]),'prolonged-NSAID-use',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'older-adult',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'chronic-pain',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s25-16',2,ref(6),[A,B,C,D,E]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'older-adult',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'chronic-pain',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s25-16',2,ref(7),[A,B,C,D,E]),'higher-adverse-effect-risk',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'older-adult',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'chronic-pain',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s25-16',2,ref(7),[A,B,C,D,E]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'older-adult',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'chronic-pain',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s25-16',2,ref(8),[A,B,C,D,E]),carry) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'older-adult',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'chronic-pain',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s25-16',2,ref(8),[A,B,C,D,E]),1,'$guideline_id'(product,'cdc2022-opioid-s25-16',2,ref(6),[A,B,C,D,E])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'older-adult',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'chronic-pain',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s25-16',2,ref(8),[A,B,C,D,E]),2,'$guideline_id'(product,'cdc2022-opioid-s25-16',2,ref(7),[A,B,C,D,E])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'older-adult',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'chronic-pain',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s25-16',2,ref(8),[A,B,C,D,E]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'older-adult',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'chronic-pain',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
