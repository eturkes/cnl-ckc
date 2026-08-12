% cdc2022-opioid-s38-05.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s38-05',ace_sha256(c49aa3860c2845ea3caabdac7f017710a9ea6d6d93a2a7a7b33535164cf9c837),ulex(sha256(ef57ba7180864095bab88b95f11ddb1a2c858593eba715dbfbe874db09506c5b))).
% S1: Every clinician may advise a patient about a frequent-and-time-diminishing-opioid-withdrawal-pain-worsening.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s38-05',1,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-05',1,ref(2),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-05',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-05',1,ref(3),[A]),'frequent-and-time-diminishing-opioid-withdrawal-pain-worsening',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-05',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s38-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-05',1,ref(4),[A]),advise) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-05',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-05',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s38-05',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s38-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-05',1,ref(4),[A]),about,'$guideline_id'(product,'cdc2022-opioid-s38-05',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
