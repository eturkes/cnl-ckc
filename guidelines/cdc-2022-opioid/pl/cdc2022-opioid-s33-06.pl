% cdc2022-opioid-s33-06.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s33-06',ace_sha256(cb891c058b9b75437b7e96072496879920b4263ee6608d45aa404607a942f854),ulex(sha256(ef57ba7180864095bab88b95f11ddb1a2c858593eba715dbfbe874db09506c5b))).
% S1: Every clinician should not use a conversion-factor for an opioid-use-disorder-dosage-decision.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s33-06',1,box(1),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-s33-06',1,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-s33-06',1,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s33-06',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-06',1,ref(2),[A]),'conversion-factor',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s33-06',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-06',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s33-06',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-06',1,ref(3),[A]),'opioid-use-disorder-dosage-decision',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s33-06',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-06',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s33-06',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-06',1,ref(4),[A]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s33-06',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-06',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s33-06',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-06',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s33-06',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s33-06',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-06',1,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s33-06',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
