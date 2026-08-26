% cdc2022-opioid-s54-02.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s54-02',ace_sha256('1053b1b9981a85374f3d09418b866928fdb506e120ba097fae6ba9afacbdaa10'),ulex(sha256('4c55bed7525c11e80d870254a4f876c23b6aa3c2a077242833ec32997784b01c'))).
% S1: Every clinician should have an unexpected-result-response-plan before a toxicology-testing.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s54-02',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s54-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-02',1,ref(2),[A]),'unexpected-result-response-plan',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s54-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-02',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s54-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-02',1,ref(3),[A]),'toxicology-testing',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s54-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-02',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s54-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-02',1,ref(4),[A]),have) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s54-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-02',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s54-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-02',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s54-02',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s54-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s54-02',1,ref(4),[A]),before,'$guideline_id'(product,'cdc2022-opioid-s54-02',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
