% cdc2022-opioid-s24-26.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s24-26',ace_sha256(fe74c5b90c356fc2d9e3fe6c5253b16b64beadbe578b7ca8308917b9a8d479a5),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If a patient has a headache then it is possible that an opioid-initiation-benefit does not likely outweigh an opioid-initiation-risk.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s24-26',1,box(1),[A,B,C]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,headache,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s24-26',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s24-26',1,ref(4),[A,B,C]),'opioid-initiation-benefit',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,headache,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s24-26',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s24-26',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,headache,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-s24-26',1,box(1),[A,B,C]),'$guideline_id'(context,'cdc2022-opioid-s24-26',1,box(2),[A,B,C]),-) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,headache,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s24-26',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s24-26',1,ref(5),[A,B,C]),'opioid-initiation-risk',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,headache,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s24-26',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s24-26',1,ref(5),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,headache,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s24-26',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s24-26',1,ref(6),[A,B,C]),possibility,likely) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,headache,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s24-26',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s24-26',1,ref(6),[A,B,C]),outweigh) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,headache,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s24-26',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s24-26',1,ref(6),[A,B,C]),1,'$guideline_id'(product,'cdc2022-opioid-s24-26',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,headache,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s24-26',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s24-26',1,ref(6),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s24-26',1,ref(5),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,headache,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S2: If a patient has a fibromyalgia then it is possible that an opioid-initiation-benefit does not likely outweigh an opioid-initiation-risk.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s24-26',2,box(1),[A,B,C]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s24-26',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s24-26',2,ref(4),[A,B,C]),'opioid-initiation-benefit',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s24-26',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s24-26',2,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-s24-26',2,box(1),[A,B,C]),'$guideline_id'(context,'cdc2022-opioid-s24-26',2,box(2),[A,B,C]),-) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s24-26',2,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s24-26',2,ref(5),[A,B,C]),'opioid-initiation-risk',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s24-26',2,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s24-26',2,ref(5),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s24-26',2,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s24-26',2,ref(6),[A,B,C]),possibility,likely) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s24-26',2,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s24-26',2,ref(6),[A,B,C]),outweigh) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s24-26',2,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s24-26',2,ref(6),[A,B,C]),1,'$guideline_id'(product,'cdc2022-opioid-s24-26',2,ref(4),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s24-26',2,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s24-26',2,ref(6),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s24-26',2,ref(5),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,fibromyalgia,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
