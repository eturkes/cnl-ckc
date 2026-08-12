% cdc2022-opioid-rec05-imp20.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec05-imp20',ace_sha256('12b7095a512dbcb298dd1d4c1b874ca3c012c1bd39d850d9c002ddfeeaa2fee1'),ulex(sha256('0e8a9a04fd387b8eab12faf03224671330b94c03dff5f0bf39f23a8d14af88bd'))).
% S1: Every clinician should explain an abrupt-previous-higher-dose-return-overdose-risk with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp20',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp20',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp20',1,ref(2),[A]),'abrupt-previous-higher-dose-return-overdose-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp20',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp20',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp20',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp20',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp20',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp20',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp20',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp20',1,ref(4),[A]),explain) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp20',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp20',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp20',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp20',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp20',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp20',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp20',1,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-rec05-imp20',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should provide an opioid-overdose-education and should offer a naloxone-dose.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp20',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp20',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp20',2,ref(2),[A]),'opioid-overdose-education',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp20',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp20',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp20',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp20',2,ref(3),[A]),provide) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp20',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp20',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp20',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp20',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp20',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp20',2,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp20',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp20',2,ref(4),[A]),'naloxone-dose',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp20',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp20',2,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp20',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp20',2,ref(5),[A]),offer) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp20',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp20',2,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp20',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp20',2,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp20',2,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
