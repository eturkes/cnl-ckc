% cdc2022-opioid-s9-04.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s9-04',ace_sha256(d2694e97cea76ead5a23b518a6efabc20d5a5d698b8f82e05e7b95700c2891de),ulex(sha256('2a4a3e829c919b3dd3b20356e7a69585cfb305c3c57dfb531e3f2c1eb97637b6'))).
% S1: Every pain-duration-definition is an approximate-guide.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-04',1,ref(2),[A]),'approximate-guide',countable) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-04',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-04',1,ref(3),[A]),be) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-04',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-04',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-04',1,ref(2),[A])) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every pain-duration-definition is not an absolute-boundary.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-04',2,box(1),[A]),-) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',2,ref(2),[A]),'absolute-boundary',countable) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',2,ref(3),[A]),be) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-04',2,ref(2),[A])) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every pain-duration-definition can facilitate a recommendation-consideration.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-04',3,box(1),[A]),can) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-04',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',3,ref(2),[A]),'recommendation-consideration',countable) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-04',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-04',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',3,ref(3),[A]),facilitate) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-04',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-04',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-04',3,ref(2),[A])) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every pain-duration-definition can facilitate a practical-recommendation-use.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-04',4,box(1),[A]),can) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-04',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',4,ref(2),[A]),'practical-recommendation-use',countable) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-04',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-04',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',4,ref(3),[A]),facilitate) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-04',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',4,ref(3),[A]),1,A) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-04',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',4,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-04',4,ref(2),[A])) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
