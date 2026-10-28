% cdc2022-opioid-s34-22.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s34-22',ace_sha256('2e679993ddad16faa2cefc9011164b8152a531b6b891c07f11dc6eaf7d8f5b52'),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If a patient needs an additional-caution then an available-lower-opioid-dose-formulation can facilitate a dosing for the patient.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s34-22',1,ref(4),[A,B,C]),'available-lower-opioid-dose-formulation',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'additional-caution',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,need), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s34-22',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'additional-caution',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,need), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s34-22',1,box(1),[A,B,C]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'additional-caution',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,need), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s34-22',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s34-22',1,ref(5),[A,B,C]),dosing,countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'additional-caution',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,need), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s34-22',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s34-22',1,ref(5),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'additional-caution',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,need), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s34-22',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s34-22',1,ref(6),[A,B,C]),facilitate) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'additional-caution',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,need), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s34-22',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s34-22',1,ref(6),[A,B,C]),1,'$guideline_id'(product,'cdc2022-opioid-s34-22',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'additional-caution',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,need), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s34-22',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s34-22',1,ref(6),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s34-22',1,ref(5),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'additional-caution',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,need), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s34-22',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s34-22',1,ref(6),[A,B,C]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'additional-caution',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,need), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S2: A lower-opioid-dose-formulation-example contains exactly 2500 hydrocodone-bitartrate-micrograms and contains exactly 325 acetaminophen-milligrams.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s34-22',2,ref(1),[]),'lower-opioid-dose-formulation-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s34-22',2,ref(1),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s34-22',2,ref(2),[]),contain).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s34-22',2,ref(2),[]),1,'$guideline_id'(product,'cdc2022-opioid-s34-22',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s34-22',2,ref(2),[]),2,'$guideline_id'(product,'cdc2022-opioid-s34-22',2,ref(3),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s34-22',2,ref(3),[]),'hydrocodone-bitartrate-microgram',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s34-22',2,ref(3),[]),na,exactly,2500).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s34-22',2,ref(4),[]),contain).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s34-22',2,ref(4),[]),1,'$guideline_id'(product,'cdc2022-opioid-s34-22',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s34-22',2,ref(4),[]),2,'$guideline_id'(product,'cdc2022-opioid-s34-22',2,ref(5),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s34-22',2,ref(5),[]),'acetaminophen-milligram',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s34-22',2,ref(5),[]),na,exactly,325).
