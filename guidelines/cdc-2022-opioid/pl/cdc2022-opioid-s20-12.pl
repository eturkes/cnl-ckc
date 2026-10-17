% cdc2022-opioid-s20-12.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_schema_version(1).
guideline_document('cdc2022-opioid-s20-12',ace_sha256(cfba05d27108376a39a379e679346c43c2cbeb5e1c7b15e7e6a0416247564df8),ulex(sha256(d08a8bd3e80ef6e01301a079bb5496315f35233bf199aa89b2d2ccc93b858776)),vocabulary(sha256('9d7632637554f8cbfe72e7d34a9b9d14c29eb8593aebb1c128a870c5dcf77ead'))).
% S1: If a patient has a kidney-stone-pain and the patient does not have an NSAID-contraindication then an NSAID is preferred for the kidney-stone-pain.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s20-12',1,ref(6),[A,B,C]),'NSAID',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'kidney-stone-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_operator(actual,D,-), guideline_entity(D,E,'NSAID-contraindication',countable), guideline_cardinality(D,E,na,eq,1), guideline_event(D,F,have), guideline_arg(D,F,1,A), guideline_arg(D,F,2,E).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s20-12',1,ref(6),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'kidney-stone-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_operator(actual,D,-), guideline_entity(D,E,'NSAID-contraindication',countable), guideline_cardinality(D,E,na,eq,1), guideline_event(D,F,have), guideline_arg(D,F,1,A), guideline_arg(D,F,2,E).
guideline_property(actual,'$guideline_id'(product,'cdc2022-opioid-s20-12',1,ref(7),[A,B,C]),preferred,pos) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'kidney-stone-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_operator(actual,D,-), guideline_entity(D,E,'NSAID-contraindication',countable), guideline_cardinality(D,E,na,eq,1), guideline_event(D,F,have), guideline_arg(D,F,1,A), guideline_arg(D,F,2,E).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s20-12',1,ref(8),[A,B,C]),be) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'kidney-stone-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_operator(actual,D,-), guideline_entity(D,E,'NSAID-contraindication',countable), guideline_cardinality(D,E,na,eq,1), guideline_event(D,F,have), guideline_arg(D,F,1,A), guideline_arg(D,F,2,E).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s20-12',1,ref(8),[A,B,C]),1,'$guideline_id'(product,'cdc2022-opioid-s20-12',1,ref(6),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'kidney-stone-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_operator(actual,D,-), guideline_entity(D,E,'NSAID-contraindication',countable), guideline_cardinality(D,E,na,eq,1), guideline_event(D,F,have), guideline_arg(D,F,1,A), guideline_arg(D,F,2,E).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s20-12',1,ref(8),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s20-12',1,ref(7),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'kidney-stone-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_operator(actual,D,-), guideline_entity(D,E,'NSAID-contraindication',countable), guideline_cardinality(D,E,na,eq,1), guideline_event(D,F,have), guideline_arg(D,F,1,A), guideline_arg(D,F,2,E).
guideline_preference(actual,'$guideline_id'(product,'cdc2022-opioid-s20-12',1,ref(8),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s20-12',1,ref(6),[A,B,C]),none) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'kidney-stone-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_operator(actual,D,-), guideline_entity(D,E,'NSAID-contraindication',countable), guideline_cardinality(D,E,na,eq,1), guideline_event(D,F,have), guideline_arg(D,F,1,A), guideline_arg(D,F,2,E).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s20-12',1,ref(8),[A,B,C]),for,B) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'kidney-stone-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_operator(actual,D,-), guideline_entity(D,E,'NSAID-contraindication',countable), guideline_cardinality(D,E,na,eq,1), guideline_event(D,F,have), guideline_arg(D,F,1,A), guideline_arg(D,F,2,E).
