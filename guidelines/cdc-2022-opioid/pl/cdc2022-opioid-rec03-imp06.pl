% cdc2022-opioid-rec03-imp06.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec03-imp06',ace_sha256(b5e5e732ffb90fbdcbf04c12987f380f34e8eb6b9e54eb167a9cd441833c5fe1),ulex(sha256('0e8a9a04fd387b8eab12faf03224671330b94c03dff5f0bf39f23a8d14af88bd'))).
% S1: Every clinician should not select a methadone-treatment for a first-choice-ER-LA-decision.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec03-imp06',1,box(1),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec03-imp06',1,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec03-imp06',1,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec03-imp06',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',1,ref(2),[A]),'methadone-treatment',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec03-imp06',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec03-imp06',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',1,ref(3),[A]),'first-choice-ER-LA-decision',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec03-imp06',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec03-imp06',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',1,ref(4),[A]),select) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec03-imp06',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec03-imp06',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec03-imp06',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',1,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
