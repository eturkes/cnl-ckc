% cdc2022-opioid-s39-22.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s39-22',ace_sha256('45856e48ed5ad8ae85372ca4e4f94246d6c025be92fe8513c86e9672202f8fa8'),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If a patient has a taper-progress-failure-despite-opioid-related-harm then the patient can possibly have an undiagnosed-opioid-use-disorder.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s39-22',1,box(1),[A,B,C]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'taper-progress-failure-despite-opioid-related-harm',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-22',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s39-22',1,ref(4),[A,B,C]),'undiagnosed-opioid-use-disorder',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'taper-progress-failure-despite-opioid-related-harm',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-22',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s39-22',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'taper-progress-failure-despite-opioid-related-harm',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s39-22',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s39-22',1,ref(5),[A,B,C]),possibility,possibly) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'taper-progress-failure-despite-opioid-related-harm',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s39-22',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s39-22',1,ref(5),[A,B,C]),have) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'taper-progress-failure-despite-opioid-related-harm',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-22',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s39-22',1,ref(5),[A,B,C]),1,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'taper-progress-failure-despite-opioid-related-harm',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-22',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s39-22',1,ref(5),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s39-22',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'taper-progress-failure-despite-opioid-related-harm',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S2: If a patient has an unanticipated-tapering-challenge then the patient can possibly have an undiagnosed-opioid-use-disorder.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s39-22',2,box(1),[A,B,C]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'unanticipated-tapering-challenge',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-22',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s39-22',2,ref(4),[A,B,C]),'undiagnosed-opioid-use-disorder',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'unanticipated-tapering-challenge',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-22',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s39-22',2,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'unanticipated-tapering-challenge',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s39-22',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s39-22',2,ref(5),[A,B,C]),possibility,possibly) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'unanticipated-tapering-challenge',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s39-22',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s39-22',2,ref(5),[A,B,C]),have) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'unanticipated-tapering-challenge',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-22',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s39-22',2,ref(5),[A,B,C]),1,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'unanticipated-tapering-challenge',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-22',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s39-22',2,ref(5),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s39-22',2,ref(4),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'unanticipated-tapering-challenge',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S3: An unanticipated-tapering-challenge-example is a taper-progress-failure-despite-opioid-related-harm-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s39-22',3,ref(1),[]),'unanticipated-tapering-challenge-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s39-22',3,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s39-22',3,ref(2),[]),'taper-progress-failure-despite-opioid-related-harm-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s39-22',3,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s39-22',3,ref(3),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s39-22',3,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s39-22',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s39-22',3,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s39-22',3,ref(2),[])).
