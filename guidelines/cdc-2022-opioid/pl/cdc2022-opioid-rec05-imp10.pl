% cdc2022-opioid-rec05-imp10.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec05-imp10',ace_sha256('00a5987de7c27a3945b4616f7e6b6eed8d9e86318ac3261d3e61c06d0d225951'),ulex(sha256(bac3441c3cba94d1d496ba3b04390eae38a0826cd30a0a0f713112b2172476ce))).
% S1: If a clinician reduces an opioid then the clinician should use a withdrawal-minimizing-taper.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp10',1,box(1),[A,B,C]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,reduce), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp10',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',1,ref(4),[A,B,C]),'withdrawal-minimizing-taper',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,reduce), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp10',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,reduce), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp10',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',1,ref(5),[A,B,C]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,reduce), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp10',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',1,ref(5),[A,B,C]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,reduce), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp10',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',1,ref(5),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,reduce), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S2: If a clinician discontinues an opioid then the clinician should use a withdrawal-minimizing-taper.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp10',2,box(1),[A,B,C]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,discontinue), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp10',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',2,ref(4),[A,B,C]),'withdrawal-minimizing-taper',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,discontinue), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp10',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',2,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,discontinue), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp10',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',2,ref(5),[A,B,C]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,discontinue), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp10',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',2,ref(5),[A,B,C]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,discontinue), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp10',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',2,ref(5),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',2,ref(4),[A,B,C])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,discontinue), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
