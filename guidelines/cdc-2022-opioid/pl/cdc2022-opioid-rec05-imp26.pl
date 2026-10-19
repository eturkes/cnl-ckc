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
guideline_schema_version(1).
guideline_document('cdc2022-opioid-rec05-imp26',ace_sha256(ef6880ecddcf8214eab4c60ed3ac4c8759459374cf812a854a9c7846dc65b6cf),ulex(sha256('3210e31a7a1a232faae4aed00d06481f6bef71ebd89b6c884183bc530be9986e')),vocabulary(sha256(b5cff67a2f9de9bee846e9e7650affaf95276ff3a26fc00befd8ceb076b0f4ac))).
% S1: A communication-principle is relevant during a shorter-duration-opioid-discontinuation.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(1),[]),'communication-principle',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(2),[]),'shorter-duration-opioid-discontinuation',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(2),[]),na,eq,1).
guideline_property(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(3),[]),relevant,pos).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(4),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(4),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(4),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(3),[])).
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
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',4,ref(4),[]),during,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',4,ref(2),[])).
% S5: A recommendation-5 applies during a long-term-opioid-therapy for a subacute-pain.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',5,ref(1),[]),'recommendation-5',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',5,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',5,ref(2),[]),'long-term-opioid-therapy',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',5,ref(2),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',5,ref(3),[]),'subacute-pain',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',5,ref(3),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',5,ref(4),[]),apply).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',5,ref(4),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',5,ref(1),[])).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',5,ref(4),[]),for,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',5,ref(3),[])).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',5,ref(4),[]),during,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',5,ref(2),[])).
% S6: A recommendation-5 applies during a long-term-opioid-therapy for a chronic-pain.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',6,ref(1),[]),'recommendation-5',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',6,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',6,ref(2),[]),'long-term-opioid-therapy',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',6,ref(2),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',6,ref(3),[]),'chronic-pain',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',6,ref(3),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',6,ref(4),[]),apply).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',6,ref(4),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',6,ref(1),[])).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',6,ref(4),[]),for,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',6,ref(3),[])).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',6,ref(4),[]),during,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',6,ref(2),[])).
