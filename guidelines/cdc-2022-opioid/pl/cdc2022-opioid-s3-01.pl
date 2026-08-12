% cdc2022-opioid-s3-01.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s3-01',ace_sha256('984e377bd1e9b88786fdbdb09c5f608f16247671b67e0cb1207262ba44a1deda'),ulex(sha256(ef57ba7180864095bab88b95f11ddb1a2c858593eba715dbfbe874db09506c5b))).
% S1: If a patient has a pain then the patient should receive an appropriate-pain-treatment.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s3-01',1,box(1),[A,B,C]),should) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s3-01',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s3-01',1,ref(4),[A,B,C]),'appropriate-pain-treatment',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s3-01',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s3-01',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s3-01',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s3-01',1,ref(5),[A,B,C]),receive) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s3-01',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s3-01',1,ref(5),[A,B,C]),1,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s3-01',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s3-01',1,ref(5),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s3-01',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S2: Every clinician should weigh a treatment-benefit against a treatment-risk in a patient-circumstance.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s3-01',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s3-01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-01',2,ref(2),[A]),'treatment-benefit',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s3-01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-01',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s3-01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-01',2,ref(3),[A]),'treatment-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s3-01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-01',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s3-01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-01',2,ref(4),[A]),'patient-circumstance',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s3-01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-01',2,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s3-01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-01',2,ref(5),[A]),weigh) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s3-01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-01',2,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s3-01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-01',2,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s3-01',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s3-01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-01',2,ref(5),[A]),in,'$guideline_id'(product,'cdc2022-opioid-s3-01',2,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s3-01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-01',2,ref(5),[A]),against,'$guideline_id'(product,'cdc2022-opioid-s3-01',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
