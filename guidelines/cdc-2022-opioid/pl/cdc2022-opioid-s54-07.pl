% cdc2022-opioid-s54-07.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s54-07',ace_sha256(d66a989de8dce2024529e728568de18a17f4758de3b1c446c7630e0fd4146bd4),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If a patient has an oxycodone-prescription then every clinician should order an oxycodone-specific-toxicology-testing for the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s54-07',1,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'oxycodone-prescription',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s54-07',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s54-07',1,ref(5),[A,B,C,D]),'oxycodone-specific-toxicology-testing',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'oxycodone-prescription',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s54-07',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s54-07',1,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'oxycodone-prescription',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s54-07',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s54-07',1,ref(6),[A,B,C,D]),order) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'oxycodone-prescription',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s54-07',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s54-07',1,ref(6),[A,B,C,D]),1,D) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'oxycodone-prescription',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s54-07',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s54-07',1,ref(6),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-s54-07',1,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'oxycodone-prescription',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s54-07',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s54-07',1,ref(6),[A,B,C,D]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'oxycodone-prescription',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
% S2: If a patient receives a prescribed-nonmorphine-based-opioid then every clinician should order a specific-toxicology-testing for the prescribed-nonmorphine-based-opioid for the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s54-07',2,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prescribed-nonmorphine-based-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s54-07',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s54-07',2,ref(5),[A,B,C,D]),'specific-toxicology-testing',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prescribed-nonmorphine-based-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s54-07',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s54-07',2,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prescribed-nonmorphine-based-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s54-07',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s54-07',2,ref(6),[A,B,C,D]),order) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prescribed-nonmorphine-based-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s54-07',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s54-07',2,ref(6),[A,B,C,D]),1,D) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prescribed-nonmorphine-based-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s54-07',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s54-07',2,ref(6),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-s54-07',2,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prescribed-nonmorphine-based-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s54-07',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s54-07',2,ref(6),[A,B,C,D]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prescribed-nonmorphine-based-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s54-07',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s54-07',2,ref(6),[A,B,C,D]),for,B) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prescribed-nonmorphine-based-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
% S3: A nonmorphine-based-opioid-example includes a buprenorphine-example and includes a methadone-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s54-07',3,ref(1),[]),'nonmorphine-based-opioid-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s54-07',3,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s54-07',3,ref(2),[]),'buprenorphine-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s54-07',3,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s54-07',3,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s54-07',3,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s54-07',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s54-07',3,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s54-07',3,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s54-07',3,ref(4),[]),'methadone-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s54-07',3,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s54-07',3,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s54-07',3,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s54-07',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s54-07',3,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s54-07',3,ref(4),[])).
