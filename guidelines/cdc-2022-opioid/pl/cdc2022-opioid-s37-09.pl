% cdc2022-opioid-s37-09.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_schema_version(1).
guideline_document('cdc2022-opioid-s37-09',ace_sha256('6216aff909eeb21063ee8349335c4ef9f6d83f827ae553c60a16aa76de07b801'),ulex(sha256('6028ca436ba3763c4e3a5f23e316b6a1b582cdb7dc76e395f1787016efb02bee')),vocabulary(sha256(c1f07c5282df4f4d69b5a8c2ef4b1fbe8beae14dbb75f5931ece77ea612f2d95))).
% S1: If a patient is pregnant and has an opioid-use-disorder then an opioid-use-disorder-medication is preferred instead-of a withdrawal-management for the opioid-use-disorder.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(6),[A,B,C,D,E]),'opioid-use-disorder-medication',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,pregnant,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'opioid-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(6),[A,B,C,D,E]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,pregnant,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'opioid-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(7),[A,B,C,D,E]),'withdrawal-management',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,pregnant,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'opioid-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(7),[A,B,C,D,E]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,pregnant,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'opioid-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_property(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(8),[A,B,C,D,E]),preferred,pos) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,pregnant,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'opioid-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(9),[A,B,C,D,E]),be) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,pregnant,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'opioid-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(9),[A,B,C,D,E]),1,'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(6),[A,B,C,D,E])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,pregnant,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'opioid-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(9),[A,B,C,D,E]),2,'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(8),[A,B,C,D,E])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,pregnant,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'opioid-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(9),[A,B,C,D,E]),for,D) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,pregnant,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'opioid-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(9),[A,B,C,D,E]),'instead-of','$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(7),[A,B,C,D,E])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,pregnant,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'opioid-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_preference(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(9),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(6),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(7),[A,B,C,D,E])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,pregnant,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'opioid-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
