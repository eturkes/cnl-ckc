% cdc2022-opioid-s30-08.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s30-08',ace_sha256('379ec153de6e7b144285e32fc6a37296d79222597f1808cbf3505385473615e1'),ulex(sha256('2a4a3e829c919b3dd3b20356e7a69585cfb305c3c57dfb531e3f2c1eb97637b6'))).
% S1: Every clinician should heed a medication-combination-caution during a medication-combination-use.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s30-08',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',1,ref(2),[A]),'medication-combination-caution',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',1,ref(3),[A]),'medication-combination-use',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s30-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',1,ref(4),[A]),heed) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s30-08',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s30-08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',1,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s30-08',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should avoid a synergistic-medication-risk during a medication-combination-use.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s30-08',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-08',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',2,ref(2),[A]),'synergistic-medication-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-08',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-08',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',2,ref(3),[A]),'medication-combination-use',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-08',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s30-08',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',2,ref(4),[A]),avoid) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-08',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-08',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s30-08',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s30-08',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-08',2,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s30-08',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
