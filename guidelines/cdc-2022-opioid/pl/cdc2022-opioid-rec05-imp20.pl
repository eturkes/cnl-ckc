% cdc2022-opioid-rec05-imp20.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
:- multifile(guideline_grade/5).
:- discontiguous(guideline_grade/5).
:- multifile(guideline_preference/4).
:- discontiguous(guideline_preference/4).
:- multifile(guideline_qualifier/4).
:- discontiguous(guideline_qualifier/4).
:- multifile(guideline_unit/3).
:- discontiguous(guideline_unit/3).
guideline_schema_version(1).
guideline_document('cdc2022-opioid-rec05-imp20',ace_sha256('12b7095a512dbcb298dd1d4c1b874ca3c012c1bd39d850d9c002ddfeeaa2fee1'),ulex(sha256(d08a8bd3e80ef6e01301a079bb5496315f35233bf199aa89b2d2ccc93b858776)),vocabulary(sha256('9d7632637554f8cbfe72e7d34a9b9d14c29eb8593aebb1c128a870c5dcf77ead'))).
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
