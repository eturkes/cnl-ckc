% cdc2022-opioid-s39-16.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s39-16',ace_sha256(b2f55eb739d7dcdb281c014588f0e72bbe2dbf55554cef846467c8c47f497486),ulex(sha256('06c1c26d6b6abe1436b8c94a276937ad072806062117417de2ab8086fdf9a3e9'))).
% S1: Every clinician should ensure a mild-to-moderate-full-agonist-opioid-withdrawal for a patient before a first-buprenorphine-dose.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s39-16',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(2),[A]),'mild-to-moderate-full-agonist-opioid-withdrawal',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(4),[A]),'first-buprenorphine-dose',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s39-16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(5),[A]),ensure) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s39-16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(5),[A]),before,'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s39-16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(5),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s39-16',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
