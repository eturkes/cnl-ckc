% cdc2022-opioid-s38-03.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s38-03',ace_sha256(ac68dd9faaaaf6b673cf1e382e55bae765cb34eb35f32aa17eee96ed2b00688f),ulex(sha256('4a882a1996ea910df0fbf91fea897c0805e67d48f9e1ce2fcdbbbd9250c193ae'))).
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
% S5: Every clinician may include a patient in a medication-first-decrease-decision.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s38-03',5,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',5,ref(2),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',5,ref(3),[A]),'medication-first-decrease-decision',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',5,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s38-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',5,ref(4),[A]),include) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',5,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',5,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s38-03',5,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s38-03',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-03',5,ref(4),[A]),in,'$guideline_id'(product,'cdc2022-opioid-s38-03',5,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
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
