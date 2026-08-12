% cdc2022-opioid-s32-03.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s32-03',ace_sha256('414db34c4d691af685ca5e4815386dd16ba57a312da738b18552131234583200'),ulex(sha256('6bd0df91739d5a42b1a472d866e0dd72897517f8e4652458e4893adbed9cc663'))).
% S1: If a clinician prescribes an ER-LA-opioid then the clinician should use a predictable-pharmacokinetic-and-pharmacodynamic-ER-LA-opioid during an unintentional-overdose-risk-minimization.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s32-03',1,box(1),[A,B,C]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'ER-LA-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,prescribe), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s32-03',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s32-03',1,ref(4),[A,B,C]),'predictable-pharmacokinetic-and-pharmacodynamic-ER-LA-opioid',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'ER-LA-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,prescribe), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s32-03',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s32-03',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'ER-LA-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,prescribe), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s32-03',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s32-03',1,ref(5),[A,B,C]),'unintentional-overdose-risk-minimization',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'ER-LA-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,prescribe), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s32-03',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s32-03',1,ref(5),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'ER-LA-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,prescribe), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s32-03',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s32-03',1,ref(6),[A,B,C]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'ER-LA-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,prescribe), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s32-03',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s32-03',1,ref(6),[A,B,C]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'ER-LA-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,prescribe), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s32-03',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s32-03',1,ref(6),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s32-03',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'ER-LA-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,prescribe), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s32-03',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s32-03',1,ref(6),[A,B,C]),during,'$guideline_id'(product,'cdc2022-opioid-s32-03',1,ref(5),[A,B,C])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'ER-LA-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,prescribe), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
