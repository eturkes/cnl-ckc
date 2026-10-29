% cdc2022-opioid-rec02-imp03.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec02-imp03',ace_sha256('73f0f1d745aaf4240f4c318de069f36e199a767d504b38061a7ef850fb7a6110'),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: It is possible that a physical-therapy helps a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp03',1,box(1),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp03',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',1,ref(1),[]),'physical-therapy',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp03',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',1,ref(1),[]),na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp03',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',1,ref(2),[]),patient,countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp03',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',1,ref(2),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp03',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',1,ref(3),[]),help).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp03',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',1,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',1,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp03',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',1,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',1,ref(2),[])).
% S2: If a patient has a limited-safe-public-exercise-space-access then it is possible that a physical-therapy particularly helps the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp03',2,box(1),[A,B,C]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'limited-safe-public-exercise-space-access',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp03',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',2,ref(4),[A,B,C]),'physical-therapy',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'limited-safe-public-exercise-space-access',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp03',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',2,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'limited-safe-public-exercise-space-access',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-rec02-imp03',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',2,ref(5),[A,B,C]),focus,particularly) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'limited-safe-public-exercise-space-access',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp03',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',2,ref(5),[A,B,C]),help) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'limited-safe-public-exercise-space-access',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp03',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',2,ref(5),[A,B,C]),1,'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',2,ref(4),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'limited-safe-public-exercise-space-access',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp03',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',2,ref(5),[A,B,C]),2,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'limited-safe-public-exercise-space-access',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S3: If a patient has a limited-public-recreation-facility-exercise-access then it is possible that a physical-therapy particularly helps the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp03',3,box(1),[A,B,C]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'limited-public-recreation-facility-exercise-access',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp03',3,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',3,ref(4),[A,B,C]),'physical-therapy',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'limited-public-recreation-facility-exercise-access',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp03',3,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',3,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'limited-public-recreation-facility-exercise-access',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-rec02-imp03',3,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',3,ref(5),[A,B,C]),focus,particularly) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'limited-public-recreation-facility-exercise-access',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp03',3,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',3,ref(5),[A,B,C]),help) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'limited-public-recreation-facility-exercise-access',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp03',3,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',3,ref(5),[A,B,C]),1,'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',3,ref(4),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'limited-public-recreation-facility-exercise-access',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp03',3,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',3,ref(5),[A,B,C]),2,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'limited-public-recreation-facility-exercise-access',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S4: If a patient has a pain that does not improve with a low-intensity-exercise then it is possible that a physical-therapy particularly helps the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp03',4,box(2),[A,B,C]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(D,E,'low-intensity-exercise',countable), guideline_operator(actual,D,-), guideline_cardinality(D,E,na,eq,1), guideline_event(D,F,improve), guideline_arg(D,F,1,B), guideline_pp(D,F,with,E), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp03',4,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',4,ref(6),[A,B,C]),'physical-therapy',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(D,E,'low-intensity-exercise',countable), guideline_operator(actual,D,-), guideline_cardinality(D,E,na,eq,1), guideline_event(D,F,improve), guideline_arg(D,F,1,B), guideline_pp(D,F,with,E), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp03',4,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',4,ref(6),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(D,E,'low-intensity-exercise',countable), guideline_operator(actual,D,-), guideline_cardinality(D,E,na,eq,1), guideline_event(D,F,improve), guideline_arg(D,F,1,B), guideline_pp(D,F,with,E), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-rec02-imp03',4,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',4,ref(7),[A,B,C]),focus,particularly) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(D,E,'low-intensity-exercise',countable), guideline_operator(actual,D,-), guideline_cardinality(D,E,na,eq,1), guideline_event(D,F,improve), guideline_arg(D,F,1,B), guideline_pp(D,F,with,E), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp03',4,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',4,ref(7),[A,B,C]),help) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(D,E,'low-intensity-exercise',countable), guideline_operator(actual,D,-), guideline_cardinality(D,E,na,eq,1), guideline_event(D,F,improve), guideline_arg(D,F,1,B), guideline_pp(D,F,with,E), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp03',4,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',4,ref(7),[A,B,C]),1,'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',4,ref(6),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(D,E,'low-intensity-exercise',countable), guideline_operator(actual,D,-), guideline_cardinality(D,E,na,eq,1), guideline_event(D,F,improve), guideline_arg(D,F,1,B), guideline_pp(D,F,with,E), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp03',4,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp03',4,ref(7),[A,B,C]),2,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,pain,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(D,E,'low-intensity-exercise',countable), guideline_operator(actual,D,-), guideline_cardinality(D,E,na,eq,1), guideline_event(D,F,improve), guideline_arg(D,F,1,B), guideline_pp(D,F,with,E), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
