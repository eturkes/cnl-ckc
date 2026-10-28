% cdc2022-opioid-rec09-imp10.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec09-imp10',ace_sha256('0fb11c9431b1f9dcdb7d89831609b5b8a12e2fda45b5a4354bafb838a14e56ee'),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If a clinician reviews a state-PDMP-record for a patient then the clinician should discuss the state-PDMP-record with the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec09-imp10',1,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'state-PDMP-record',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,review), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), \+ guideline_exclusion('cdc-2022-opioid').
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec09-imp10',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp10',1,ref(5),[A,B,C,D]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'state-PDMP-record',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,review), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec09-imp10',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp10',1,ref(5),[A,B,C,D]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'state-PDMP-record',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,review), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec09-imp10',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp10',1,ref(5),[A,B,C,D]),2,B) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'state-PDMP-record',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,review), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec09-imp10',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp10',1,ref(5),[A,B,C,D]),with,C) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'state-PDMP-record',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,review), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
% S2: If a clinician identifies an additional-prescription in a state-PDMP-record for a patient then the clinician should confirm an additional-prescription-awareness with the patient about the additional-prescription.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec09-imp10',2,box(1),[A,B,C,D,E]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'additional-prescription',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'state-PDMP-record',countable), guideline_cardinality(actual,C,na,eq,1), guideline_entity(actual,D,patient,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,identify), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,B), guideline_pp(actual,E,for,D), guideline_pp(actual,E,in,C), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec09-imp10',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp10',2,ref(6),[A,B,C,D,E]),'additional-prescription-awareness',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'additional-prescription',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'state-PDMP-record',countable), guideline_cardinality(actual,C,na,eq,1), guideline_entity(actual,D,patient,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,identify), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,B), guideline_pp(actual,E,for,D), guideline_pp(actual,E,in,C).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec09-imp10',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp10',2,ref(6),[A,B,C,D,E]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'additional-prescription',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'state-PDMP-record',countable), guideline_cardinality(actual,C,na,eq,1), guideline_entity(actual,D,patient,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,identify), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,B), guideline_pp(actual,E,for,D), guideline_pp(actual,E,in,C).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec09-imp10',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp10',2,ref(7),[A,B,C,D,E]),confirm) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'additional-prescription',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'state-PDMP-record',countable), guideline_cardinality(actual,C,na,eq,1), guideline_entity(actual,D,patient,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,identify), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,B), guideline_pp(actual,E,for,D), guideline_pp(actual,E,in,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec09-imp10',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp10',2,ref(7),[A,B,C,D,E]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'additional-prescription',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'state-PDMP-record',countable), guideline_cardinality(actual,C,na,eq,1), guideline_entity(actual,D,patient,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,identify), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,B), guideline_pp(actual,E,for,D), guideline_pp(actual,E,in,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec09-imp10',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp10',2,ref(7),[A,B,C,D,E]),2,'$guideline_id'(product,'cdc2022-opioid-rec09-imp10',2,ref(6),[A,B,C,D,E])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'additional-prescription',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'state-PDMP-record',countable), guideline_cardinality(actual,C,na,eq,1), guideline_entity(actual,D,patient,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,identify), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,B), guideline_pp(actual,E,for,D), guideline_pp(actual,E,in,C).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec09-imp10',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp10',2,ref(7),[A,B,C,D,E]),about,B) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'additional-prescription',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'state-PDMP-record',countable), guideline_cardinality(actual,C,na,eq,1), guideline_entity(actual,D,patient,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,identify), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,B), guideline_pp(actual,E,for,D), guideline_pp(actual,E,in,C).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec09-imp10',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp10',2,ref(7),[A,B,C,D,E]),with,D) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'additional-prescription',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'state-PDMP-record',countable), guideline_cardinality(actual,C,na,eq,1), guideline_entity(actual,D,patient,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,identify), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,B), guideline_pp(actual,E,for,D), guideline_pp(actual,E,in,C).
