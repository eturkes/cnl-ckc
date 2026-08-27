% cdc2022-opioid-s23-12.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s23-12',ace_sha256('695310bfd2d02a63040514e7e256c4276d3a3d982e36da1751e8978a25af694b'),ulex(sha256('4a882a1996ea910df0fbf91fea897c0805e67d48f9e1ce2fcdbbbd9250c193ae'))).
% S1: Every clinician should discuss a higher-opioid-dosage-opioid-use-disorder-risk with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-12',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',1,ref(2),[A]),'higher-opioid-dosage-opioid-use-disorder-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',1,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-12',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',1,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-12',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should discuss a higher-opioid-dosage-respiratory-depression-risk with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-12',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',2,ref(2),[A]),'higher-opioid-dosage-respiratory-depression-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',2,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',2,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-12',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',2,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-12',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should discuss a higher-opioid-dosage-death-risk with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-12',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',3,ref(2),[A]),'higher-opioid-dosage-death-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',3,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',3,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-12',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',3,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-12',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician should discuss a prescribed-opioid-amount-and-frequency-adherence with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-12',4,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-12',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',4,ref(2),[A]),'prescribed-opioid-amount-and-frequency-adherence',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-12',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-12',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',4,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-12',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-12',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',4,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-12',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-12',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-12',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-12',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',4,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-12',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
