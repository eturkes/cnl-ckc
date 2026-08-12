% cdc2022-opioid-s38-17.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s38-17',ace_sha256(e8d7a7031f22e66b4a25643d11c448382c89230053555806eb387b3dcecd5c89),ulex(sha256('2a4a3e829c919b3dd3b20356e7a69585cfb305c3c57dfb531e3f2c1eb97637b6'))).
% S1: Every clinician may explain a slow-withdrawal-minimizing-taper-expectation to a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s38-17',1,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-17',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-17',1,ref(2),[A]),'slow-withdrawal-minimizing-taper-expectation',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-17',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-17',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-17',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-17',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-17',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-17',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s38-17',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-17',1,ref(4),[A]),explain) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-17',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-17',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-17',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-17',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s38-17',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s38-17',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-17',1,ref(4),[A]),to,'$guideline_id'(product,'cdc2022-opioid-s38-17',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician may explain an initial-pain-worsening-and-usual-improvement-expectation to a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s38-17',2,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-17',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-17',2,ref(2),[A]),'initial-pain-worsening-and-usual-improvement-expectation',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-17',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-17',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-17',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-17',2,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-17',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-17',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s38-17',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-17',2,ref(4),[A]),explain) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-17',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-17',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-17',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-17',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s38-17',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s38-17',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-17',2,ref(4),[A]),to,'$guideline_id'(product,'cdc2022-opioid-s38-17',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician may provide a taper-process-support to a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s38-17',3,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-17',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-17',3,ref(2),[A]),'taper-process-support',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-17',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-17',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-17',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-17',3,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-17',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-17',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s38-17',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-17',3,ref(4),[A]),provide) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-17',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-17',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-17',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-17',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s38-17',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s38-17',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-17',3,ref(4),[A]),to,'$guideline_id'(product,'cdc2022-opioid-s38-17',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
