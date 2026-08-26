% cdc2022-opioid-rec12-imp02.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec12-imp02',ace_sha256('45ed2535a4894def15f82fbbd8f1111a1e4557340cfa244a94af8d590fa099a9'),ulex(sha256('4c55bed7525c11e80d870254a4f876c23b6aa3c2a077242833ec32997784b01c'))).
% S1: Every clinician should conduct a DSM-5-opioid-use-disorder-assessment for a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec12-imp02',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec12-imp02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec12-imp02',1,ref(2),[A]),'DSM-5-opioid-use-disorder-assessment',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec12-imp02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec12-imp02',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec12-imp02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec12-imp02',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec12-imp02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec12-imp02',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec12-imp02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec12-imp02',1,ref(4),[A]),conduct) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec12-imp02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec12-imp02',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec12-imp02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec12-imp02',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec12-imp02',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec12-imp02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec12-imp02',1,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec12-imp02',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
