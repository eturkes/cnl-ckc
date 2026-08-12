% cdc2022-opioid-s12-02.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s12-02',ace_sha256(d6eb20804bcc6d391de2ecc64980416e5dc638393a70f602697c7cdad43ba51c),ulex(sha256(ef57ba7180864095bab88b95f11ddb1a2c858593eba715dbfbe874db09506c5b))).
% S1: Every category-B-recommendation has a possible-partial-person-applicability.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s12-02',1,ref(2),[A]),'possible-partial-person-applicability',countable) :- guideline_entity(actual,A,'category-B-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s12-02',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'category-B-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s12-02',1,ref(3),[A]),have) :- guideline_entity(actual,A,'category-B-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s12-02',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'category-B-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s12-02',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s12-02',1,ref(2),[A])) :- guideline_entity(actual,A,'category-B-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every category-B-recommendation indicates an appropriate-different-choice.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s12-02',2,ref(2),[A]),'appropriate-different-choice',countable) :- guideline_entity(actual,A,'category-B-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s12-02',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'category-B-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s12-02',2,ref(3),[A]),indicate) :- guideline_entity(actual,A,'category-B-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s12-02',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'category-B-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s12-02',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s12-02',2,ref(2),[A])) :- guideline_entity(actual,A,'category-B-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should base a category-B-decision on a patient-circumstance.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s12-02',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s12-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-02',3,ref(2),[A]),'category-B-decision',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s12-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-02',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s12-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-02',3,ref(3),[A]),'patient-circumstance',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s12-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-02',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s12-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-02',3,ref(4),[A]),base) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s12-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-02',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s12-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-02',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s12-02',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s12-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-02',3,ref(4),[A]),on,'$guideline_id'(product,'cdc2022-opioid-s12-02',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
