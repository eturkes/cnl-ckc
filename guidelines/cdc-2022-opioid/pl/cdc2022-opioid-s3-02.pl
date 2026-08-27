% cdc2022-opioid-s3-02.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s3-02',ace_sha256('7067a4cc33f497750a42d2e1ea0c6a6f58d96272ace53e2ff600f6a6ad789623'),ulex(sha256('4a882a1996ea910df0fbf91fea897c0805e67d48f9e1ce2fcdbbbd9250c193ae'))).
% S1: Every recommendation is not an inflexible-standard.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s3-02',1,box(1),[A]),-) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s3-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-02',1,ref(2),[A]),'inflexible-standard',countable) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s3-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-02',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s3-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-02',1,ref(3),[A]),be) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s3-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-02',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s3-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s3-02',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s3-02',1,ref(2),[A])) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
