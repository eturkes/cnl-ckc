% cdc2022-opioid-s53-13.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s53-13',ace_sha256(f04b82b98e31c53e3ad6785e93e320b9eec095b9917375223deb64725aeaa019),ulex(sha256('64c086862c2f48d491a9fde0ba9f0fcf090e25e84456b9749332ec3ace211ce4')),vocabulary(sha256('405832ed3f275f091f60ded2e828af610266110b6c9a29536ca799d29ca43d72'))).
% S1: Every toxicology-test can assist a clinician in a prescribed-opioid-nonuse-identification.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s53-13',1,box(1),[A]),can) :- guideline_entity(actual,A,'toxicology-test',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s53-13',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-13',1,ref(2),[A]),clinician,countable) :- guideline_entity(actual,A,'toxicology-test',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s53-13',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-13',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'toxicology-test',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s53-13',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-13',1,ref(3),[A]),'prescribed-opioid-nonuse-identification',countable) :- guideline_entity(actual,A,'toxicology-test',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s53-13',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-13',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'toxicology-test',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s53-13',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-13',1,ref(4),[A]),assist) :- guideline_entity(actual,A,'toxicology-test',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s53-13',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-13',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,'toxicology-test',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s53-13',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-13',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s53-13',1,ref(2),[A])) :- guideline_entity(actual,A,'toxicology-test',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s53-13',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-13',1,ref(4),[A]),in,'$guideline_id'(product,'cdc2022-opioid-s53-13',1,ref(3),[A])) :- guideline_entity(actual,A,'toxicology-test',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: If a patient does not take a prescribed-opioid then the patient can sometimes have a diversion-or-other-clinically-important-issue.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s53-13',2,box(2),[A]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(B,C,'prescribed-opioid',countable), guideline_operator(actual,B,-), guideline_cardinality(B,C,na,eq,1), guideline_event(B,D,take), guideline_arg(B,D,1,A), guideline_arg(B,D,2,C).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s53-13',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-13',2,ref(4),[A]),'diversion-or-other-clinically-important-issue',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(B,C,'prescribed-opioid',countable), guideline_operator(actual,B,-), guideline_cardinality(B,C,na,eq,1), guideline_event(B,D,take), guideline_arg(B,D,1,A), guideline_arg(B,D,2,C).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s53-13',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-13',2,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(B,C,'prescribed-opioid',countable), guideline_operator(actual,B,-), guideline_cardinality(B,C,na,eq,1), guideline_event(B,D,take), guideline_arg(B,D,1,A), guideline_arg(B,D,2,C).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s53-13',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-13',2,ref(5),[A]),usuality,sometimes) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(B,C,'prescribed-opioid',countable), guideline_operator(actual,B,-), guideline_cardinality(B,C,na,eq,1), guideline_event(B,D,take), guideline_arg(B,D,1,A), guideline_arg(B,D,2,C).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s53-13',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-13',2,ref(5),[A]),have) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(B,C,'prescribed-opioid',countable), guideline_operator(actual,B,-), guideline_cardinality(B,C,na,eq,1), guideline_event(B,D,take), guideline_arg(B,D,1,A), guideline_arg(B,D,2,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s53-13',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-13',2,ref(5),[A]),1,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(B,C,'prescribed-opioid',countable), guideline_operator(actual,B,-), guideline_cardinality(B,C,na,eq,1), guideline_event(B,D,take), guideline_arg(B,D,1,A), guideline_arg(B,D,2,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s53-13',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s53-13',2,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s53-13',2,ref(4),[A])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(B,C,'prescribed-opioid',countable), guideline_operator(actual,B,-), guideline_cardinality(B,C,na,eq,1), guideline_event(B,D,take), guideline_arg(B,D,1,A), guideline_arg(B,D,2,C).
% S3: An other-clinically-important-issue-example is an adverse-effect-difficulty-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s53-13',3,ref(1),[]),'other-clinically-important-issue-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s53-13',3,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s53-13',3,ref(2),[]),'adverse-effect-difficulty-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s53-13',3,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s53-13',3,ref(3),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s53-13',3,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s53-13',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s53-13',3,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s53-13',3,ref(2),[])).
