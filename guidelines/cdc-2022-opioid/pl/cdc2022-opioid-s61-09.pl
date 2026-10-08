% cdc2022-opioid-s61-09.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
:- multifile(guideline_schema_version/1).
:- discontiguous(guideline_schema_version/1).
:- multifile(guideline_document/4).
:- discontiguous(guideline_document/4).
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
:- multifile(guideline_interval/6).
:- discontiguous(guideline_interval/6).
:- multifile(guideline_recurrence/4).
:- discontiguous(guideline_recurrence/4).
guideline_schema_version(2).
guideline_document('cdc2022-opioid-s61-09',ace_sha256('6de02a6f136c6f025e8f1f52a3d67d3549115c7a2d314787edf2ed3b6bf23656'),ulex(sha256('91e746c5bcd4cd921613eef21bbe590b36f690a3199d3ecc6310436f36baacab')),temporal(sha256('453ec475d0109cb207adcaf4fbfb5a512f931e03f54cc4bb8267a18f87ed3f9c'))).
% S1: Every payer may support a broader-nonpharmacologic-intervention-array.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s61-09',1,box(1),[A]),may) :- guideline_entity(actual,A,payer,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s61-09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s61-09',1,ref(2),[A]),'broader-nonpharmacologic-intervention-array',countable) :- guideline_entity(actual,A,payer,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s61-09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s61-09',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,payer,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s61-09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s61-09',1,ref(3),[A]),support) :- guideline_entity(actual,A,payer,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s61-09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s61-09',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,payer,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s61-09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s61-09',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s61-09',1,ref(2),[A])) :- guideline_entity(actual,A,payer,countable), guideline_cardinality(actual,A,na,eq,1).
