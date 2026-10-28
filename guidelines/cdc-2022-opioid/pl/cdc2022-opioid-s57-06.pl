% cdc2022-opioid-s57-06.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s57-06',ace_sha256('2d4eb20321e66aa0d04bc9dbcb7f9aaa0209251521bed943919355c4a213e821'),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If a patient takes an opioid solely under an appropriate-medical-supervision then the patient does not meet an opioid-tolerance-criterion and does not meet an opioid-withdrawal-criterion.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s57-06',1,box(1),[A,B,C,D]),-) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'appropriate-medical-supervision',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,take), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,under,C), guideline_qualifier(actual,D,focus,solely).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s57-06',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s57-06',1,ref(5),[A,B,C,D]),'opioid-tolerance-criterion',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'appropriate-medical-supervision',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,take), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,under,C), guideline_qualifier(actual,D,focus,solely).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s57-06',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s57-06',1,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'appropriate-medical-supervision',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,take), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,under,C), guideline_qualifier(actual,D,focus,solely).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s57-06',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s57-06',1,ref(6),[A,B,C,D]),meet) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'appropriate-medical-supervision',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,take), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,under,C), guideline_qualifier(actual,D,focus,solely).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s57-06',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s57-06',1,ref(6),[A,B,C,D]),1,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'appropriate-medical-supervision',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,take), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,under,C), guideline_qualifier(actual,D,focus,solely).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s57-06',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s57-06',1,ref(6),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-s57-06',1,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'appropriate-medical-supervision',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,take), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,under,C), guideline_qualifier(actual,D,focus,solely).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s57-06',1,box(2),[A,B,C,D]),-) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'appropriate-medical-supervision',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,take), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,under,C), guideline_qualifier(actual,D,focus,solely).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s57-06',1,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s57-06',1,ref(7),[A,B,C,D]),'opioid-withdrawal-criterion',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'appropriate-medical-supervision',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,take), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,under,C), guideline_qualifier(actual,D,focus,solely).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s57-06',1,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s57-06',1,ref(7),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'appropriate-medical-supervision',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,take), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,under,C), guideline_qualifier(actual,D,focus,solely).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s57-06',1,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s57-06',1,ref(8),[A,B,C,D]),meet) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'appropriate-medical-supervision',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,take), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,under,C), guideline_qualifier(actual,D,focus,solely).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s57-06',1,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s57-06',1,ref(8),[A,B,C,D]),1,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'appropriate-medical-supervision',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,take), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,under,C), guideline_qualifier(actual,D,focus,solely).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s57-06',1,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s57-06',1,ref(8),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-s57-06',1,ref(7),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'appropriate-medical-supervision',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,take), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,under,C), guideline_qualifier(actual,D,focus,solely).
