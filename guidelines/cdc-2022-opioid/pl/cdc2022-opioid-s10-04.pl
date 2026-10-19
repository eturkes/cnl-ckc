% cdc2022-opioid-s10-04.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_schema_version(1).
guideline_document('cdc2022-opioid-s10-04',ace_sha256('04306393f635798e351c586fae19d3dfd986846e124228370846069aca9b56b2'),ulex(sha256('3210e31a7a1a232faae4aed00d06481f6bef71ebd89b6c884183bc530be9986e')),vocabulary(sha256(b5cff67a2f9de9bee846e9e7650affaf95276ff3a26fc00befd8ceb076b0f4ac))).
% S1: If a patient has an age and the age includes less than 18 years then it is false that a recommendation applies to the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s10-04',1,box(1),[A,B,C,D,E]),-) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,age,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,include), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_cardinality(actual,D,na,F,G), guideline_unit(actual,D,H), guideline_satisfies(F,G,H,less,18,year).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s10-04',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s10-04',1,ref(6),[A,B,C,D,E]),recommendation,countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,age,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,include), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_cardinality(actual,D,na,F,G), guideline_unit(actual,D,H), guideline_satisfies(F,G,H,less,18,year).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s10-04',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s10-04',1,ref(6),[A,B,C,D,E]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,age,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,include), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_cardinality(actual,D,na,F,G), guideline_unit(actual,D,H), guideline_satisfies(F,G,H,less,18,year).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s10-04',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s10-04',1,ref(7),[A,B,C,D,E]),apply) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,age,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,include), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_cardinality(actual,D,na,F,G), guideline_unit(actual,D,H), guideline_satisfies(F,G,H,less,18,year).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s10-04',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s10-04',1,ref(7),[A,B,C,D,E]),1,'$guideline_id'(product,'cdc2022-opioid-s10-04',1,ref(6),[A,B,C,D,E])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,age,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,include), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_cardinality(actual,D,na,F,G), guideline_unit(actual,D,H), guideline_satisfies(F,G,H,less,18,year).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s10-04',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s10-04',1,ref(7),[A,B,C,D,E]),to,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,age,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,include), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_cardinality(actual,D,na,F,G), guideline_unit(actual,D,H), guideline_satisfies(F,G,H,less,18,year).
guideline_exclusion('cdc-2022-opioid') :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,age,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,D,include), guideline_arg(actual,D,1,B), guideline_arg(actual,D,2,E), guideline_cardinality(actual,E,na,F,G), guideline_unit(actual,E,H), guideline_satisfies(F,G,H,less,18,year).
