% cdc2022-opioid-s59-14.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s59-14',ace_sha256('13b567eae8d7fc7416468e650d5c54a3e527a7c64c4be403becf4a6b147fa65c'),ulex(sha256('64c086862c2f48d491a9fde0ba9f0fcf090e25e84456b9749332ec3ace211ce4')),vocabulary(sha256('06621a3dca592ca4385262a4a36dd8184670cdaef31827cd8b224c8c870c2f7d'))).
% S1: If a patient does not provably have a very-limited-oral-naltrexone-circumstance and there is a clinician then it is recommended that the clinician does not use an oral-naltrexone-treatment for the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s59-14',1,box(1),[A,B]),should) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,C,'very-limited-oral-naltrexone-circumstance',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C)), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-s59-14',1,box(1),[A,B]),'$guideline_id'(context,'cdc2022-opioid-s59-14',1,box(2),[A,B]),-) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,C,'very-limited-oral-naltrexone-circumstance',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C)), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s59-14',1,box(2),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s59-14',1,ref(5),[A,B]),'oral-naltrexone-treatment',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,C,'very-limited-oral-naltrexone-circumstance',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C)), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s59-14',1,box(2),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s59-14',1,ref(5),[A,B]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,C,'very-limited-oral-naltrexone-circumstance',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C)), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s59-14',1,box(2),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s59-14',1,ref(6),[A,B]),use) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,C,'very-limited-oral-naltrexone-circumstance',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C)), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s59-14',1,box(2),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s59-14',1,ref(6),[A,B]),1,B) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,C,'very-limited-oral-naltrexone-circumstance',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C)), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s59-14',1,box(2),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s59-14',1,ref(6),[A,B]),2,'$guideline_id'(product,'cdc2022-opioid-s59-14',1,ref(5),[A,B])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,C,'very-limited-oral-naltrexone-circumstance',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C)), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s59-14',1,box(2),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s59-14',1,ref(6),[A,B]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,C,'very-limited-oral-naltrexone-circumstance',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C)), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
% S2: If a patient has a very-limited-oral-naltrexone-circumstance then every clinician may use an oral-naltrexone-treatment for the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s59-14',2,box(1),[A,B,C,D]),may) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'very-limited-oral-naltrexone-circumstance',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s59-14',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s59-14',2,ref(5),[A,B,C,D]),'oral-naltrexone-treatment',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'very-limited-oral-naltrexone-circumstance',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s59-14',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s59-14',2,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'very-limited-oral-naltrexone-circumstance',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s59-14',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s59-14',2,ref(6),[A,B,C,D]),use) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'very-limited-oral-naltrexone-circumstance',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s59-14',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s59-14',2,ref(6),[A,B,C,D]),1,D) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'very-limited-oral-naltrexone-circumstance',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s59-14',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s59-14',2,ref(6),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-s59-14',2,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'very-limited-oral-naltrexone-circumstance',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s59-14',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s59-14',2,ref(6),[A,B,C,D]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'very-limited-oral-naltrexone-circumstance',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
