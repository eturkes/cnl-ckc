% cdc2022-opioid-rec02-imp15.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec02-imp15',ace_sha256(c2de973e6032de4ba643657ba79ee29bae70e31c4222bce21c972dd975d940f2),ulex(sha256('26c94eb4c3e425db0b6c278b6861f816b856b8629608554394634e35ee8e1f3c'))).
% S1: Every clinician should not impose a nonpharmacologic-failure-requirement before an opioid-therapy.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp15',1,box(1),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',1,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec02-imp15',1,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',1,ref(2),[A]),'nonpharmacologic-failure-requirement',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',1,ref(3),[A]),'opioid-therapy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',1,ref(4),[A]),impose) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',1,ref(4),[A]),before,'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should not impose a nonopioid-pharmacologic-failure-requirement before an opioid-therapy.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp15',2,box(1),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',2,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec02-imp15',2,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',2,ref(2),[A]),'nonopioid-pharmacologic-failure-requirement',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',2,ref(3),[A]),'opioid-therapy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',2,ref(4),[A]),impose) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',2,ref(4),[A]),before,'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should not impose a specific-treatment-requirement before an opioid-therapy.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp15',3,box(1),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',3,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec02-imp15',3,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',3,ref(2),[A]),'specific-treatment-requirement',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',3,ref(3),[A]),'opioid-therapy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',3,ref(4),[A]),impose) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',3,ref(4),[A]),before,'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
