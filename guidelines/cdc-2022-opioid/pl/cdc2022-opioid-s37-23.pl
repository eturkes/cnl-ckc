% cdc2022-opioid-s37-23.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s37-23',ace_sha256('6864b385c7f20021b65b4e87d3c4ac723d4bc18d3e92ce7cd1e07b9f5840b6e2'),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If a patient receives a long-term-opioid-therapy then a possible-opioid-dosage-reduction can possibly provoke a substantial-anxiety in the patient.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s37-23',1,ref(4),[A,B,C]),'possible-opioid-dosage-reduction',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s37-23',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s37-23',1,box(1),[A,B,C]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s37-23',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s37-23',1,ref(5),[A,B,C]),'substantial-anxiety',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s37-23',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s37-23',1,ref(5),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s37-23',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s37-23',1,ref(6),[A,B,C]),possibility,possibly) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s37-23',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s37-23',1,ref(6),[A,B,C]),provoke) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-23',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s37-23',1,ref(6),[A,B,C]),1,'$guideline_id'(product,'cdc2022-opioid-s37-23',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-23',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s37-23',1,ref(6),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s37-23',1,ref(5),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s37-23',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s37-23',1,ref(6),[A,B,C]),in,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S2: An affected-long-term-opioid-patient-example includes an established-patient-example and includes a patient-transferring-from-another-clinician-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s37-23',2,ref(1),[]),'affected-long-term-opioid-patient-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s37-23',2,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s37-23',2,ref(2),[]),'established-patient-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s37-23',2,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s37-23',2,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s37-23',2,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s37-23',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s37-23',2,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s37-23',2,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s37-23',2,ref(4),[]),'patient-transferring-from-another-clinician-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s37-23',2,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s37-23',2,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s37-23',2,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s37-23',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s37-23',2,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s37-23',2,ref(4),[])).
