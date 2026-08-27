% cdc2022-opioid-rec09-imp07.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec09-imp07',ace_sha256('0cfcbe8ceb2c498dd8939a99f4004935615fc5a93766f7b6cf8a63b445bf5164'),ulex(sha256('4a882a1996ea910df0fbf91fea897c0805e67d48f9e1ce2fcdbbbd9250c193ae'))).
% S1: Every PDMP-generated-risk-score is not a validated-clinical-outcome-predictor.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec09-imp07',1,box(1),[A]),-) :- guideline_entity(actual,A,'PDMP-generated-risk-score',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec09-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp07',1,ref(2),[A]),'validated-clinical-outcome-predictor',countable) :- guideline_entity(actual,A,'PDMP-generated-risk-score',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec09-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp07',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'PDMP-generated-risk-score',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec09-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp07',1,ref(3),[A]),be) :- guideline_entity(actual,A,'PDMP-generated-risk-score',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec09-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp07',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'PDMP-generated-risk-score',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec09-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp07',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec09-imp07',1,ref(2),[A])) :- guideline_entity(actual,A,'PDMP-generated-risk-score',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: If a clinician considers a PDMP-generated-risk-score then the clinician should not use the PDMP-generated-risk-score as a clinical-judgment-replacement.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec09-imp07',2,box(1),[A,B,C]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'PDMP-generated-risk-score',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,consider), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec09-imp07',2,box(1),[A,B,C]),'$guideline_id'(context,'cdc2022-opioid-rec09-imp07',2,box(2),[A,B,C]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'PDMP-generated-risk-score',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,consider), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec09-imp07',2,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp07',2,ref(4),[A,B,C]),'clinical-judgment-replacement',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'PDMP-generated-risk-score',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,consider), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec09-imp07',2,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp07',2,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'PDMP-generated-risk-score',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,consider), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec09-imp07',2,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp07',2,ref(5),[A,B,C]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'PDMP-generated-risk-score',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,consider), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec09-imp07',2,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp07',2,ref(5),[A,B,C]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'PDMP-generated-risk-score',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,consider), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec09-imp07',2,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp07',2,ref(5),[A,B,C]),2,B) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'PDMP-generated-risk-score',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,consider), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec09-imp07',2,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp07',2,ref(5),[A,B,C]),as,'$guideline_id'(product,'cdc2022-opioid-rec09-imp07',2,ref(4),[A,B,C])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'PDMP-generated-risk-score',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,consider), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
