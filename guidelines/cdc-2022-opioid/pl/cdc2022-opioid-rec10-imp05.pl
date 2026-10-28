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
guideline_document('cdc2022-opioid-rec10-imp05',ace_sha256('5c8dbd4058b751f3f6150d713ed3e499db7a02586fec71eb60a4a10ec7a3186d'),ulex(sha256(de64e41d6ddfdcb89810d9fc156aa0ac54562519e3bd4b94b8f7324f773ceb91)),vocabulary(sha256('06621a3dca592ca4385262a4a36dd8184670cdaef31827cd8b224c8c870c2f7d'))).
% S1: If a patient has a toxicology-screening-result then every clinician should consider the toxicology-screening-result with an other-clinical-information for the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp05',1,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',1,ref(5),[A,B,C,D]),'other-clinical-information',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',1,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',1,ref(6),[A,B,C,D]),consider) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',1,ref(6),[A,B,C,D]),1,D) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',1,ref(6),[A,B,C,D]),2,B) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec10-imp05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',1,ref(6),[A,B,C,D]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec10-imp05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',1,ref(6),[A,B,C,D]),with,'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',1,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'toxicology-screening-result',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
% S2: If a potential-toxicology-screening-limitation is addressable then every clinician should consider a toxicology-screening.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp05',2,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,'potential-toxicology-screening-limitation',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,addressable,pos), guideline_qualifier(actual,C,applicability,addressable), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp05',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',2,ref(5),[A,B,C,D]),'toxicology-screening',countable) :- guideline_entity(actual,A,'potential-toxicology-screening-limitation',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,addressable,pos), guideline_qualifier(actual,C,applicability,addressable), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp05',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',2,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,'potential-toxicology-screening-limitation',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,addressable,pos), guideline_qualifier(actual,C,applicability,addressable), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp05',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',2,ref(6),[A,B,C,D]),consider) :- guideline_entity(actual,A,'potential-toxicology-screening-limitation',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,addressable,pos), guideline_qualifier(actual,C,applicability,addressable), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp05',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',2,ref(6),[A,B,C,D]),1,D) :- guideline_entity(actual,A,'potential-toxicology-screening-limitation',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,addressable,pos), guideline_qualifier(actual,C,applicability,addressable), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp05',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',2,ref(6),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp05',2,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,'potential-toxicology-screening-limitation',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,addressable,pos), guideline_qualifier(actual,C,applicability,addressable), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
