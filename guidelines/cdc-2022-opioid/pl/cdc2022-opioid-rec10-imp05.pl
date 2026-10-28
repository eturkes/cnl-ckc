% cdc2022-opioid-rec10-imp05.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec10-imp05',ace_sha256(d3c239a3dc57859e35575ca289dbe27ef298d7f5fa8ade939ce9e75e231f80a2),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If a patient has a toxicology-screening-result then every clinician should consider the toxicology-screening-result with an other-clinical-information for the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp05',1,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',1,ref(5),[A,B,C,D]),'other-clinical-information',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',1,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',1,ref(6),[A,B,C,D]),consider) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',1,ref(6),[A,B,C,D]),1,D) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',1,ref(6),[A,B,C,D]),2,B) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec10-imp05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',1,ref(6),[A,B,C,D]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec10-imp05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',1,ref(6),[A,B,C,D]),with,'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',1,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
% S2: Every toxicology-screening-result possibly can be useful.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp05',2,box(1),[A]),can) :- guideline_entity(actual,A,'toxicology-screening-result',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-rec10-imp05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',2,ref(3),[A]),possibility,possibly) :- guideline_entity(actual,A,'toxicology-screening-result',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_property('$guideline_id'(context,'cdc2022-opioid-rec10-imp05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',2,ref(2),[A]),useful,pos) :- guideline_entity(actual,A,'toxicology-screening-result',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',2,ref(3),[A]),be) :- guideline_entity(actual,A,'toxicology-screening-result',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'toxicology-screening-result',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',2,ref(2),[A])) :- guideline_entity(actual,A,'toxicology-screening-result',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: If a toxicology-screening has a potential-limitation and the potential-limitation is addressable then every clinician should consider the toxicology-screening.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp05',3,box(1),[A,B,C,D,E,F]),should) :- guideline_entity(actual,A,'toxicology-screening',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'potential-limitation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,be), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_property(actual,D,addressable,pos), guideline_qualifier(actual,E,applicability,addressable), guideline_entity(actual,F,clinician,countable), guideline_cardinality(actual,F,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp05',3,box(1),[A,B,C,D,E,F]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',3,ref(7),[A,B,C,D,E,F]),consider) :- guideline_entity(actual,A,'toxicology-screening',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'potential-limitation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,be), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_property(actual,D,addressable,pos), guideline_qualifier(actual,E,applicability,addressable), guideline_entity(actual,F,clinician,countable), guideline_cardinality(actual,F,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp05',3,box(1),[A,B,C,D,E,F]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',3,ref(7),[A,B,C,D,E,F]),1,F) :- guideline_entity(actual,A,'toxicology-screening',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'potential-limitation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,be), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_property(actual,D,addressable,pos), guideline_qualifier(actual,E,applicability,addressable), guideline_entity(actual,F,clinician,countable), guideline_cardinality(actual,F,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp05',3,box(1),[A,B,C,D,E,F]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',3,ref(7),[A,B,C,D,E,F]),2,A) :- guideline_entity(actual,A,'toxicology-screening',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'potential-limitation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,be), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_property(actual,D,addressable,pos), guideline_qualifier(actual,E,applicability,addressable), guideline_entity(actual,F,clinician,countable), guideline_cardinality(actual,F,na,eq,1).
