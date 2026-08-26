% cdc2022-opioid-rec11-imp07.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec11-imp07',ace_sha256('8b4060a8450c6275f4f8a2257d8ef6bc8bd9031f14507e553be66969bc25316b'),ulex(sha256('4c55bed7525c11e80d870254a4f876c23b6aa3c2a077242833ec32997784b01c'))).
% S1: Every clinician should individualize a benzodiazepine-taper-rate.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec11-imp07',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec11-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp07',1,ref(2),[A]),'benzodiazepine-taper-rate',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec11-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp07',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec11-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp07',1,ref(3),[A]),individualize) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec11-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp07',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec11-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp07',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec11-imp07',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
