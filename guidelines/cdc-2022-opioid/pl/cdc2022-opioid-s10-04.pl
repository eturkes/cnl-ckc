% cdc2022-opioid-s10-04.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s10-04',ace_sha256(fde269a20d51ccbb3e94c37c83aeb3a0515b89c34d2435ff44a5ea38170ca92b),ulex(sha256('91e746c5bcd4cd921613eef21bbe590b36f690a3199d3ecc6310436f36baacab')),temporal(sha256('453ec475d0109cb207adcaf4fbfb5a512f931e03f54cc4bb8267a18f87ed3f9c'))).
% S1: Every recommendation does not address an under-18-opioid-pain-medication-use.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s10-04',1,box(1),[A]),-) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s10-04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s10-04',1,ref(2),[A]),'under-18-opioid-pain-medication-use',countable) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s10-04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s10-04',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s10-04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s10-04',1,ref(3),[A]),address) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s10-04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s10-04',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s10-04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s10-04',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s10-04',1,ref(2),[A])) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1).
