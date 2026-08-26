% cdc2022-opioid-rec08-imp05.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
:- multifile(guideline_schema_version/1).
:- discontiguous(guideline_schema_version/1).
:- multifile(guideline_document/3).
:- discontiguous(guideline_document/3).
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
guideline_schema_version(1).
guideline_document('cdc2022-opioid-rec08-imp05',ace_sha256('46cedeb813d575fe2a15d20bbfaa023a71a82286a6021b82c4340a533b7d7f42'),ulex(sha256('4c55bed7525c11e80d870254a4f876c23b6aa3c2a077242833ec32997784b01c'))).
% S1: If a clinical-practice-organization has a naloxone-training-resource then the clinical-practice-organization may facilitate a naloxone-coprescribing.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec08-imp05',1,box(1),[A,B,C]),may) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'naloxone-training-resource',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08-imp05',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp05',1,ref(4),[A,B,C]),'naloxone-coprescribing',countable) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'naloxone-training-resource',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08-imp05',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp05',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'naloxone-training-resource',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec08-imp05',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp05',1,ref(5),[A,B,C]),facilitate) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'naloxone-training-resource',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp05',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp05',1,ref(5),[A,B,C]),1,A) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'naloxone-training-resource',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp05',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp05',1,ref(5),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-rec08-imp05',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'naloxone-training-resource',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S2: Every pharmacist-collaborative-practice-model may facilitate a naloxone-coprescribing.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec08-imp05',2,box(1),[A]),may) :- guideline_entity(actual,A,'pharmacist-collaborative-practice-model',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08-imp05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp05',2,ref(2),[A]),'naloxone-coprescribing',countable) :- guideline_entity(actual,A,'pharmacist-collaborative-practice-model',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08-imp05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp05',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'pharmacist-collaborative-practice-model',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec08-imp05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp05',2,ref(3),[A]),facilitate) :- guideline_entity(actual,A,'pharmacist-collaborative-practice-model',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp05',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'pharmacist-collaborative-practice-model',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp05',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec08-imp05',2,ref(2),[A])) :- guideline_entity(actual,A,'pharmacist-collaborative-practice-model',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every statewide-naloxone-protocol may facilitate a naloxone-coprescribing.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec08-imp05',3,box(1),[A]),may) :- guideline_entity(actual,A,'statewide-naloxone-protocol',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08-imp05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp05',3,ref(2),[A]),'naloxone-coprescribing',countable) :- guideline_entity(actual,A,'statewide-naloxone-protocol',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08-imp05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp05',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'statewide-naloxone-protocol',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec08-imp05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp05',3,ref(3),[A]),facilitate) :- guideline_entity(actual,A,'statewide-naloxone-protocol',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp05',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,'statewide-naloxone-protocol',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp05',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec08-imp05',3,ref(2),[A])) :- guideline_entity(actual,A,'statewide-naloxone-protocol',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every pharmacy-naloxone-standing-order may facilitate a naloxone-coprescribing.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec08-imp05',4,box(1),[A]),may) :- guideline_entity(actual,A,'pharmacy-naloxone-standing-order',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec08-imp05',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp05',4,ref(2),[A]),'naloxone-coprescribing',countable) :- guideline_entity(actual,A,'pharmacy-naloxone-standing-order',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec08-imp05',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp05',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'pharmacy-naloxone-standing-order',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec08-imp05',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp05',4,ref(3),[A]),facilitate) :- guideline_entity(actual,A,'pharmacy-naloxone-standing-order',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp05',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp05',4,ref(3),[A]),1,A) :- guideline_entity(actual,A,'pharmacy-naloxone-standing-order',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec08-imp05',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec08-imp05',4,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec08-imp05',4,ref(2),[A])) :- guideline_entity(actual,A,'pharmacy-naloxone-standing-order',countable), guideline_cardinality(actual,A,na,eq,1).
