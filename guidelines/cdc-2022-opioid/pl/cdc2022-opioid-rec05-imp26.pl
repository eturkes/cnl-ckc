% cdc2022-opioid-rec05-imp26.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec05-imp26',ace_sha256(cc22c80a2e2d7c5ef078f2ead5e85761e16bfd196940b426966dab687a597dc3),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: A communication-principle is relevant during a shorter-duration-opioid-discontinuation.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(1),[]),'communication-principle',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(2),[]),'shorter-duration-opioid-discontinuation',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(2),[]),na,eq,1).
guideline_property(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(3),[]),relevant,pos).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(4),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(4),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(4),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(3),[])).
guideline_qualifier(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(4),[]),applicability,relevant).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(4),[]),during,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(2),[])).
% S2: A pain-management-principle is relevant during a shorter-duration-opioid-discontinuation.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',2,ref(1),[]),'pain-management-principle',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',2,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',2,ref(2),[]),'shorter-duration-opioid-discontinuation',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',2,ref(2),[]),na,eq,1).
guideline_property(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',2,ref(3),[]),relevant,pos).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',2,ref(4),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',2,ref(4),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',2,ref(4),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',2,ref(3),[])).
guideline_qualifier(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',2,ref(4),[]),applicability,relevant).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',2,ref(4),[]),during,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',2,ref(2),[])).
% S3: A behavioral-support-principle is relevant during a shorter-duration-opioid-discontinuation.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',3,ref(1),[]),'behavioral-support-principle',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',3,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',3,ref(2),[]),'shorter-duration-opioid-discontinuation',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',3,ref(2),[]),na,eq,1).
guideline_property(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',3,ref(3),[]),relevant,pos).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',3,ref(4),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',3,ref(4),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',3,ref(4),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',3,ref(3),[])).
guideline_qualifier(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',3,ref(4),[]),applicability,relevant).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',3,ref(4),[]),during,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',3,ref(2),[])).
% S4: A slower-taper-principle is relevant during a shorter-duration-opioid-discontinuation.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',4,ref(1),[]),'slower-taper-principle',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',4,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',4,ref(2),[]),'shorter-duration-opioid-discontinuation',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',4,ref(2),[]),na,eq,1).
guideline_property(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',4,ref(3),[]),relevant,pos).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',4,ref(4),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',4,ref(4),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',4,ref(4),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',4,ref(3),[])).
guideline_qualifier(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',4,ref(4),[]),applicability,relevant).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',4,ref(4),[]),during,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',4,ref(2),[])).
% S5: If there is a recommendation-5 and there is a long-term-opioid-therapy and there is a subacute-pain then the recommendation-5 applies during the long-term-opioid-therapy for the subacute-pain.
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',5,ref(4),[A,B,C]),apply) :- guideline_entity(actual,A,'recommendation-5',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'subacute-pain',countable), guideline_cardinality(actual,C,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',5,ref(4),[A,B,C]),1,A) :- guideline_entity(actual,A,'recommendation-5',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'subacute-pain',countable), guideline_cardinality(actual,C,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',5,ref(4),[A,B,C]),for,C) :- guideline_entity(actual,A,'recommendation-5',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'subacute-pain',countable), guideline_cardinality(actual,C,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',5,ref(4),[A,B,C]),during,B) :- guideline_entity(actual,A,'recommendation-5',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'subacute-pain',countable), guideline_cardinality(actual,C,na,eq,1).
% S6: If there is a recommendation-5 and there is a long-term-opioid-therapy and there is a chronic-pain then the recommendation-5 applies during the long-term-opioid-therapy for the chronic-pain.
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',6,ref(4),[A,B,C]),apply) :- guideline_entity(actual,A,'recommendation-5',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'chronic-pain',countable), guideline_cardinality(actual,C,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',6,ref(4),[A,B,C]),1,A) :- guideline_entity(actual,A,'recommendation-5',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'chronic-pain',countable), guideline_cardinality(actual,C,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',6,ref(4),[A,B,C]),for,C) :- guideline_entity(actual,A,'recommendation-5',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'chronic-pain',countable), guideline_cardinality(actual,C,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',6,ref(4),[A,B,C]),during,B) :- guideline_entity(actual,A,'recommendation-5',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'chronic-pain',countable), guideline_cardinality(actual,C,na,eq,1).
