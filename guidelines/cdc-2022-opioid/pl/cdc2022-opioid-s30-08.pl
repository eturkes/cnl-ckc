% cdc2022-opioid-s30-08.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s30-08',ace_sha256('18bfeff0c21ec72bd5d1bffe58028994fc7f269dd67e2d8ea9534fa95b86e404'),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If a clinician uses a medication-combination for a patient then the clinician should use the medication-combination cautiously for the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s30-08',1,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'medication-combination',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,use), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), \+ guideline_exclusion('cdc-2022-opioid').
guideline_event('$guideline_id'(context,'cdc2022-opioid-s30-08',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-08',1,ref(5),[A,B,C,D]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'medication-combination',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,use), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-08',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-08',1,ref(5),[A,B,C,D]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'medication-combination',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,use), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-08',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-08',1,ref(5),[A,B,C,D]),2,B) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'medication-combination',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,use), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s30-08',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-08',1,ref(5),[A,B,C,D]),for,C) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'medication-combination',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,use), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s30-08',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-08',1,ref(5),[A,B,C,D]),manner,cautiously) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'medication-combination',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,use), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
% S2: If a clinician uses a medication-combination for a patient then it is recommended that the clinician does not expose the patient to a synergistic-medication-risk.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s30-08',2,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'medication-combination',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,use), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-s30-08',2,box(1),[A,B,C,D]),'$guideline_id'(context,'cdc2022-opioid-s30-08',2,box(2),[A,B,C,D]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'medication-combination',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,use), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-08',2,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-08',2,ref(5),[A,B,C,D]),'synergistic-medication-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'medication-combination',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,use), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-08',2,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-08',2,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'medication-combination',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,use), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s30-08',2,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-08',2,ref(6),[A,B,C,D]),expose) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'medication-combination',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,use), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-08',2,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-08',2,ref(6),[A,B,C,D]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'medication-combination',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,use), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-08',2,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-08',2,ref(6),[A,B,C,D]),2,C) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'medication-combination',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,use), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s30-08',2,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-08',2,ref(6),[A,B,C,D]),to,'$guideline_id'(product,'cdc2022-opioid-s30-08',2,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'medication-combination',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,use), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
