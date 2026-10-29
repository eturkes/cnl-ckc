% cdc2022-opioid-s60-05.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s60-05',ace_sha256('5ab838c14d252bd697ecb35d30a8e04e57186d8b635c1cd3b6e4664a44f64e06'),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If there is a clinician then it is recommended that the clinician provides a qualified-opioid-use-disorder-treatment-specialist-search-assistance to a patient for a qualified-opioid-use-disorder-treatment-specialist and arranges a specialist-follow-up for the patient with the qualified-opioid-use-disorder-treatment-specialist and coordinates a continuing-care for the patient with the qualified-opioid-use-disorder-treatment-specialist.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(2),[A]),'qualified-opioid-use-disorder-treatment-specialist-search-assistance',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(4),[A]),'qualified-opioid-use-disorder-treatment-specialist',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(5),[A]),provide) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(5),[A]),3,'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(5),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(6),[A]),'specialist-follow-up',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(6),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(7),[A]),arrange) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(7),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(7),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(6),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(7),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(7),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(8),[A]),'continuing-care',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(8),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(9),[A]),coordinate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(9),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(9),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(8),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(9),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(9),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
