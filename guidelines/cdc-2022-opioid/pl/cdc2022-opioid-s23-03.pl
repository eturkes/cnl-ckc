% cdc2022-opioid-s23-03.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
:- multifile(guideline_schema_version/1).
:- discontiguous(guideline_schema_version/1).
:- multifile(guideline_document/4).
:- discontiguous(guideline_document/4).
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
:- multifile(guideline_interval/6).
:- discontiguous(guideline_interval/6).
:- multifile(guideline_recurrence/4).
:- discontiguous(guideline_recurrence/4).
:- multifile(guideline_frequency/5).
:- discontiguous(guideline_frequency/5).
:- multifile(guideline_order/4).
:- discontiguous(guideline_order/4).
:- multifile(guideline_recurrence_window/7).
:- discontiguous(guideline_recurrence_window/7).
:- multifile(guideline_range/3).
:- discontiguous(guideline_range/3).
guideline_schema_version(3).
guideline_document('cdc2022-opioid-s23-03',ace_sha256(fcc19657d5963dfaa8a525edfee197cae51269bf48971df34387c38a48638293),ulex(sha256('0b669018d532f506c518a6b0edffa4994f771b930477e7e2b36faf50d2b75d33')),temporal(sha256(d7b0d6c9af49f8d4b3dc12370d6b1cc20e923a69a5ce745ee26a1bc0f21effb6))).
% S1: Every clinician should advise a constipation with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-03',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',1,ref(2),[A]),constipation,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',1,ref(4),[A]),advise) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-03',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',1,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-03',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should advise a dry-mouth with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-03',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',2,ref(2),[A]),'dry-mouth',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',2,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',2,ref(4),[A]),advise) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-03',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',2,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-03',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should advise a nausea with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-03',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',3,ref(2),[A]),nausea,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',3,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',3,ref(4),[A]),advise) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-03',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',3,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-03',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician should advise a vomiting with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-03',4,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',4,ref(2),[A]),vomiting,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',4,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',4,ref(4),[A]),advise) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-03',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',4,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-03',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every clinician should advise a drowsiness with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-03',5,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',5,ref(2),[A]),drowsiness,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',5,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',5,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',5,ref(4),[A]),advise) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',5,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',5,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-03',5,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',5,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-03',5,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S6: Every clinician should advise a confusion with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-03',6,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',6,ref(2),[A]),confusion,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',6,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',6,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',6,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-03',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',6,ref(4),[A]),advise) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',6,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',6,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-03',6,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-03',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',6,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-03',6,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S7: Every clinician should advise a tolerance with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-03',7,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',7,ref(2),[A]),tolerance,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',7,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',7,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',7,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-03',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',7,ref(4),[A]),advise) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',7,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',7,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-03',7,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-03',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',7,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-03',7,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S8: Every clinician should advise a physical-dependence with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-03',8,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',8,ref(2),[A]),'physical-dependence',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',8,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',8,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',8,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-03',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',8,ref(4),[A]),advise) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',8,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',8,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-03',8,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-03',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',8,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-03',8,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S9: Every clinician should advise an opioid-withdrawal-symptom with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-03',9,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',9,ref(2),[A]),'opioid-withdrawal-symptom',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',9,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-03',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',9,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-03',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',9,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-03',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',9,ref(4),[A]),advise) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',9,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-03',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',9,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-03',9,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-03',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-03',9,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-03',9,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
