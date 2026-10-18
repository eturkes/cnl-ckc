% cdc2022-opioid-s38-02.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s38-02',ace_sha256('18fc6c7f742b7a7eb674aefdd387669fe6d56149ebb284382dd0f00f8f32e172'),ulex(sha256(d08a8bd3e80ef6e01301a079bb5496315f35233bf199aa89b2d2ccc93b858776)),vocabulary(sha256('9d7632637554f8cbfe72e7d34a9b9d14c29eb8593aebb1c128a870c5dcf77ead'))).
% S1: Every clinician should openly discuss an opioid-discontinuation-goal with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s38-02',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',1,ref(2),[A]),'opioid-discontinuation-goal',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s38-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',1,ref(4),[A]),manner,openly) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s38-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',1,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s38-02',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s38-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',1,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s38-02',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should openly discuss a benefit-risk-favorable-opioid-reduction-goal with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s38-02',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',2,ref(2),[A]),'benefit-risk-favorable-opioid-reduction-goal',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',2,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s38-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',2,ref(4),[A]),manner,openly) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s38-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',2,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s38-02',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s38-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',2,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s38-02',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should individualize a taper-goal for a patient with an individualized-benefit-risk-assessment.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s38-02',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',3,ref(2),[A]),'taper-goal',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',3,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',3,ref(4),[A]),'individualized-benefit-risk-assessment',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',3,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s38-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',3,ref(5),[A]),individualize) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',3,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',3,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s38-02',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s38-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',3,ref(5),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s38-02',3,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s38-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',3,ref(5),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s38-02',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician should consider a patient-circumstance during an individualized-benefit-risk-assessment.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s38-02',4,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',4,ref(2),[A]),'patient-circumstance',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',4,ref(3),[A]),'individualized-benefit-risk-assessment',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s38-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',4,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s38-02',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s38-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',4,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s38-02',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
