% cdc2022-opioid-s33-04.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
:- multifile(guideline_unit/3).
:- discontiguous(guideline_unit/3).
guideline_schema_version(1).
guideline_document('cdc2022-opioid-s33-04',ace_sha256('52fe7dff465f4c665a1d1076bebb5f003ab0759f9a876d894fe1908428f25a6d'),ulex(sha256(d08a8bd3e80ef6e01301a079bb5496315f35233bf199aa89b2d2ccc93b858776)),vocabulary(sha256('9d7632637554f8cbfe72e7d34a9b9d14c29eb8593aebb1c128a870c5dcf77ead'))).
% S1: If a clinician converts a methadone-dose then the clinician should particularly convert the methadone-dose cautiously.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s33-04',1,box(1),[A,B,C]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'methadone-dose',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,convert), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s33-04',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s33-04',1,ref(4),[A,B,C]),focus,particularly) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'methadone-dose',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,convert), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s33-04',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s33-04',1,ref(4),[A,B,C]),convert) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'methadone-dose',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,convert), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s33-04',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s33-04',1,ref(4),[A,B,C]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'methadone-dose',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,convert), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s33-04',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s33-04',1,ref(4),[A,B,C]),2,B) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'methadone-dose',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,convert), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s33-04',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s33-04',1,ref(4),[A,B,C]),manner,cautiously) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'methadone-dose',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,convert), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
