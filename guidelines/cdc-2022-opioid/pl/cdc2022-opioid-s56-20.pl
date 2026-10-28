% cdc2022-opioid-s56-20.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s56-20',ace_sha256(eeec19eaee69d605fed650fe4ffe345e5c49013b2442068bc0c81795183e539b),ulex(sha256(de64e41d6ddfdcb89810d9fc156aa0ac54562519e3bd4b94b8f7324f773ceb91)),vocabulary(sha256('06621a3dca592ca4385262a4a36dd8184670cdaef31827cd8b224c8c870c2f7d'))).
% S1: Every medication-taper-sequencing-decision includes a patient-priority-consideration.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',1,ref(2),[A]),'patient-priority-consideration',countable) :- guideline_entity(actual,A,'medication-taper-sequencing-decision',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'medication-taper-sequencing-decision',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',1,ref(3),[A]),include) :- guideline_entity(actual,A,'medication-taper-sequencing-decision',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'medication-taper-sequencing-decision',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s56-20',1,ref(2),[A])) :- guideline_entity(actual,A,'medication-taper-sequencing-decision',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every medication-taper-sequencing-decision includes a patient-clinical-consideration.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',2,ref(2),[A]),'patient-clinical-consideration',countable) :- guideline_entity(actual,A,'medication-taper-sequencing-decision',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'medication-taper-sequencing-decision',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',2,ref(3),[A]),include) :- guideline_entity(actual,A,'medication-taper-sequencing-decision',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'medication-taper-sequencing-decision',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s56-20',2,ref(2),[A])) :- guideline_entity(actual,A,'medication-taper-sequencing-decision',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every medication-taper-sequencing-decision includes a therapeutic-change-response-consideration.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',3,ref(2),[A]),'therapeutic-change-response-consideration',countable) :- guideline_entity(actual,A,'medication-taper-sequencing-decision',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'medication-taper-sequencing-decision',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',3,ref(3),[A]),include) :- guideline_entity(actual,A,'medication-taper-sequencing-decision',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,'medication-taper-sequencing-decision',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s56-20',3,ref(2),[A])) :- guideline_entity(actual,A,'medication-taper-sequencing-decision',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every medication-taper-sequencing-decision includes an other-managing-clinician-consultation.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',4,ref(2),[A]),'other-managing-clinician-consultation',countable) :- guideline_entity(actual,A,'medication-taper-sequencing-decision',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'medication-taper-sequencing-decision',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',4,ref(3),[A]),include) :- guideline_entity(actual,A,'medication-taper-sequencing-decision',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',4,ref(3),[A]),1,A) :- guideline_entity(actual,A,'medication-taper-sequencing-decision',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',4,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s56-20',4,ref(2),[A])) :- guideline_entity(actual,A,'medication-taper-sequencing-decision',countable), guideline_cardinality(actual,A,na,eq,1).
% S5: If an other-specialist-consultation is needed then every medication-taper-sequencing-decision includes the other-specialist-consultation.
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',5,ref(5),[A,B,C,D]),include) :- guideline_entity(actual,A,'other-specialist-consultation',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,needed,pos), guideline_qualifier(actual,C,applicability,needed), guideline_entity(actual,D,'medication-taper-sequencing-decision',countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',5,ref(5),[A,B,C,D]),1,D) :- guideline_entity(actual,A,'other-specialist-consultation',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,needed,pos), guideline_qualifier(actual,C,applicability,needed), guideline_entity(actual,D,'medication-taper-sequencing-decision',countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',5,ref(5),[A,B,C,D]),2,A) :- guideline_entity(actual,A,'other-specialist-consultation',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,needed,pos), guideline_qualifier(actual,C,applicability,needed), guideline_entity(actual,D,'medication-taper-sequencing-decision',countable), guideline_cardinality(actual,D,na,eq,1).
% S6: An other-specialist-example is an addiction-specialist-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',6,ref(1),[]),'other-specialist-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',6,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',6,ref(2),[]),'addiction-specialist-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',6,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',6,ref(3),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',6,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s56-20',6,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s56-20',6,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s56-20',6,ref(2),[])).
