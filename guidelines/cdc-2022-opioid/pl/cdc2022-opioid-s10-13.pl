% cdc2022-opioid-s10-13.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s10-13',ace_sha256('86a3e4cd8dc9386df1ff7b667040209466505d877f876b91067f2829a7d676e2'),ulex(sha256('64c086862c2f48d491a9fde0ba9f0fcf090e25e84456b9749332ec3ace211ce4')),vocabulary(sha256('06621a3dca592ca4385262a4a36dd8184670cdaef31827cd8b224c8c870c2f7d'))).
% S1: Every opioid-pain-clinical-practice-guideline follows an Institute-of-Medicine-palliative-care-definition.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s10-13',1,ref(2),[A]),'Institute-of-Medicine-palliative-care-definition',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s10-13',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s10-13',1,ref(3),[A]),follow) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s10-13',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s10-13',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s10-13',1,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every palliative-care provides a pain-and-other-symptom-relief.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s10-13',2,ref(2),[A]),'pain-and-other-symptom-relief',countable) :- guideline_entity(actual,A,'palliative-care',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s10-13',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'palliative-care',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s10-13',2,ref(3),[A]),provide) :- guideline_entity(actual,A,'palliative-care',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s10-13',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'palliative-care',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s10-13',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s10-13',2,ref(2),[A])) :- guideline_entity(actual,A,'palliative-care',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every palliative-care supports a patient-quality-of-life.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s10-13',3,ref(2),[A]),'patient-quality-of-life',countable) :- guideline_entity(actual,A,'palliative-care',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s10-13',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'palliative-care',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s10-13',3,ref(3),[A]),support) :- guideline_entity(actual,A,'palliative-care',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s10-13',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,'palliative-care',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s10-13',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s10-13',3,ref(2),[A])) :- guideline_entity(actual,A,'palliative-care',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every palliative-care focuses on a serious-advanced-illness-patient-group.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s10-13',4,ref(2),[A]),'serious-advanced-illness-patient-group',countable) :- guideline_entity(actual,A,'palliative-care',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s10-13',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'palliative-care',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s10-13',4,ref(3),[A]),focus) :- guideline_entity(actual,A,'palliative-care',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s10-13',4,ref(3),[A]),1,A) :- guideline_entity(actual,A,'palliative-care',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s10-13',4,ref(3),[A]),on,'$guideline_id'(product,'cdc2022-opioid-s10-13',4,ref(2),[A])) :- guideline_entity(actual,A,'palliative-care',countable), guideline_cardinality(actual,A,na,eq,1).
% S5: If a patient has a serious-advanced-illness and receives a care that provides a pain-and-other-symptom-relief and that supports a patient-quality-of-life then the patient receives a palliative-care.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s10-13',5,ref(10),[A,B,C,D,E,F,G,H,I]),'palliative-care',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'serious-advanced-illness',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,care,countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,'pain-and-other-symptom-relief',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,provide), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E), guideline_entity(actual,G,'patient-quality-of-life',countable), guideline_cardinality(actual,G,na,eq,1), guideline_event(actual,H,support), guideline_arg(actual,H,1,D), guideline_arg(actual,H,2,G), guideline_event(actual,I,receive), guideline_arg(actual,I,1,A), guideline_arg(actual,I,2,D).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s10-13',5,ref(10),[A,B,C,D,E,F,G,H,I]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'serious-advanced-illness',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,care,countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,'pain-and-other-symptom-relief',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,provide), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E), guideline_entity(actual,G,'patient-quality-of-life',countable), guideline_cardinality(actual,G,na,eq,1), guideline_event(actual,H,support), guideline_arg(actual,H,1,D), guideline_arg(actual,H,2,G), guideline_event(actual,I,receive), guideline_arg(actual,I,1,A), guideline_arg(actual,I,2,D).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s10-13',5,ref(11),[A,B,C,D,E,F,G,H,I]),receive) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'serious-advanced-illness',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,care,countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,'pain-and-other-symptom-relief',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,provide), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E), guideline_entity(actual,G,'patient-quality-of-life',countable), guideline_cardinality(actual,G,na,eq,1), guideline_event(actual,H,support), guideline_arg(actual,H,1,D), guideline_arg(actual,H,2,G), guideline_event(actual,I,receive), guideline_arg(actual,I,1,A), guideline_arg(actual,I,2,D).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s10-13',5,ref(11),[A,B,C,D,E,F,G,H,I]),1,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'serious-advanced-illness',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,care,countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,'pain-and-other-symptom-relief',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,provide), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E), guideline_entity(actual,G,'patient-quality-of-life',countable), guideline_cardinality(actual,G,na,eq,1), guideline_event(actual,H,support), guideline_arg(actual,H,1,D), guideline_arg(actual,H,2,G), guideline_event(actual,I,receive), guideline_arg(actual,I,1,A), guideline_arg(actual,I,2,D).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s10-13',5,ref(11),[A,B,C,D,E,F,G,H,I]),2,'$guideline_id'(product,'cdc2022-opioid-s10-13',5,ref(10),[A,B,C,D,E,F,G,H,I])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'serious-advanced-illness',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,care,countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,'pain-and-other-symptom-relief',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,provide), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E), guideline_entity(actual,G,'patient-quality-of-life',countable), guideline_cardinality(actual,G,na,eq,1), guideline_event(actual,H,support), guideline_arg(actual,H,1,D), guideline_arg(actual,H,2,G), guideline_event(actual,I,receive), guideline_arg(actual,I,1,A), guideline_arg(actual,I,2,D).
