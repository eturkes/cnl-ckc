% cdc2022-opioid-s54-09.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s54-09',ace_sha256(b2b916c16cec86d959c1f5fef356535de73b898fc5c9b5deebaa1bd478f2b6ab),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: Every clinician may minimize an inequitable-toxicology-testing-cost-burden for a patient and may limit a specific-toxicology-testing to a necessary-situation.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s54-09',1,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s54-09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',1,ref(2),[A]),'inequitable-toxicology-testing-cost-burden',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s54-09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s54-09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s54-09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s54-09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',1,ref(4),[A]),minimize) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s54-09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s54-09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s54-09',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s54-09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',1,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s54-09',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s54-09',1,box(2),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s54-09',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',1,ref(5),[A]),'specific-toxicology-testing',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s54-09',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',1,ref(5),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s54-09',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',1,ref(6),[A]),'necessary-situation',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s54-09',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',1,ref(6),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s54-09',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',1,ref(7),[A]),limit) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s54-09',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',1,ref(7),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s54-09',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',1,ref(7),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s54-09',1,ref(5),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s54-09',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',1,ref(7),[A]),to,'$guideline_id'(product,'cdc2022-opioid-s54-09',1,ref(6),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every health-system may minimize an inequitable-toxicology-testing-cost-burden for a patient and may limit a specific-toxicology-testing to a necessary-situation.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s54-09',2,box(1),[A]),may) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s54-09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',2,ref(2),[A]),'inequitable-toxicology-testing-cost-burden',countable) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s54-09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s54-09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',2,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s54-09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s54-09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',2,ref(4),[A]),minimize) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s54-09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s54-09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s54-09',2,ref(2),[A])) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s54-09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',2,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s54-09',2,ref(3),[A])) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s54-09',2,box(2),[A]),may) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s54-09',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',2,ref(5),[A]),'specific-toxicology-testing',countable) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s54-09',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',2,ref(5),[A]),na,eq,1) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s54-09',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',2,ref(6),[A]),'necessary-situation',countable) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s54-09',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',2,ref(6),[A]),na,eq,1) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s54-09',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',2,ref(7),[A]),limit) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s54-09',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',2,ref(7),[A]),1,A) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s54-09',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',2,ref(7),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s54-09',2,ref(5),[A])) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s54-09',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-09',2,ref(7),[A]),to,'$guideline_id'(product,'cdc2022-opioid-s54-09',2,ref(6),[A])) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
