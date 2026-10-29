% cdc2022-opioid-s27-09.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s27-09',ace_sha256(da30098612aee7cea5edd0276dd465b25b222d8c1a80edec4f378d72548188a6),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: Every duloxetine has a fibromyalgia-FDA-approval.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s27-09',1,ref(2),[A]),'fibromyalgia-FDA-approval',countable) :- guideline_entity(actual,A,duloxetine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s27-09',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,duloxetine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s27-09',1,ref(3),[A]),have) :- guideline_entity(actual,A,duloxetine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s27-09',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,duloxetine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s27-09',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s27-09',1,ref(2),[A])) :- guideline_entity(actual,A,duloxetine,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every milnacipran has a fibromyalgia-FDA-approval.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s27-09',2,ref(2),[A]),'fibromyalgia-FDA-approval',countable) :- guideline_entity(actual,A,milnacipran,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s27-09',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,milnacipran,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s27-09',2,ref(3),[A]),have) :- guideline_entity(actual,A,milnacipran,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s27-09',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,milnacipran,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s27-09',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s27-09',2,ref(2),[A])) :- guideline_entity(actual,A,milnacipran,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every pregabalin has a fibromyalgia-FDA-approval.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s27-09',3,ref(2),[A]),'fibromyalgia-FDA-approval',countable) :- guideline_entity(actual,A,pregabalin,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s27-09',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,pregabalin,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s27-09',3,ref(3),[A]),have) :- guideline_entity(actual,A,pregabalin,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s27-09',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,pregabalin,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s27-09',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s27-09',3,ref(2),[A])) :- guideline_entity(actual,A,pregabalin,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: If a patient has a fibromyalgia then every clinician should consider a duloxetine for the fibromyalgia.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s27-09',4,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s27-09',4,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s27-09',4,ref(5),[A,B,C,D]),duloxetine,countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s27-09',4,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s27-09',4,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s27-09',4,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s27-09',4,ref(6),[A,B,C,D]),consider) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s27-09',4,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s27-09',4,ref(6),[A,B,C,D]),1,D) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s27-09',4,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s27-09',4,ref(6),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-s27-09',4,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s27-09',4,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s27-09',4,ref(6),[A,B,C,D]),for,B) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
% S5: If a patient has a fibromyalgia then every clinician should consider a milnacipran for the fibromyalgia.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s27-09',5,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s27-09',5,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s27-09',5,ref(5),[A,B,C,D]),milnacipran,countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s27-09',5,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s27-09',5,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s27-09',5,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s27-09',5,ref(6),[A,B,C,D]),consider) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s27-09',5,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s27-09',5,ref(6),[A,B,C,D]),1,D) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s27-09',5,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s27-09',5,ref(6),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-s27-09',5,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s27-09',5,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s27-09',5,ref(6),[A,B,C,D]),for,B) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
% S6: If a patient has a fibromyalgia then every clinician should consider a pregabalin for the fibromyalgia.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s27-09',6,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s27-09',6,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s27-09',6,ref(5),[A,B,C,D]),pregabalin,countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s27-09',6,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s27-09',6,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s27-09',6,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s27-09',6,ref(6),[A,B,C,D]),consider) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s27-09',6,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s27-09',6,ref(6),[A,B,C,D]),1,D) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s27-09',6,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s27-09',6,ref(6),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-s27-09',6,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s27-09',6,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s27-09',6,ref(6),[A,B,C,D]),for,B) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
