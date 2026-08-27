% cdc2022-opioid-s33-03.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s33-03',ace_sha256('0a1d1d27cc71b9db7b6474be9674dfee04b35170b616e0ea80b99060378c2da7'),ulex(sha256('4a882a1996ea910df0fbf91fea897c0805e67d48f9e1ce2fcdbbbd9250c193ae'))).
% S1: Every clinician should not use a calculated-MME-dosage for an opioid-conversion-dosage.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s33-03',1,box(1),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-s33-03',1,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-s33-03',1,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s33-03',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-03',1,ref(2),[A]),'calculated-MME-dosage',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s33-03',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-03',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s33-03',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-03',1,ref(3),[A]),'opioid-conversion-dosage',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s33-03',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-03',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s33-03',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-03',1,ref(4),[A]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s33-03',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-03',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s33-03',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-03',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s33-03',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s33-03',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-03',1,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s33-03',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should use a substantially-lower-opioid-dosage during an opioid-conversion.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s33-03',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s33-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-03',2,ref(2),[A]),'substantially-lower-opioid-dosage',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s33-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-03',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s33-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-03',2,ref(3),[A]),'opioid-conversion',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s33-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-03',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s33-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-03',2,ref(4),[A]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s33-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-03',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s33-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-03',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s33-03',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s33-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s33-03',2,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s33-03',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
