% cdc2022-opioid-s19-15.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s19-15',ace_sha256('9b9287854e3116dde156c3adb1f752efb6df097a5830b2ab6c886cd866110bb1'),ulex(sha256(de64e41d6ddfdcb89810d9fc156aa0ac54562519e3bd4b94b8f7324f773ceb91)),vocabulary(sha256('06621a3dca592ca4385262a4a36dd8184670cdaef31827cd8b224c8c870c2f7d'))).
% S1: Every nonopioid-therapy shows an at-least-opioid-equal-acute-low-back-pain-effectiveness.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',1,ref(2),[A]),'at-least-opioid-equal-acute-low-back-pain-effectiveness',countable) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',1,ref(3),[A]),show) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s19-15',1,ref(2),[A])) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every nonopioid-therapy shows an at-least-opioid-equal-acute-neck-pain-effectiveness.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',2,ref(2),[A]),'at-least-opioid-equal-acute-neck-pain-effectiveness',countable) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',2,ref(3),[A]),show) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s19-15',2,ref(2),[A])) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every nonopioid-therapy shows an at-least-opioid-equal-other-musculoskeletal-injury-pain-effectiveness.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',3,ref(2),[A]),'at-least-opioid-equal-other-musculoskeletal-injury-pain-effectiveness',countable) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',3,ref(3),[A]),show) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s19-15',3,ref(2),[A])) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every nonopioid-therapy shows an at-least-opioid-equal-minimal-injury-minor-surgery-pain-effectiveness.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',4,ref(2),[A]),'at-least-opioid-equal-minimal-injury-minor-surgery-pain-effectiveness',countable) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',4,ref(3),[A]),show) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',4,ref(3),[A]),1,A) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',4,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s19-15',4,ref(2),[A])) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every nonopioid-therapy shows an at-least-opioid-equal-acute-dental-pain-effectiveness.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',5,ref(2),[A]),'at-least-opioid-equal-acute-dental-pain-effectiveness',countable) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',5,ref(3),[A]),show) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',5,ref(3),[A]),1,A) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',5,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s19-15',5,ref(2),[A])) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
% S6: Every nonopioid-therapy shows an at-least-opioid-equal-kidney-stone-pain-effectiveness.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',6,ref(2),[A]),'at-least-opioid-equal-kidney-stone-pain-effectiveness',countable) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',6,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',6,ref(3),[A]),show) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',6,ref(3),[A]),1,A) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',6,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s19-15',6,ref(2),[A])) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
% S7: Every nonopioid-therapy shows an at-least-opioid-equal-acute-headache-effectiveness.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',7,ref(2),[A]),'at-least-opioid-equal-acute-headache-effectiveness',countable) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',7,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',7,ref(3),[A]),show) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',7,ref(3),[A]),1,A) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',7,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s19-15',7,ref(2),[A])) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
% S8: Every nonopioid-therapy shows an at-least-opioid-equal-episodic-migraine-effectiveness.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',8,ref(2),[A]),'at-least-opioid-equal-episodic-migraine-effectiveness',countable) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',8,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',8,ref(3),[A]),show) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',8,ref(3),[A]),1,A) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',8,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s19-15',8,ref(2),[A])) :- guideline_entity(actual,A,'nonopioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
% S9: A musculoskeletal-injury-example includes a sprain-example and includes a strain-example and includes a tendonitis-example and includes a bursitis-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(1),[]),'musculoskeletal-injury-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(2),[]),'sprain-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(4),[]),'strain-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(4),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(6),[]),'tendonitis-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(6),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(7),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(7),[]),1,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(7),[]),2,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(6),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(8),[]),'bursitis-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(8),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(9),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(9),[]),1,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(9),[]),2,'$guideline_id'(product,'cdc2022-opioid-s19-15',9,ref(8),[])).
% S10: A minor-surgery-example is a simple-dental-extraction-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',10,ref(1),[]),'minor-surgery-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',10,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',10,ref(2),[]),'simple-dental-extraction-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',10,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',10,ref(3),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',10,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s19-15',10,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s19-15',10,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s19-15',10,ref(2),[])).
