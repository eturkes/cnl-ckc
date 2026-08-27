% cdc2022-opioid-rec11-imp06.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec11-imp06',ace_sha256('03f4e1dff04c8f9d9c4baa729128e8f78ab787c874e880371201a12391146c1f'),ulex(sha256('4a882a1996ea910df0fbf91fea897c0805e67d48f9e1ce2fcdbbbd9250c193ae'))).
% S1: Every clinician should use a gradual-benzodiazepine-taper before a benzodiazepine-discontinuation.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec11-imp06',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',1,ref(2),[A]),'gradual-benzodiazepine-taper',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',1,ref(3),[A]),'benzodiazepine-discontinuation',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',1,ref(4),[A]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',1,ref(4),[A]),before,'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
