% cdc2022-opioid-s37-10.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
:- multifile(guideline_schema_version/1).
:- discontiguous(guideline_schema_version/1).
:- multifile(guideline_document/3).
:- discontiguous(guideline_document/3).
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
guideline_schema_version(1).
guideline_document('cdc2022-opioid-s37-10',ace_sha256(ad7d8f0934031a27e52fe01a7128265668a2071d1531209e95dc181011d2b918),ulex(sha256('06c1c26d6b6abe1436b8c94a276937ad072806062117417de2ab8086fdf9a3e9'))).
% S1: If a patient uses at least 2 respiratory-depressant-medications then every clinician may consider a respiratory-depression-risk-reducing-medication-taper for the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s37-10',1,box(1),[A,B,C,D]),may) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'respiratory-depressant-medication',countable), guideline_cardinality(actual,B,na,geq,2), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s37-10',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s37-10',1,ref(5),[A,B,C,D]),'respiratory-depression-risk-reducing-medication-taper',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'respiratory-depressant-medication',countable), guideline_cardinality(actual,B,na,geq,2), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s37-10',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s37-10',1,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'respiratory-depressant-medication',countable), guideline_cardinality(actual,B,na,geq,2), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s37-10',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s37-10',1,ref(6),[A,B,C,D]),consider) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'respiratory-depressant-medication',countable), guideline_cardinality(actual,B,na,geq,2), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-10',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s37-10',1,ref(6),[A,B,C,D]),1,D) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'respiratory-depressant-medication',countable), guideline_cardinality(actual,B,na,geq,2), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-10',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s37-10',1,ref(6),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-s37-10',1,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'respiratory-depressant-medication',countable), guideline_cardinality(actual,B,na,geq,2), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s37-10',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s37-10',1,ref(6),[A,B,C,D]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'respiratory-depressant-medication',countable), guideline_cardinality(actual,B,na,geq,2), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
