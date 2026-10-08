% cdc2022-opioid-s11-01.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s11-01',ace_sha256(a52bec7b9ab6afb16f6f7d80e1e89d6bce82a18f38d942913104d99e4d8af235),ulex(sha256('91e746c5bcd4cd921613eef21bbe590b36f690a3199d3ecc6310436f36baacab')),temporal(sha256('453ec475d0109cb207adcaf4fbfb5a512f931e03f54cc4bb8267a18f87ed3f9c'))).
% S1: Every opioid-pain-clinical-practice-guideline contains an opioid-use-disorder-pain-management-content.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s11-01',1,ref(2),[A]),'opioid-use-disorder-pain-management-content',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s11-01',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s11-01',1,ref(3),[A]),contain) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s11-01',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s11-01',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s11-01',1,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every opioid-pain-clinical-practice-guideline contains an opioid-use-disorder-complication-management-content.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s11-01',2,ref(2),[A]),'opioid-use-disorder-complication-management-content',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s11-01',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s11-01',2,ref(3),[A]),contain) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s11-01',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s11-01',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s11-01',2,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every opioid-pain-clinical-practice-guideline is not an opioid-use-disorder-medication-guideline.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s11-01',3,box(1),[A]),-) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s11-01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s11-01',3,ref(2),[A]),'opioid-use-disorder-medication-guideline',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s11-01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s11-01',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s11-01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s11-01',3,ref(3),[A]),be) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s11-01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s11-01',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s11-01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s11-01',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s11-01',3,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
