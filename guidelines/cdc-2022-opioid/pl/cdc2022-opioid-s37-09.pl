% cdc2022-opioid-s37-09.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s37-09',ace_sha256('0211e174a570336854d083d63e3a4d5f5e73a3ec5a22b7299a84b86706a8f08a'),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If a patient is pregnant and has an opioid-use-disorder then an opioid-use-disorder-medication is preferred instead-of a withdrawal-management for the opioid-use-disorder.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(6),[A,B,C,D,E]),'opioid-use-disorder-medication',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,pregnant,pos), guideline_entity(actual,D,'opioid-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(6),[A,B,C,D,E]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,pregnant,pos), guideline_entity(actual,D,'opioid-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(7),[A,B,C,D,E]),'withdrawal-management',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,pregnant,pos), guideline_entity(actual,D,'opioid-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(7),[A,B,C,D,E]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,pregnant,pos), guideline_entity(actual,D,'opioid-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_property(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(8),[A,B,C,D,E]),preferred,pos) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,pregnant,pos), guideline_entity(actual,D,'opioid-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(9),[A,B,C,D,E]),be) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,pregnant,pos), guideline_entity(actual,D,'opioid-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(9),[A,B,C,D,E]),1,'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(6),[A,B,C,D,E])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,pregnant,pos), guideline_entity(actual,D,'opioid-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(9),[A,B,C,D,E]),2,'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(8),[A,B,C,D,E])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,pregnant,pos), guideline_entity(actual,D,'opioid-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(9),[A,B,C,D,E]),for,D) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,pregnant,pos), guideline_entity(actual,D,'opioid-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(9),[A,B,C,D,E]),'instead-of','$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(7),[A,B,C,D,E])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,pregnant,pos), guideline_entity(actual,D,'opioid-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_preference(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(9),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(6),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s37-09',1,ref(7),[A,B,C,D,E])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,pregnant,pos), guideline_entity(actual,D,'opioid-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
% S2: Every withdrawal-management uses a short-or-medium-term-opioid-discontinuation-taper.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',2,ref(2),[A]),'short-or-medium-term-opioid-discontinuation-taper',countable) :- guideline_entity(actual,A,'withdrawal-management',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'withdrawal-management',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',2,ref(3),[A]),use) :- guideline_entity(actual,A,'withdrawal-management',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'withdrawal-management',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s37-09',2,ref(2),[A])) :- guideline_entity(actual,A,'withdrawal-management',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: A withdrawal-management-example is an opioid-discontinuation-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',3,ref(1),[]),'withdrawal-management-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',3,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',3,ref(2),[]),'opioid-discontinuation-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',3,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',3,ref(3),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',3,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s37-09',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s37-09',3,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s37-09',3,ref(2),[])).
