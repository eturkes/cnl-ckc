% cdc2022-opioid-s30-03.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s30-03',ace_sha256(a6ddf11c2d084f0a4c6a08417a972e2566503ab6bac870fa7d32a702ba554d30),ulex(sha256('6bd0df91739d5a42b1a472d866e0dd72897517f8e4652458e4893adbed9cc663'))).
% S1: Every clinician may consult a qualified-pain-management-specialist during a patient-specific-interventional-procedure-appropriateness-assessment.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s30-03',1,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-03',1,ref(2),[A]),'qualified-pain-management-specialist',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-03',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-03',1,ref(3),[A]),'patient-specific-interventional-procedure-appropriateness-assessment',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-03',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s30-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-03',1,ref(4),[A]),consult) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-03',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-03',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s30-03',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s30-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-03',1,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s30-03',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
