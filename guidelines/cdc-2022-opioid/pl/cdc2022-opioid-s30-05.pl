% cdc2022-opioid-s30-05.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s30-05',ace_sha256(d74f3e85ba4cacc29d2b68be97029718cdee3c81b9704e6d30b5ca2e4fa051af),ulex(sha256('64c086862c2f48d491a9fde0ba9f0fcf090e25e84456b9749332ec3ace211ce4')),vocabulary(sha256('405832ed3f275f091f60ded2e828af610266110b6c9a29536ca799d29ca43d72'))).
% S1: If a medication is needed then every clinician should ideally combine the medication with a nonpharmacologic-therapy.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s30-05',1,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,medication,countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,needed,pos), guideline_qualifier(actual,C,applicability,needed), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-05',1,ref(5),[A,B,C,D]),'nonpharmacologic-therapy',countable) :- guideline_entity(actual,A,medication,countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,needed,pos), guideline_qualifier(actual,C,applicability,needed), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-05',1,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,medication,countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,needed,pos), guideline_qualifier(actual,C,applicability,needed), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s30-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-05',1,ref(6),[A,B,C,D]),ideality,ideally) :- guideline_entity(actual,A,medication,countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,needed,pos), guideline_qualifier(actual,C,applicability,needed), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s30-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-05',1,ref(6),[A,B,C,D]),combine) :- guideline_entity(actual,A,medication,countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,needed,pos), guideline_qualifier(actual,C,applicability,needed), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-05',1,ref(6),[A,B,C,D]),1,D) :- guideline_entity(actual,A,medication,countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,needed,pos), guideline_qualifier(actual,C,applicability,needed), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-05',1,ref(6),[A,B,C,D]),2,A) :- guideline_entity(actual,A,medication,countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,needed,pos), guideline_qualifier(actual,C,applicability,needed), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s30-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-05',1,ref(6),[A,B,C,D]),with,'$guideline_id'(product,'cdc2022-opioid-s30-05',1,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,medication,countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,needed,pos), guideline_qualifier(actual,C,applicability,needed), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
% S2: A medication-nonpharmacologic-therapy-combination can improve a pain and can improve a patient-function.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(1),[]),'medication-nonpharmacologic-therapy-combination',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(1),[]),na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s30-05',2,box(1),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-05',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(2),[]),pain,countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-05',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(2),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s30-05',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(3),[]),improve).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-05',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-05',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(2),[])).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s30-05',2,box(2),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-05',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(4),[]),'patient-function',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-05',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(4),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s30-05',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(5),[]),improve).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-05',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-05',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(4),[])).
