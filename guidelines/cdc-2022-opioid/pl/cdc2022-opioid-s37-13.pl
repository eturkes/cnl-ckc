% cdc2022-opioid-s37-13.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
:- multifile(guideline_exclusion/1).
:- discontiguous(guideline_exclusion/1).
:- multifile(guideline_supersession/4).
:- discontiguous(guideline_supersession/4).
:- multifile(guideline_evaluation_date/1).
:- discontiguous(guideline_evaluation_date/1).
guideline_schema_version(1).
guideline_document('cdc2022-opioid-s37-13',ace_sha256(b46991bd75de0e61c467cb6ee787cb12a316c8224da7919b8e30664b2bc404e5),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If a patient does not take a prescribed-opioid then the patient does not require an opioid-taper.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s37-13',1,box(2),[A]),-) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(B,C,'prescribed-opioid',countable), guideline_operator(actual,B,-), guideline_cardinality(B,C,na,eq,1), guideline_event(B,D,take), guideline_arg(B,D,1,A), guideline_arg(B,D,2,C).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s37-13',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-13',1,ref(4),[A]),'opioid-taper',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(B,C,'prescribed-opioid',countable), guideline_operator(actual,B,-), guideline_cardinality(B,C,na,eq,1), guideline_event(B,D,take), guideline_arg(B,D,1,A), guideline_arg(B,D,2,C).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s37-13',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-13',1,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(B,C,'prescribed-opioid',countable), guideline_operator(actual,B,-), guideline_cardinality(B,C,na,eq,1), guideline_event(B,D,take), guideline_arg(B,D,1,A), guideline_arg(B,D,2,C).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s37-13',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-13',1,ref(5),[A]),require) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(B,C,'prescribed-opioid',countable), guideline_operator(actual,B,-), guideline_cardinality(B,C,na,eq,1), guideline_event(B,D,take), guideline_arg(B,D,1,A), guideline_arg(B,D,2,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-13',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-13',1,ref(5),[A]),1,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(B,C,'prescribed-opioid',countable), guideline_operator(actual,B,-), guideline_cardinality(B,C,na,eq,1), guideline_event(B,D,take), guideline_arg(B,D,1,A), guideline_arg(B,D,2,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-13',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-13',1,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s37-13',1,ref(4),[A])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(B,C,'prescribed-opioid',countable), guideline_operator(actual,B,-), guideline_cardinality(B,C,na,eq,1), guideline_event(B,D,take), guideline_arg(B,D,1,A), guideline_arg(B,D,2,C).
% S2: A prescribed-opioid-nonuse-example is an all-obtained-opioid-diversion-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s37-13',2,ref(1),[]),'prescribed-opioid-nonuse-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s37-13',2,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s37-13',2,ref(2),[]),'all-obtained-opioid-diversion-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s37-13',2,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s37-13',2,ref(3),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s37-13',2,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s37-13',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s37-13',2,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s37-13',2,ref(2),[])).
