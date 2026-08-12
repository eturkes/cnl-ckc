% cdc2022-opioid-s29-03.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s29-03',ace_sha256('7767f34b21443eb5a09eccb5784fe620e7ae36c37d30f98c1856316033af25c6'),ulex(sha256(ef57ba7180864095bab88b95f11ddb1a2c858593eba715dbfbe874db09506c5b))).
% S1: Every clinician should discuss a realistic-expected-opioid-benefit with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s29-03',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',1,ref(2),[A]),'realistic-expected-opioid-benefit',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s29-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',1,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s29-03',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s29-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',1,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s29-03',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should explain a limited-long-term-opioid-pain-evidence with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s29-03',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',2,ref(2),[A]),'limited-long-term-opioid-pain-evidence',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',2,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s29-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',2,ref(4),[A]),explain) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s29-03',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s29-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',2,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s29-03',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should explain a limited-long-term-opioid-function-evidence with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s29-03',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',3,ref(2),[A]),'limited-long-term-opioid-function-evidence',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',3,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s29-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',3,ref(4),[A]),explain) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s29-03',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s29-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',3,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s29-03',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician should explain an unlikely-complete-pain-elimination with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s29-03',4,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',4,ref(2),[A]),'unlikely-complete-pain-elimination',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',4,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s29-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',4,ref(4),[A]),explain) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s29-03',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s29-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',4,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s29-03',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
