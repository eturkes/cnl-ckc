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
guideline_schema_version(3).
guideline_document('cdc2022-opioid-s37-19',ace_sha256('8c5aeb573b4384f65159e208629b09c7c4d79a96360d309afd87edab992a1c49'),ulex(sha256(be24cf56c59049d3c31fba641013faa22821ecda0669443a1fc4a49979ef57ae)),temporal(sha256('29a90281976bb48dfa32d9ef80651076896404c524c9bf1d7a55b06c1880be2a'))).
% S1: If a patient does not have an imminent-opioid-regimen-harm-risk then every clinician may delay an opioid-taper for the patient and may reach a taper-agreement with the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s37-19',1,box(2),[A,B]),may) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,C,-), guideline_entity(C,D,'imminent-opioid-regimen-harm-risk',countable), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s37-19',1,box(2),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s37-19',1,ref(5),[A,B]),'opioid-taper',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,C,-), guideline_entity(C,D,'imminent-opioid-regimen-harm-risk',countable), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s37-19',1,box(2),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s37-19',1,ref(5),[A,B]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,C,-), guideline_entity(C,D,'imminent-opioid-regimen-harm-risk',countable), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s37-19',1,box(2),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s37-19',1,ref(6),[A,B]),delay) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,C,-), guideline_entity(C,D,'imminent-opioid-regimen-harm-risk',countable), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-19',1,box(2),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s37-19',1,ref(6),[A,B]),1,B) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,C,-), guideline_entity(C,D,'imminent-opioid-regimen-harm-risk',countable), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-19',1,box(2),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s37-19',1,ref(6),[A,B]),2,'$guideline_id'(product,'cdc2022-opioid-s37-19',1,ref(5),[A,B])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,C,-), guideline_entity(C,D,'imminent-opioid-regimen-harm-risk',countable), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s37-19',1,box(2),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s37-19',1,ref(6),[A,B]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,C,-), guideline_entity(C,D,'imminent-opioid-regimen-harm-risk',countable), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s37-19',1,box(3),[A,B]),may) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,C,-), guideline_entity(C,D,'imminent-opioid-regimen-harm-risk',countable), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s37-19',1,box(3),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s37-19',1,ref(7),[A,B]),'taper-agreement',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,C,-), guideline_entity(C,D,'imminent-opioid-regimen-harm-risk',countable), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s37-19',1,box(3),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s37-19',1,ref(7),[A,B]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,C,-), guideline_entity(C,D,'imminent-opioid-regimen-harm-risk',countable), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s37-19',1,box(3),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s37-19',1,ref(8),[A,B]),reach) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,C,-), guideline_entity(C,D,'imminent-opioid-regimen-harm-risk',countable), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-19',1,box(3),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s37-19',1,ref(8),[A,B]),1,B) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,C,-), guideline_entity(C,D,'imminent-opioid-regimen-harm-risk',countable), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-19',1,box(3),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s37-19',1,ref(8),[A,B]),2,'$guideline_id'(product,'cdc2022-opioid-s37-19',1,ref(7),[A,B])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,C,-), guideline_entity(C,D,'imminent-opioid-regimen-harm-risk',countable), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s37-19',1,box(3),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s37-19',1,ref(8),[A,B]),with,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,C,-), guideline_entity(C,D,'imminent-opioid-regimen-harm-risk',countable), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
