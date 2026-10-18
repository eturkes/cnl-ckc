% cdc2022-opioid-s37-10.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s37-10',ace_sha256(ff5324873d05402226c5176ff4a111a4fcde27f43cdb888c06bea419f7903f08),ulex(sha256(d08a8bd3e80ef6e01301a079bb5496315f35233bf199aa89b2d2ccc93b858776)),vocabulary(sha256('9d7632637554f8cbfe72e7d34a9b9d14c29eb8593aebb1c128a870c5dcf77ead'))).
% S1: If a patient uses at least 2 respiratory-depressant-medications then the patient can possibly require a respiratory-depression-risk-reducing-medication-taper.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s37-10',1,box(1),[A,B,C]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'respiratory-depressant-medication',countable), guideline_cardinality(actual,B,na,D,E), guideline_satisfies(D,E,geq,2), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s37-10',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s37-10',1,ref(4),[A,B,C]),'respiratory-depression-risk-reducing-medication-taper',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'respiratory-depressant-medication',countable), guideline_cardinality(actual,B,na,D,E), guideline_satisfies(D,E,geq,2), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s37-10',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s37-10',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'respiratory-depressant-medication',countable), guideline_cardinality(actual,B,na,D,E), guideline_satisfies(D,E,geq,2), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s37-10',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s37-10',1,ref(5),[A,B,C]),possibility,possibly) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'respiratory-depressant-medication',countable), guideline_cardinality(actual,B,na,D,E), guideline_satisfies(D,E,geq,2), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s37-10',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s37-10',1,ref(5),[A,B,C]),require) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'respiratory-depressant-medication',countable), guideline_cardinality(actual,B,na,D,E), guideline_satisfies(D,E,geq,2), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-10',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s37-10',1,ref(5),[A,B,C]),1,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'respiratory-depressant-medication',countable), guideline_cardinality(actual,B,na,D,E), guideline_satisfies(D,E,geq,2), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-10',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s37-10',1,ref(5),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s37-10',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'respiratory-depressant-medication',countable), guideline_cardinality(actual,B,na,D,E), guideline_satisfies(D,E,geq,2), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
