% cdc2022-opioid-rec05-imp07.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec05-imp07',ace_sha256(a14f67b915919aef2c815be23a2909a72f4424dd70a95c3f3781229df2a36438),ulex(sha256('4c55bed7525c11e80d870254a4f876c23b6aa3c2a077242833ec32997784b01c'))).
% S1: Every clinician should collaborate with a patient on an opioid-taper-plan.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp07',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',1,ref(2),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',1,ref(3),[A]),'opioid-taper-plan',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',1,ref(4),[A]),collaborate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',1,ref(4),[A]),on,'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',1,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should include a patient in a taper-speed-decision and should include a patient in a taper-pause-decision.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp07',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',2,ref(2),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',2,ref(3),[A]),'taper-speed-decision',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',2,ref(4),[A]),include) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp07',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',2,ref(4),[A]),in,'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp07',2,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp07',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',2,ref(5),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp07',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',2,ref(5),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp07',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',2,ref(6),[A]),'taper-pause-decision',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp07',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',2,ref(6),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp07',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',2,ref(7),[A]),include) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp07',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',2,ref(7),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp07',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',2,ref(7),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',2,ref(5),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp07',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',2,ref(7),[A]),in,'$guideline_id'(product,'cdc2022-opioid-rec05-imp07',2,ref(6),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
