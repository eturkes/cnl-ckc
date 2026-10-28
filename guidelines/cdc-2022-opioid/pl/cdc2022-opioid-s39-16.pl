% cdc2022-opioid-s39-16.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s39-16',ace_sha256('0914e0ca3c582a2239e0958ed321d662f9b86ab2fb7068a1f5e251b9efafc40b'),ulex(sha256(de64e41d6ddfdcb89810d9fc156aa0ac54562519e3bd4b94b8f7324f773ceb91)),vocabulary(sha256('06621a3dca592ca4385262a4a36dd8184670cdaef31827cd8b224c8c870c2f7d'))).
% S1: Every clinician should ensure a mild-to-moderate-full-agonist-opioid-withdrawal for a patient before a first-buprenorphine-dose.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s39-16',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(2),[A]),'mild-to-moderate-full-agonist-opioid-withdrawal',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(4),[A]),'first-buprenorphine-dose',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s39-16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(5),[A]),ensure) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s39-16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(5),[A]),before,'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_order('$guideline_id'(context,'cdc2022-opioid-s39-16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(5),[A]),before,'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s39-16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(5),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: If a patient takes a full-agonist-opioid and the patient does not have a mild-to-moderate-full-agonist-opioid-withdrawal and there is a clinician then it is recommended that the clinician does not conduct a standard-buprenorphine-initiation for the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s39-16',2,box(2),[A,B,C,D]),should) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'full-agonist-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,take), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(E,F,'mild-to-moderate-full-agonist-opioid-withdrawal',countable), guideline_operator(actual,E,-), guideline_cardinality(E,F,na,eq,1), guideline_event(E,G,have), guideline_arg(E,G,1,A), guideline_arg(E,G,2,F), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-s39-16',2,box(2),[A,B,C,D]),'$guideline_id'(context,'cdc2022-opioid-s39-16',2,box(3),[A,B,C,D]),-) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'full-agonist-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,take), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(E,F,'mild-to-moderate-full-agonist-opioid-withdrawal',countable), guideline_operator(actual,E,-), guideline_cardinality(E,F,na,eq,1), guideline_event(E,G,have), guideline_arg(E,G,1,A), guideline_arg(E,G,2,F), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-16',2,box(3),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s39-16',2,ref(7),[A,B,C,D]),'standard-buprenorphine-initiation',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'full-agonist-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,take), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(E,F,'mild-to-moderate-full-agonist-opioid-withdrawal',countable), guideline_operator(actual,E,-), guideline_cardinality(E,F,na,eq,1), guideline_event(E,G,have), guideline_arg(E,G,1,A), guideline_arg(E,G,2,F), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-16',2,box(3),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s39-16',2,ref(7),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'full-agonist-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,take), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(E,F,'mild-to-moderate-full-agonist-opioid-withdrawal',countable), guideline_operator(actual,E,-), guideline_cardinality(E,F,na,eq,1), guideline_event(E,G,have), guideline_arg(E,G,1,A), guideline_arg(E,G,2,F), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s39-16',2,box(3),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s39-16',2,ref(8),[A,B,C,D]),conduct) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'full-agonist-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,take), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(E,F,'mild-to-moderate-full-agonist-opioid-withdrawal',countable), guideline_operator(actual,E,-), guideline_cardinality(E,F,na,eq,1), guideline_event(E,G,have), guideline_arg(E,G,1,A), guideline_arg(E,G,2,F), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-16',2,box(3),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s39-16',2,ref(8),[A,B,C,D]),1,D) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'full-agonist-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,take), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(E,F,'mild-to-moderate-full-agonist-opioid-withdrawal',countable), guideline_operator(actual,E,-), guideline_cardinality(E,F,na,eq,1), guideline_event(E,G,have), guideline_arg(E,G,1,A), guideline_arg(E,G,2,F), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-16',2,box(3),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s39-16',2,ref(8),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-s39-16',2,ref(7),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'full-agonist-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,take), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(E,F,'mild-to-moderate-full-agonist-opioid-withdrawal',countable), guideline_operator(actual,E,-), guideline_cardinality(E,F,na,eq,1), guideline_event(E,G,have), guideline_arg(E,G,1,A), guideline_arg(E,G,2,F), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s39-16',2,box(3),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s39-16',2,ref(8),[A,B,C,D]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'full-agonist-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,take), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(E,F,'mild-to-moderate-full-agonist-opioid-withdrawal',countable), guideline_operator(actual,E,-), guideline_cardinality(E,F,na,eq,1), guideline_event(E,G,have), guideline_arg(E,G,1,A), guideline_arg(E,G,2,F), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
