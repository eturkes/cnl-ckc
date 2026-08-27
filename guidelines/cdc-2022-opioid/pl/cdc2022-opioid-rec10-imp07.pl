% cdc2022-opioid-rec10-imp07.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec10-imp07',ace_sha256(efae824758307ae5c008e6d51136c33268a5540e8f4b102a0b8c008c9f75add7),ulex(sha256('4a882a1996ea910df0fbf91fea897c0805e67d48f9e1ce2fcdbbbd9250c193ae'))).
% S1: Every clinician should explain an expected-prescribed-medication-presence to a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp07',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',1,ref(2),[A]),'expected-prescribed-medication-presence',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',1,ref(4),[A]),explain) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',1,ref(4),[A]),to,'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should explain an expected-unreported-drug-absence to a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp07',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',2,ref(2),[A]),'expected-unreported-drug-absence',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',2,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',2,ref(4),[A]),explain) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',2,ref(4),[A]),to,'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should explain an expected-unreported-nonprescribed-controlled-substance-absence to a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp07',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',3,ref(2),[A]),'expected-unreported-nonprescribed-controlled-substance-absence',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',3,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',3,ref(4),[A]),explain) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',3,ref(4),[A]),to,'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician should ask a nonjudgmental-prescribed-drug-use-question during a patient-discussion.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp07',4,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',4,ref(2),[A]),'nonjudgmental-prescribed-drug-use-question',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',4,ref(3),[A]),'patient-discussion',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',4,ref(4),[A]),ask) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',4,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every clinician should ask a nonjudgmental-other-drug-use-question during a patient-discussion.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp07',5,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',5,ref(2),[A]),'nonjudgmental-other-drug-use-question',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',5,ref(3),[A]),'patient-discussion',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',5,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',5,ref(4),[A]),ask) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',5,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',5,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',5,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',5,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',5,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S6: Every clinician should ask a nonjudgmental-possible-unexpected-result-question during a patient-discussion.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp07',6,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',6,ref(2),[A]),'nonjudgmental-possible-unexpected-result-question',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',6,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',6,ref(3),[A]),'patient-discussion',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',6,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',6,ref(4),[A]),ask) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',6,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',6,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',6,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec10-imp07',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',6,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec10-imp07',6,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
