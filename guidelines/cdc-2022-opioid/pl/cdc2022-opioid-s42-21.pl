% cdc2022-opioid-s42-21.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s42-21',ace_sha256(c9f49ef9a13ddf78c4b4f7f558d1ce1d15e199e8736bccdbc0ce1acd7960f30f),ulex(sha256('67d1410100ff45477e758b366ae03bb860a5c23e3fd444d55dc901b2f3c7d8a1')),vocabulary(sha256('9d7632637554f8cbfe72e7d34a9b9d14c29eb8593aebb1c128a870c5dcf77ead'))).
% S1: If an opioid is needed for a common-nonsurgical-acute-pain-cause then an opioid-therapy for a few-day-period often can be sufficient.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s42-21',1,ref(5),[A,B,C,D]),'opioid-therapy',countable) :- guideline_entity(actual,A,opioid,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'common-nonsurgical-acute-pain-cause',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,needed,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_qualifier(actual,D,applicability,needed), guideline_pp(actual,D,for,B).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s42-21',1,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,opioid,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'common-nonsurgical-acute-pain-cause',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,needed,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_qualifier(actual,D,applicability,needed), guideline_pp(actual,D,for,B).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s42-21',1,ref(6),[A,B,C,D]),'few-day-period',countable) :- guideline_entity(actual,A,opioid,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'common-nonsurgical-acute-pain-cause',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,needed,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_qualifier(actual,D,applicability,needed), guideline_pp(actual,D,for,B).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s42-21',1,ref(6),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,opioid,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'common-nonsurgical-acute-pain-cause',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,needed,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_qualifier(actual,D,applicability,needed), guideline_pp(actual,D,for,B).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s42-21',1,box(1),[A,B,C,D]),can) :- guideline_entity(actual,A,opioid,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'common-nonsurgical-acute-pain-cause',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,needed,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_qualifier(actual,D,applicability,needed), guideline_pp(actual,D,for,B).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s42-21',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s42-21',1,ref(7),[A,B,C,D]),usuality,often) :- guideline_entity(actual,A,opioid,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'common-nonsurgical-acute-pain-cause',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,needed,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_qualifier(actual,D,applicability,needed), guideline_pp(actual,D,for,B).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s42-21',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s42-21',1,ref(7),[A,B,C,D]),for,'$guideline_id'(product,'cdc2022-opioid-s42-21',1,ref(6),[A,B,C,D])) :- guideline_entity(actual,A,opioid,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'common-nonsurgical-acute-pain-cause',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,needed,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_qualifier(actual,D,applicability,needed), guideline_pp(actual,D,for,B).
guideline_property('$guideline_id'(context,'cdc2022-opioid-s42-21',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s42-21',1,ref(8),[A,B,C,D]),sufficient,pos) :- guideline_entity(actual,A,opioid,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'common-nonsurgical-acute-pain-cause',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,needed,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_qualifier(actual,D,applicability,needed), guideline_pp(actual,D,for,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s42-21',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s42-21',1,ref(7),[A,B,C,D]),be) :- guideline_entity(actual,A,opioid,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'common-nonsurgical-acute-pain-cause',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,needed,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_qualifier(actual,D,applicability,needed), guideline_pp(actual,D,for,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s42-21',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s42-21',1,ref(7),[A,B,C,D]),1,'$guideline_id'(product,'cdc2022-opioid-s42-21',1,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,opioid,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'common-nonsurgical-acute-pain-cause',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,needed,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_qualifier(actual,D,applicability,needed), guideline_pp(actual,D,for,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s42-21',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s42-21',1,ref(7),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-s42-21',1,ref(8),[A,B,C,D])) :- guideline_entity(actual,A,opioid,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'common-nonsurgical-acute-pain-cause',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,needed,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_qualifier(actual,D,applicability,needed), guideline_pp(actual,D,for,B).
% S2: Every opioid-therapy-duration-limitation can minimize a withdrawal-preventing-taper-need and can limit an unused-opioid.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s42-21',2,box(1),[A]),can) :- guideline_entity(actual,A,'opioid-therapy-duration-limitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s42-21',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s42-21',2,ref(2),[A]),'withdrawal-preventing-taper-need',countable) :- guideline_entity(actual,A,'opioid-therapy-duration-limitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s42-21',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s42-21',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-therapy-duration-limitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s42-21',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s42-21',2,ref(3),[A]),minimize) :- guideline_entity(actual,A,'opioid-therapy-duration-limitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s42-21',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s42-21',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-therapy-duration-limitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s42-21',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s42-21',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s42-21',2,ref(2),[A])) :- guideline_entity(actual,A,'opioid-therapy-duration-limitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s42-21',2,box(2),[A]),can) :- guideline_entity(actual,A,'opioid-therapy-duration-limitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s42-21',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s42-21',2,ref(4),[A]),'unused-opioid',countable) :- guideline_entity(actual,A,'opioid-therapy-duration-limitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s42-21',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s42-21',2,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-therapy-duration-limitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s42-21',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s42-21',2,ref(5),[A]),limit) :- guideline_entity(actual,A,'opioid-therapy-duration-limitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s42-21',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s42-21',2,ref(5),[A]),1,A) :- guideline_entity(actual,A,'opioid-therapy-duration-limitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s42-21',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s42-21',2,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s42-21',2,ref(4),[A])) :- guideline_entity(actual,A,'opioid-therapy-duration-limitation',countable), guideline_cardinality(actual,A,na,eq,1).
