% cdc2022-opioid-s3-03.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s3-03',ace_sha256('34bf4c6964c27ffadbe103316fe41c00118807193d5dc666c94a6bb6654ded02'),ulex(sha256('2a4a3e829c919b3dd3b20356e7a69585cfb305c3c57dfb531e3f2c1eb97637b6'))).
% S1: Every recommendation does not apply during a sickle-cell-disease-pain.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s3-03',1,box(1),[A]),-) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s3-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-03',1,ref(2),[A]),'sickle-cell-disease-pain',countable) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s3-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-03',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s3-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-03',1,ref(3),[A]),apply) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s3-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-03',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s3-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-03',1,ref(3),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s3-03',1,ref(2),[A])) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every recommendation does not apply during a cancer-related-pain.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s3-03',2,box(1),[A]),-) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s3-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-03',2,ref(2),[A]),'cancer-related-pain',countable) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s3-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-03',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s3-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-03',2,ref(3),[A]),apply) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s3-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-03',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s3-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-03',2,ref(3),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s3-03',2,ref(2),[A])) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every recommendation does not apply during a palliative-care.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s3-03',3,box(1),[A]),-) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s3-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-03',3,ref(2),[A]),'palliative-care',countable) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s3-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-03',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s3-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-03',3,ref(3),[A]),apply) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s3-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-03',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s3-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-03',3,ref(3),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s3-03',3,ref(2),[A])) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every recommendation does not apply during an end-of-life-care.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s3-03',4,box(1),[A]),-) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s3-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-03',4,ref(2),[A]),'end-of-life-care',countable) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s3-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-03',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s3-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-03',4,ref(3),[A]),apply) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s3-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-03',4,ref(3),[A]),1,A) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s3-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-03',4,ref(3),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s3-03',4,ref(2),[A])) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
