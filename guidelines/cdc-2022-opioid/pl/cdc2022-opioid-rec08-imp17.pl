% cdc2022-opioid-rec08-imp17.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec08-imp17',ace_sha256('620a4f88a0e491ab0e73e4b8cef5a51496b4aae95e6926534a8e5be953504456'),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If a patient has a pain and the patient has a substance-use-disorder then the patient must receive an ongoing-pain-management that maximizes a benefit over a risk.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec08-imp17',1,box(1),[A,B,C,D,E]),must) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'substance-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08-imp17',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp17',1,ref(6),[A,B,C,D,E]),'ongoing-pain-management',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'substance-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08-imp17',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp17',1,ref(6),[A,B,C,D,E]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'substance-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08-imp17',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp17',1,ref(7),[A,B,C,D,E]),benefit,countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'substance-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08-imp17',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp17',1,ref(7),[A,B,C,D,E]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'substance-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08-imp17',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp17',1,ref(8),[A,B,C,D,E]),risk,countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'substance-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08-imp17',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp17',1,ref(8),[A,B,C,D,E]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'substance-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec08-imp17',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp17',1,ref(9),[A,B,C,D,E]),maximize) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'substance-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp17',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp17',1,ref(9),[A,B,C,D,E]),1,'$guideline_id'(product,'cdc2022-opioid-rec08-imp17',1,ref(6),[A,B,C,D,E])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'substance-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp17',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp17',1,ref(9),[A,B,C,D,E]),2,'$guideline_id'(product,'cdc2022-opioid-rec08-imp17',1,ref(7),[A,B,C,D,E])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'substance-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec08-imp17',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp17',1,ref(9),[A,B,C,D,E]),over,'$guideline_id'(product,'cdc2022-opioid-rec08-imp17',1,ref(8),[A,B,C,D,E])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'substance-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec08-imp17',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp17',1,ref(10),[A,B,C,D,E]),receive) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'substance-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp17',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp17',1,ref(10),[A,B,C,D,E]),1,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'substance-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp17',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp17',1,ref(10),[A,B,C,D,E]),2,'$guideline_id'(product,'cdc2022-opioid-rec08-imp17',1,ref(6),[A,B,C,D,E])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'substance-use-disorder',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D).
% S2: Every substance-use-disorder can alter an expected-opioid-benefit and can alter an expected-opioid-risk.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec08-imp17',2,box(1),[A]),can) :- guideline_entity(actual,A,'substance-use-disorder',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08-imp17',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp17',2,ref(2),[A]),'expected-opioid-benefit',countable) :- guideline_entity(actual,A,'substance-use-disorder',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08-imp17',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp17',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'substance-use-disorder',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec08-imp17',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp17',2,ref(3),[A]),alter) :- guideline_entity(actual,A,'substance-use-disorder',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp17',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp17',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'substance-use-disorder',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp17',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp17',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec08-imp17',2,ref(2),[A])) :- guideline_entity(actual,A,'substance-use-disorder',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec08-imp17',2,box(2),[A]),can) :- guideline_entity(actual,A,'substance-use-disorder',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08-imp17',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp17',2,ref(4),[A]),'expected-opioid-risk',countable) :- guideline_entity(actual,A,'substance-use-disorder',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08-imp17',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp17',2,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'substance-use-disorder',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec08-imp17',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp17',2,ref(5),[A]),alter) :- guideline_entity(actual,A,'substance-use-disorder',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp17',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp17',2,ref(5),[A]),1,A) :- guideline_entity(actual,A,'substance-use-disorder',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp17',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp17',2,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec08-imp17',2,ref(4),[A])) :- guideline_entity(actual,A,'substance-use-disorder',countable), guideline_cardinality(actual,A,na,eq,1).
