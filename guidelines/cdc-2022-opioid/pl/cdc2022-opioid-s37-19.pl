% cdc2022-opioid-s37-19.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s37-19',ace_sha256('352c4513048358b6ecd3d2a4c1b62eaf55303696f949745667200f4b5ab9238a'),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If a patient does not have an imminent-opioid-regimen-harm-risk then every clinician may delay an opioid-taper for the patient and may reach a taper-agreement with the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s37-19',1,box(2),[A,B]),may) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(C,D,'imminent-opioid-regimen-harm-risk',countable), guideline_operator(actual,C,-), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s37-19',1,box(2),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s37-19',1,ref(5),[A,B]),'opioid-taper',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(C,D,'imminent-opioid-regimen-harm-risk',countable), guideline_operator(actual,C,-), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s37-19',1,box(2),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s37-19',1,ref(5),[A,B]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(C,D,'imminent-opioid-regimen-harm-risk',countable), guideline_operator(actual,C,-), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s37-19',1,box(2),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s37-19',1,ref(6),[A,B]),delay) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(C,D,'imminent-opioid-regimen-harm-risk',countable), guideline_operator(actual,C,-), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-19',1,box(2),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s37-19',1,ref(6),[A,B]),1,B) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(C,D,'imminent-opioid-regimen-harm-risk',countable), guideline_operator(actual,C,-), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-19',1,box(2),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s37-19',1,ref(6),[A,B]),2,'$guideline_id'(product,'cdc2022-opioid-s37-19',1,ref(5),[A,B])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(C,D,'imminent-opioid-regimen-harm-risk',countable), guideline_operator(actual,C,-), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s37-19',1,box(2),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s37-19',1,ref(6),[A,B]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(C,D,'imminent-opioid-regimen-harm-risk',countable), guideline_operator(actual,C,-), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s37-19',1,box(3),[A,B]),may) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(C,D,'imminent-opioid-regimen-harm-risk',countable), guideline_operator(actual,C,-), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s37-19',1,box(3),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s37-19',1,ref(7),[A,B]),'taper-agreement',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(C,D,'imminent-opioid-regimen-harm-risk',countable), guideline_operator(actual,C,-), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s37-19',1,box(3),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s37-19',1,ref(7),[A,B]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(C,D,'imminent-opioid-regimen-harm-risk',countable), guideline_operator(actual,C,-), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s37-19',1,box(3),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s37-19',1,ref(8),[A,B]),reach) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(C,D,'imminent-opioid-regimen-harm-risk',countable), guideline_operator(actual,C,-), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-19',1,box(3),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s37-19',1,ref(8),[A,B]),1,B) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(C,D,'imminent-opioid-regimen-harm-risk',countable), guideline_operator(actual,C,-), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-19',1,box(3),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s37-19',1,ref(8),[A,B]),2,'$guideline_id'(product,'cdc2022-opioid-s37-19',1,ref(7),[A,B])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(C,D,'imminent-opioid-regimen-harm-risk',countable), guideline_operator(actual,C,-), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s37-19',1,box(3),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s37-19',1,ref(8),[A,B]),with,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(C,D,'imminent-opioid-regimen-harm-risk',countable), guideline_operator(actual,C,-), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
% S2: An imminent-opioid-regimen-harm-risk-example includes an imminent-overdose-risk-example and includes an imminent-other-injury-risk-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s37-19',2,ref(1),[]),'imminent-opioid-regimen-harm-risk-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s37-19',2,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s37-19',2,ref(2),[]),'imminent-overdose-risk-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s37-19',2,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s37-19',2,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s37-19',2,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s37-19',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s37-19',2,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s37-19',2,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s37-19',2,ref(4),[]),'imminent-other-injury-risk-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s37-19',2,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s37-19',2,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s37-19',2,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s37-19',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s37-19',2,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s37-19',2,ref(4),[])).
