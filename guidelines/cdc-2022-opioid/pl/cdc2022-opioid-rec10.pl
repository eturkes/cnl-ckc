% cdc2022-opioid-rec10.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec10',ace_sha256('3579624ea87b3150510d00a8aae6f1dda5976c4b69880e99fd79ef7d7dab73aa'),ulex(sha256('3210e31a7a1a232faae4aed00d06481f6bef71ebd89b6c884183bc530be9986e')),vocabulary(sha256(b5cff67a2f9de9bee846e9e7650affaf95276ff3a26fc00befd8ceb076b0f4ac))).
% S1: A recommendation is a category-B-recommendation and is an evidence-type-4-recommendation.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec10',1,ref(1),[]),recommendation,countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec10',1,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec10',1,ref(2),[]),'category-B-recommendation',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec10',1,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec10',1,ref(3),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec10',1,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec10',1,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec10',1,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec10',1,ref(2),[])).
guideline_grade(actual,'$guideline_id'(product,'cdc2022-opioid-rec10',1,ref(1),[]),strength,conditional,'$guideline_id'(product,'cdc2022-opioid-rec10',1,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec10',1,ref(4),[]),'evidence-type-4-recommendation',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec10',1,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec10',1,ref(5),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec10',1,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec10',1,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec10',1,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec10',1,ref(4),[])).
guideline_grade(actual,'$guideline_id'(product,'cdc2022-opioid-rec10',1,ref(1),[]),evidence,very_low,'$guideline_id'(product,'cdc2022-opioid-rec10',1,ref(4),[])).
% S2: If a clinician prescribes an opioid for a subacute-pain then the clinician should consider a testing-benefit and should consider a testing-risk.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10',2,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'subacute-pain',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10',2,ref(5),[A,B,C,D]),'testing-benefit',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'subacute-pain',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10',2,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'subacute-pain',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10',2,ref(6),[A,B,C,D]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'subacute-pain',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10',2,ref(6),[A,B,C,D]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'subacute-pain',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10',2,ref(6),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-rec10',2,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'subacute-pain',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10',2,box(2),[A,B,C,D]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'subacute-pain',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10',2,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10',2,ref(7),[A,B,C,D]),'testing-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'subacute-pain',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10',2,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10',2,ref(7),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'subacute-pain',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10',2,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10',2,ref(8),[A,B,C,D]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'subacute-pain',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10',2,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10',2,ref(8),[A,B,C,D]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'subacute-pain',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10',2,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10',2,ref(8),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-rec10',2,ref(7),[A,B,C,D])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'subacute-pain',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
% S3: If a clinician prescribes an opioid for a chronic-pain then the clinician should consider a testing-benefit and should consider a testing-risk.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10',3,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'chronic-pain',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10',3,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10',3,ref(5),[A,B,C,D]),'testing-benefit',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'chronic-pain',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10',3,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10',3,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'chronic-pain',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10',3,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10',3,ref(6),[A,B,C,D]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'chronic-pain',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10',3,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10',3,ref(6),[A,B,C,D]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'chronic-pain',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10',3,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10',3,ref(6),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-rec10',3,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'chronic-pain',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10',3,box(2),[A,B,C,D]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'chronic-pain',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10',3,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10',3,ref(7),[A,B,C,D]),'testing-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'chronic-pain',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10',3,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10',3,ref(7),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'chronic-pain',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10',3,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10',3,ref(8),[A,B,C,D]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'chronic-pain',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10',3,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10',3,ref(8),[A,B,C,D]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'chronic-pain',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10',3,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10',3,ref(8),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-rec10',3,ref(7),[A,B,C,D])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'chronic-pain',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
% S4: A toxicology-testing can assess a prescribed-medication and can assess a prescribed-controlled-substance and can assess a nonprescribed-controlled-substance.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec10',4,ref(1),[]),'toxicology-testing',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec10',4,ref(1),[]),na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10',4,box(1),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10',4,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec10',4,ref(2),[]),'prescribed-medication',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10',4,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec10',4,ref(2),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10',4,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec10',4,ref(3),[]),assess).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10',4,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec10',4,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec10',4,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10',4,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec10',4,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec10',4,ref(2),[])).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10',4,box(2),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10',4,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec10',4,ref(4),[]),'prescribed-controlled-substance',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10',4,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec10',4,ref(4),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10',4,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec10',4,ref(5),[]),assess).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10',4,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec10',4,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec10',4,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10',4,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec10',4,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec10',4,ref(4),[])).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10',4,box(3),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10',4,box(3),[]),'$guideline_id'(product,'cdc2022-opioid-rec10',4,ref(6),[]),'nonprescribed-controlled-substance',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10',4,box(3),[]),'$guideline_id'(product,'cdc2022-opioid-rec10',4,ref(6),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10',4,box(3),[]),'$guideline_id'(product,'cdc2022-opioid-rec10',4,ref(7),[]),assess).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10',4,box(3),[]),'$guideline_id'(product,'cdc2022-opioid-rec10',4,ref(7),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec10',4,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10',4,box(3),[]),'$guideline_id'(product,'cdc2022-opioid-rec10',4,ref(7),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec10',4,ref(6),[])).
