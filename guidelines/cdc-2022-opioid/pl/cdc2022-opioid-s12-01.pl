% cdc2022-opioid-s12-01.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s12-01',ace_sha256('7bf201e2905aee59f29bf833416fe376bdd24cb68770825ec3c0967fec0bb1af'),ulex(sha256('4c55bed7525c11e80d870254a4f876c23b6aa3c2a077242833ec32997784b01c'))).
% S1: Every category-A-recommendation has a typical-all-person-applicability.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s12-01',1,ref(2),[A]),'typical-all-person-applicability',countable) :- guideline_entity(actual,A,'category-A-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s12-01',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'category-A-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s12-01',1,ref(3),[A]),have) :- guideline_entity(actual,A,'category-A-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s12-01',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'category-A-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s12-01',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s12-01',1,ref(2),[A])) :- guideline_entity(actual,A,'category-A-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every category-A-recommendation indicates a most-circumstance-course-of-action.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s12-01',2,ref(2),[A]),'most-circumstance-course-of-action',countable) :- guideline_entity(actual,A,'category-A-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s12-01',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'category-A-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s12-01',2,ref(3),[A]),indicate) :- guideline_entity(actual,A,'category-A-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s12-01',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'category-A-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s12-01',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s12-01',2,ref(2),[A])) :- guideline_entity(actual,A,'category-A-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician can follow a most-circumstance-course-of-action.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s12-01',3,box(1),[A]),can) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s12-01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-01',3,ref(2),[A]),'most-circumstance-course-of-action',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s12-01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-01',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s12-01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-01',3,ref(3),[A]),follow) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s12-01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-01',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s12-01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-01',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s12-01',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
