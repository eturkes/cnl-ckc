% cdc2022-opioid-rec03-imp06.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
:- multifile(guideline_frequency/5).
:- discontiguous(guideline_frequency/5).
:- multifile(guideline_order/4).
:- discontiguous(guideline_order/4).
:- multifile(guideline_recurrence_window/7).
:- discontiguous(guideline_recurrence_window/7).
:- multifile(guideline_range/3).
:- discontiguous(guideline_range/3).
:- multifile(guideline_grade/5).
:- discontiguous(guideline_grade/5).
:- multifile(guideline_preference/4).
:- discontiguous(guideline_preference/4).
:- multifile(guideline_qualifier/4).
:- discontiguous(guideline_qualifier/4).
guideline_schema_version(1).
guideline_document('cdc2022-opioid-rec03-imp06',ace_sha256('29ce494d3d88d2a86957e04e8f3c1e4d47d92fd8cb7db83d49c91055f00352e6'),ulex(sha256('6028ca436ba3763c4e3a5f23e316b6a1b582cdb7dc76e395f1787016efb02bee')),vocabulary(sha256(c1f07c5282df4f4d69b5a8c2ef4b1fbe8beae14dbb75f5931ece77ea612f2d95))).
% S1: Every clinician should not select a methadone-treatment for a first-choice-ER-LA-decision.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec03-imp06',1,box(1),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec03-imp06',1,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec03-imp06',1,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec03-imp06',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',1,ref(2),[A]),'methadone-treatment',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec03-imp06',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec03-imp06',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',1,ref(3),[A]),'first-choice-ER-LA-decision',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec03-imp06',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec03-imp06',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',1,ref(4),[A]),select) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec03-imp06',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec03-imp06',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec03-imp06',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',1,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every methadone-treatment should not be first-line for an ER-LA-opioid.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec03-imp06',2,box(1),[A]),-) :- guideline_entity(actual,A,'methadone-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec03-imp06',2,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec03-imp06',2,box(2),[A]),should) :- guideline_entity(actual,A,'methadone-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec03-imp06',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',2,ref(2),[A]),'ER-LA-opioid',countable) :- guideline_entity(actual,A,'methadone-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec03-imp06',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'methadone-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_property('$guideline_id'(context,'cdc2022-opioid-rec03-imp06',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',2,ref(3),[A]),'first-line',pos) :- guideline_entity(actual,A,'methadone-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec03-imp06',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',2,ref(4),[A]),be) :- guideline_entity(actual,A,'methadone-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec03-imp06',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,'methadone-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec03-imp06',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',2,ref(3),[A])) :- guideline_entity(actual,A,'methadone-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_preference('$guideline_id'(context,'cdc2022-opioid-rec03-imp06',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',2,ref(4),[A]),A,none) :- guideline_entity(actual,A,'methadone-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec03-imp06',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',2,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec03-imp06',2,ref(2),[A])) :- guideline_entity(actual,A,'methadone-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
