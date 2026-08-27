% cdc2022-opioid-s29-20.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s29-20',ace_sha256('07300be6b8b31c8cf055c9ed99237c233830fd9c99c8a3b65f00a134c14c507c'),ulex(sha256('4a882a1996ea910df0fbf91fea897c0805e67d48f9e1ce2fcdbbbd9250c193ae'))).
% S1: Every clinician should discuss a longitudinal-medication-use-reassessment with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s29-20',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-20',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-20',1,ref(2),[A]),'longitudinal-medication-use-reassessment',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-20',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-20',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-20',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-20',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-20',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-20',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s29-20',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-20',1,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-20',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-20',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-20',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-20',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s29-20',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s29-20',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-20',1,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s29-20',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: If a caregiver-involvement is appropriate then every clinician should discuss a longitudinal-medication-use-reassessment with a caregiver.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s29-20',2,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,'caregiver-involvement',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,appropriate,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-20',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s29-20',2,ref(5),[A,B,C,D]),'longitudinal-medication-use-reassessment',countable) :- guideline_entity(actual,A,'caregiver-involvement',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,appropriate,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-20',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s29-20',2,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,'caregiver-involvement',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,appropriate,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-20',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s29-20',2,ref(6),[A,B,C,D]),caregiver,countable) :- guideline_entity(actual,A,'caregiver-involvement',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,appropriate,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-20',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s29-20',2,ref(6),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,'caregiver-involvement',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,appropriate,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s29-20',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s29-20',2,ref(7),[A,B,C,D]),discuss) :- guideline_entity(actual,A,'caregiver-involvement',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,appropriate,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-20',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s29-20',2,ref(7),[A,B,C,D]),1,D) :- guideline_entity(actual,A,'caregiver-involvement',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,appropriate,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-20',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s29-20',2,ref(7),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-s29-20',2,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,'caregiver-involvement',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,appropriate,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s29-20',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s29-20',2,ref(7),[A,B,C,D]),with,'$guideline_id'(product,'cdc2022-opioid-s29-20',2,ref(6),[A,B,C,D])) :- guideline_entity(actual,A,'caregiver-involvement',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,appropriate,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
