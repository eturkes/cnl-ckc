% cdc2022-opioid-s50-03.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s50-03',ace_sha256('22260fd6f6ff38c2a7d861e8cf202b5706c47910b4611904a7073e19c670fc01'),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If a clinical-practice-organization has a naloxone-training-resource then the clinical-practice-organization can facilitate a naloxone-coprescribing.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s50-03',1,box(1),[A,B,C]),can) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'naloxone-training-resource',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s50-03',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s50-03',1,ref(4),[A,B,C]),'naloxone-coprescribing',countable) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'naloxone-training-resource',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s50-03',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s50-03',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'naloxone-training-resource',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s50-03',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s50-03',1,ref(5),[A,B,C]),facilitate) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'naloxone-training-resource',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s50-03',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s50-03',1,ref(5),[A,B,C]),1,A) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'naloxone-training-resource',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s50-03',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s50-03',1,ref(5),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s50-03',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,'clinical-practice-organization',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'naloxone-training-resource',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S2: Every pharmacist-collaborative-practice-model can facilitate a naloxone-coprescribing.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s50-03',2,box(1),[A]),can) :- guideline_entity(actual,A,'pharmacist-collaborative-practice-model',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s50-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s50-03',2,ref(2),[A]),'naloxone-coprescribing',countable) :- guideline_entity(actual,A,'pharmacist-collaborative-practice-model',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s50-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s50-03',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'pharmacist-collaborative-practice-model',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s50-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s50-03',2,ref(3),[A]),facilitate) :- guideline_entity(actual,A,'pharmacist-collaborative-practice-model',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s50-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s50-03',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'pharmacist-collaborative-practice-model',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s50-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s50-03',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s50-03',2,ref(2),[A])) :- guideline_entity(actual,A,'pharmacist-collaborative-practice-model',countable), guideline_cardinality(actual,A,na,eq,1).
