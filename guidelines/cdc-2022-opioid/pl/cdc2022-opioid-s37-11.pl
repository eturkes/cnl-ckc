% cdc2022-opioid-s37-11.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s37-11',ace_sha256('301ed83e2826216533350b20801171b058e52fe6bfddeea375a99400018feae1'),ulex(sha256('4c55bed7525c11e80d870254a4f876c23b6aa3c2a077242833ec32997784b01c'))).
% S1: Every clinician should coordinate a taper-decision with a respiratory-depressant-medication-prescriber.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s37-11',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s37-11',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-11',1,ref(2),[A]),'taper-decision',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s37-11',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-11',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s37-11',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-11',1,ref(3),[A]),'respiratory-depressant-medication-prescriber',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s37-11',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-11',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s37-11',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-11',1,ref(4),[A]),coordinate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-11',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-11',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-11',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-11',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s37-11',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s37-11',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-11',1,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s37-11',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should coordinate a taper-plan with a respiratory-depressant-medication-prescriber.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s37-11',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s37-11',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-11',2,ref(2),[A]),'taper-plan',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s37-11',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-11',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s37-11',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-11',2,ref(3),[A]),'respiratory-depressant-medication-prescriber',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s37-11',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-11',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s37-11',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-11',2,ref(4),[A]),coordinate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-11',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-11',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-11',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-11',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s37-11',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s37-11',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-11',2,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s37-11',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
