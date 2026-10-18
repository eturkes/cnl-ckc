% cdc2022-opioid-s38-03.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_schema_version(1).
guideline_document('cdc2022-opioid-s38-03',ace_sha256('523ad0a42a1058354a28126496ef0eae03ec44f4fb734a11f4605a8a71d90105'),ulex(sha256(ebe2d3db8559958f9ed44ec7a76cb2fb6ea75017f9f9437c3ea03b07092b0e75)),vocabulary(sha256('9d7632637554f8cbfe72e7d34a9b9d14c29eb8593aebb1c128a870c5dcf77ead'))).
% S1: Every clinician may discuss a continued-opioid-benefit-perception with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s38-03',1,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',1,ref(2),[A]),'continued-opioid-benefit-perception',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s38-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',1,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s38-03',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s38-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',1,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s38-03',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician may discuss a continued-opioid-risk-perception with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s38-03',2,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',2,ref(2),[A]),'continued-opioid-risk-perception',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',2,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s38-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',2,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s38-03',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s38-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',2,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s38-03',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician may discuss a continued-opioid-adverse-effect-perception with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s38-03',3,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',3,ref(2),[A]),'continued-opioid-adverse-effect-perception',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',3,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s38-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',3,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s38-03',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s38-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',3,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s38-03',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician may include a patient-concern in a taper-plan.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s38-03',4,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',4,ref(2),[A]),'patient-concern',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',4,ref(3),[A]),'taper-plan',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s38-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',4,ref(4),[A]),include) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s38-03',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s38-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',4,ref(4),[A]),in,'$guideline_id'(product,'cdc2022-opioid-s38-03',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S5: If a patient receives more than 1 prescribed-opioid then every clinician may include the patient in a medication-first-decrease-decision.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s38-03',5,box(1),[A,B,C,D]),may) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prescribed-opioid',countable), guideline_cardinality(actual,B,na,E,F), guideline_satisfies(E,F,greater,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-03',5,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s38-03',5,ref(5),[A,B,C,D]),'medication-first-decrease-decision',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prescribed-opioid',countable), guideline_cardinality(actual,B,na,E,F), guideline_satisfies(E,F,greater,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-03',5,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s38-03',5,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prescribed-opioid',countable), guideline_cardinality(actual,B,na,E,F), guideline_satisfies(E,F,greater,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s38-03',5,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s38-03',5,ref(6),[A,B,C,D]),include) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prescribed-opioid',countable), guideline_cardinality(actual,B,na,E,F), guideline_satisfies(E,F,greater,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-03',5,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s38-03',5,ref(6),[A,B,C,D]),1,D) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prescribed-opioid',countable), guideline_cardinality(actual,B,na,E,F), guideline_satisfies(E,F,greater,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-03',5,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s38-03',5,ref(6),[A,B,C,D]),2,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prescribed-opioid',countable), guideline_cardinality(actual,B,na,E,F), guideline_satisfies(E,F,greater,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s38-03',5,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s38-03',5,ref(6),[A,B,C,D]),in,'$guideline_id'(product,'cdc2022-opioid-s38-03',5,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prescribed-opioid',countable), guideline_cardinality(actual,B,na,E,F), guideline_satisfies(E,F,greater,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
% S6: Every clinician may include a patient in a taper-speed-decision.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s38-03',6,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-03',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',6,ref(2),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-03',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',6,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-03',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',6,ref(3),[A]),'taper-speed-decision',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-03',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',6,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s38-03',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',6,ref(4),[A]),include) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-03',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',6,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-03',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',6,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s38-03',6,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s38-03',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',6,ref(4),[A]),in,'$guideline_id'(product,'cdc2022-opioid-s38-03',6,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
