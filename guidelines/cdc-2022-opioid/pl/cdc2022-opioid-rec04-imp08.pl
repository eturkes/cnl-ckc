% cdc2022-opioid-rec04-imp08.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec04-imp08',ace_sha256(ea4944bf60c52132b639f5a09af9ef3e34a7596219837ba1da8528456a383036),ulex(sha256(ef57ba7180864095bab88b95f11ddb1a2c858593eba715dbfbe874db09506c5b))).
% S1: If a clinician decides a dosage-increase then the clinician should heed a caution and should use a smallest-practical-increase.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(1),[A,B,C]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(4),[A,B,C]),caution,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(5),[A,B,C]),heed) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(5),[A,B,C]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(5),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(2),[A,B,C]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(6),[A,B,C]),'smallest-practical-increase',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(6),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(7),[A,B,C]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(7),[A,B,C]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(7),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(6),[A,B,C])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
