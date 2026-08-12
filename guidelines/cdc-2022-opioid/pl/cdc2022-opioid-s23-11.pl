% cdc2022-opioid-s23-11.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s23-11',ace_sha256('5dd5aad73387df4e38569363d064aad24d4f5c2cbddaf1c4823fa36e062e7fa7'),ulex(sha256('6bd0df91739d5a42b1a472d866e0dd72897517f8e4652458e4893adbed9cc663'))).
% S1: Every clinician should discuss a workplace-toxicology-detection-of-therapeutic-opioid-use with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-11',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-11',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-11',1,ref(2),[A]),'workplace-toxicology-detection-of-therapeutic-opioid-use',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-11',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-11',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-11',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-11',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-11',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-11',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-11',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-11',1,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-11',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-11',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-11',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-11',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-11',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-11',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-11',1,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-11',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
