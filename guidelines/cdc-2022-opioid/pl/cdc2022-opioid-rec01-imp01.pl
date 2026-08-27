% cdc2022-opioid-rec01-imp01.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec01-imp01',ace_sha256(f03a66470307db0b5004dcda583ae54d8f91ba03d9c9c0033c67a92f7cc88568),ulex(sha256('4a882a1996ea910df0fbf91fea897c0805e67d48f9e1ce2fcdbbbd9250c193ae'))).
% S1: Every clinician should maximize a nonopioid-pharmacologic-therapy for a specific-condition and should maximize a nonpharmacologic-therapy for a specific-condition.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec01-imp01',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',1,ref(2),[A]),'nonopioid-pharmacologic-therapy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',1,ref(3),[A]),'specific-condition',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',1,ref(4),[A]),maximize) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',1,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec01-imp01',1,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',1,ref(5),[A]),'nonpharmacologic-therapy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',1,ref(5),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',1,ref(6),[A]),'specific-condition',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',1,ref(6),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',1,ref(7),[A]),maximize) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',1,ref(7),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',1,ref(7),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',1,ref(5),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',1,ref(7),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',1,ref(6),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
