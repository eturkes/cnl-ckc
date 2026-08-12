% cdc2022-opioid-s23-06.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s23-06',ace_sha256('77706c97fe3020f34213eb251a640d375a85d93b1d1b3edf79780555299871da'),ulex(sha256('06c1c26d6b6abe1436b8c94a276937ad072806062117417de2ab8086fdf9a3e9'))).
% S1: Every clinician should avoid a stool-softener-without-another-laxative.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-06',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-06',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-06',1,ref(2),[A]),'stool-softener-without-another-laxative',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-06',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-06',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-06',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-06',1,ref(3),[A]),avoid) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-06',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-06',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-06',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-06',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-06',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should avoid a fiber-laxative-without-another-laxative.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-06',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-06',2,ref(2),[A]),'fiber-laxative-without-another-laxative',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-06',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-06',2,ref(3),[A]),avoid) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-06',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-06',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-06',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
