% cdc2022-opioid-s61-07.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s61-07',ace_sha256('6683ac7a011f99eb54db9d2aad21b2b90f0f3a07b80a9a74716b114506001d3e'),ulex(sha256('4a882a1996ea910df0fbf91fea897c0805e67d48f9e1ce2fcdbbbd9250c193ae'))).
% S1: Every clinician should monitor a longitudinal-patient-progress.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s61-07',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s61-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s61-07',1,ref(2),[A]),'longitudinal-patient-progress',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s61-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s61-07',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s61-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s61-07',1,ref(3),[A]),monitor) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s61-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s61-07',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s61-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s61-07',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s61-07',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: If a clinician monitors a longitudinal-patient-progress then the clinician should adjust a treatment-protocol to the longitudinal-patient-progress.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s61-07',2,box(1),[A,B,C]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'longitudinal-patient-progress',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,monitor), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s61-07',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s61-07',2,ref(4),[A,B,C]),'treatment-protocol',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'longitudinal-patient-progress',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,monitor), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s61-07',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s61-07',2,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'longitudinal-patient-progress',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,monitor), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s61-07',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s61-07',2,ref(5),[A,B,C]),adjust) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'longitudinal-patient-progress',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,monitor), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s61-07',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s61-07',2,ref(5),[A,B,C]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'longitudinal-patient-progress',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,monitor), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s61-07',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s61-07',2,ref(5),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s61-07',2,ref(4),[A,B,C])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'longitudinal-patient-progress',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,monitor), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s61-07',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s61-07',2,ref(5),[A,B,C]),to,B) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'longitudinal-patient-progress',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,monitor), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
