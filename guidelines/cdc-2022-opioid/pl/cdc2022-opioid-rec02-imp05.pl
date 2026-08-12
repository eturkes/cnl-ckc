% cdc2022-opioid-rec02-imp05.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec02-imp05',ace_sha256(a45a1f83fc339d7741e63e5d3d775591344bd1dbbf95a6539ab18abe0919062f),ulex(sha256('9d39595833bb1358c343511806091b34e2442561f073b432f531f68ac385a15e'))).
% S1: Every clinician should review an FDA-approved-labeling before a pharmacologic-therapy and should review a boxed-warning before a pharmacologic-therapy and should weigh a benefit against a risk before a pharmacologic-therapy.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(2),[A]),'FDA-approved-labeling',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(3),[A]),'pharmacologic-therapy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(4),[A]),review) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(4),[A]),before,'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(5),[A]),'boxed-warning',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(5),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(6),[A]),'pharmacologic-therapy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(6),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(7),[A]),review) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(7),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(7),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(5),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(7),[A]),before,'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(6),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(3),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(8),[A]),benefit,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(8),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(9),[A]),risk,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(9),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(10),[A]),'pharmacologic-therapy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(10),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(11),[A]),weigh) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(11),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(11),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(8),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(11),[A]),before,'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(10),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec02-imp05',1,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(11),[A]),against,'$guideline_id'(product,'cdc2022-opioid-rec02-imp05',1,ref(9),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
