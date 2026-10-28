% cdc2022-opioid-s12-01.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s12-01',ace_sha256('9377c870e787e783a8c012af66019e9d95953f5333d3840b13b3888bb12d231b'),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: Every category-A-recommendation can typically apply to an addressed-group.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s12-01',1,box(1),[A]),can) :- guideline_entity(actual,A,'category-A-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s12-01',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-01',1,ref(2),[A]),'addressed-group',countable) :- guideline_entity(actual,A,'category-A-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s12-01',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-01',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'category-A-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s12-01',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-01',1,ref(3),[A]),usuality,typically) :- guideline_entity(actual,A,'category-A-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s12-01',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-01',1,ref(3),[A]),apply) :- guideline_entity(actual,A,'category-A-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s12-01',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-01',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'category-A-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s12-01',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-01',1,ref(3),[A]),to,'$guideline_id'(product,'cdc2022-opioid-s12-01',1,ref(2),[A])) :- guideline_entity(actual,A,'category-A-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every category-A-recommendation indicates a most-circumstance-course-of-action.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s12-01',2,ref(2),[A]),'most-circumstance-course-of-action',countable) :- guideline_entity(actual,A,'category-A-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s12-01',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'category-A-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s12-01',2,ref(3),[A]),indicate) :- guideline_entity(actual,A,'category-A-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s12-01',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'category-A-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s12-01',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s12-01',2,ref(2),[A])) :- guideline_entity(actual,A,'category-A-recommendation',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician can follow a most-circumstance-course-of-action.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s12-01',3,box(1),[A]),can) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s12-01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-01',3,ref(2),[A]),'most-circumstance-course-of-action',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s12-01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-01',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s12-01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-01',3,ref(3),[A]),follow) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s12-01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-01',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s12-01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s12-01',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s12-01',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
