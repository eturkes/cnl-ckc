% cdc2022-opioid-s40-19.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s40-19',ace_sha256(fedb85dc62cebf1353afd9aeecaf188cc3e46f82d86bd49810901da86d1a3a20),ulex(sha256('64c086862c2f48d491a9fde0ba9f0fcf090e25e84456b9749332ec3ace211ce4')),vocabulary(sha256('06621a3dca592ca4385262a4a36dd8184670cdaef31827cd8b224c8c870c2f7d'))).
% S1: If a patient has an injury-mechanism with an expected-prolonged-severe-pain-result then a longer-opioid-therapy-duration shows a higher-need-likelihood for the patient.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s40-19',1,ref(5),[A,B,C,D]),'longer-opioid-therapy-duration',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'injury-mechanism',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'expected-prolonged-severe-pain-result',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s40-19',1,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'injury-mechanism',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'expected-prolonged-severe-pain-result',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s40-19',1,ref(6),[A,B,C,D]),'higher-need-likelihood',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'injury-mechanism',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'expected-prolonged-severe-pain-result',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s40-19',1,ref(6),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'injury-mechanism',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'expected-prolonged-severe-pain-result',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s40-19',1,ref(7),[A,B,C,D]),show) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'injury-mechanism',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'expected-prolonged-severe-pain-result',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s40-19',1,ref(7),[A,B,C,D]),1,'$guideline_id'(product,'cdc2022-opioid-s40-19',1,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'injury-mechanism',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'expected-prolonged-severe-pain-result',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s40-19',1,ref(7),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-s40-19',1,ref(6),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'injury-mechanism',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'expected-prolonged-severe-pain-result',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s40-19',1,ref(7),[A,B,C,D]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'injury-mechanism',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'expected-prolonged-severe-pain-result',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
% S2: A prolonged-severe-pain-mechanism-example is a severe-traumatic-injury-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s40-19',2,ref(1),[]),'prolonged-severe-pain-mechanism-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s40-19',2,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s40-19',2,ref(2),[]),'severe-traumatic-injury-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s40-19',2,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s40-19',2,ref(3),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s40-19',2,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s40-19',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s40-19',2,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s40-19',2,ref(2),[])).
