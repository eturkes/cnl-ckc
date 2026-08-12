% cdc2022-opioid-rec05-imp11.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec05-imp11',ace_sha256('210e34ae80e62b72a01ffa58c1c38bdb479b8bf95500698ca7d6d27944df295d'),ulex(sha256('06c1c26d6b6abe1436b8c94a276937ad072806062117417de2ab8086fdf9a3e9'))).
% S1: If an opioid-therapy has a longer-duration then a taper may require a longer-duration.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp11',1,ref(4),[A,B,C]),taper,countable) :- guideline_entity(actual,A,'opioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'longer-duration',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp11',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,'opioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'longer-duration',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp11',1,box(1),[A,B,C]),may) :- guideline_entity(actual,A,'opioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'longer-duration',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp11',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp11',1,ref(5),[A,B,C]),'longer-duration',countable) :- guideline_entity(actual,A,'opioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'longer-duration',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp11',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp11',1,ref(5),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,'opioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'longer-duration',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp11',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp11',1,ref(6),[A,B,C]),require) :- guideline_entity(actual,A,'opioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'longer-duration',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp11',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp11',1,ref(6),[A,B,C]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp11',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,'opioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'longer-duration',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp11',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp11',1,ref(6),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp11',1,ref(5),[A,B,C])) :- guideline_entity(actual,A,'opioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'longer-duration',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
