% cdc2022-opioid-s37-12.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s37-12',ace_sha256('5f87c672daf2c219952a9defd7609b6a7e6ccc774609f2ae5b1e9822e58f8560'),ulex(sha256('67d1410100ff45477e758b366ae03bb860a5c23e3fd444d55dc901b2f3c7d8a1')),vocabulary(sha256('9d7632637554f8cbfe72e7d34a9b9d14c29eb8593aebb1c128a870c5dcf77ead'))).
% S1: If a clinician tapers a benzodiazepine then the clinician should taper the benzodiazepine gradually.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s37-12',1,box(1),[A,B,C]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,benzodiazepine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,taper), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s37-12',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s37-12',1,ref(4),[A,B,C]),taper) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,benzodiazepine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,taper), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-12',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s37-12',1,ref(4),[A,B,C]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,benzodiazepine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,taper), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-12',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s37-12',1,ref(4),[A,B,C]),2,B) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,benzodiazepine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,taper), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s37-12',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s37-12',1,ref(4),[A,B,C]),manner,gradually) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,benzodiazepine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,taper), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
