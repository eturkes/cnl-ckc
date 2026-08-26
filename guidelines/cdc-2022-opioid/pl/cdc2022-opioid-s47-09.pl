% cdc2022-opioid-s47-09.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s47-09',ace_sha256('3467b670827b1b856bf4f377375503ec03e578356f4fb29dc635451be9419701'),ulex(sha256('4c55bed7525c11e80d870254a4f876c23b6aa3c2a077242833ec32997784b01c'))).
% S1: Every clinician should use a noncoercive-contraceptive-counseling with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s47-09',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s47-09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-09',1,ref(2),[A]),'noncoercive-contraceptive-counseling',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s47-09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-09',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s47-09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-09',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s47-09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-09',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s47-09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-09',1,ref(4),[A]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s47-09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-09',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s47-09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-09',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s47-09',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s47-09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-09',1,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s47-09',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should discuss a contraceptive-option with a patient during a noncoercive-contraceptive-counseling.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s47-09',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s47-09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-09',2,ref(2),[A]),'contraceptive-option',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s47-09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-09',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s47-09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-09',2,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s47-09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-09',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s47-09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-09',2,ref(4),[A]),'noncoercive-contraceptive-counseling',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s47-09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-09',2,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s47-09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-09',2,ref(5),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s47-09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-09',2,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s47-09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-09',2,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s47-09',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s47-09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-09',2,ref(5),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s47-09',2,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s47-09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-09',2,ref(5),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s47-09',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
