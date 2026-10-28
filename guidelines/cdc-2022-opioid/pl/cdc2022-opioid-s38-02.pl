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
:- multifile(guideline_exclusion/1).
:- discontiguous(guideline_exclusion/1).
:- multifile(guideline_supersession/4).
:- discontiguous(guideline_supersession/4).
:- multifile(guideline_evaluation_date/1).
:- discontiguous(guideline_evaluation_date/1).
guideline_schema_version(1).
guideline_document('cdc2022-opioid-s38-02',ace_sha256('1cc1e424be97591f4118735270f7d91b102b5b3ee4224fb434571b94707b0b3a'),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If an opioid-taper for a patient has an opioid-discontinuation-goal then every clinician should openly discuss the opioid-discontinuation-goal with the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s38-02',1,box(1),[A,B,C,D,E]),should) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'opioid-discontinuation-goal',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_pp(actual,D,for,B), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s38-02',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s38-02',1,ref(6),[A,B,C,D,E]),manner,openly) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'opioid-discontinuation-goal',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_pp(actual,D,for,B), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s38-02',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s38-02',1,ref(6),[A,B,C,D,E]),discuss) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'opioid-discontinuation-goal',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_pp(actual,D,for,B), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-02',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s38-02',1,ref(6),[A,B,C,D,E]),1,E) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'opioid-discontinuation-goal',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_pp(actual,D,for,B), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-02',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s38-02',1,ref(6),[A,B,C,D,E]),2,C) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'opioid-discontinuation-goal',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_pp(actual,D,for,B), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s38-02',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s38-02',1,ref(6),[A,B,C,D,E]),with,B) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'opioid-discontinuation-goal',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_pp(actual,D,for,B), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
% S2: If an opioid-taper for a patient has a benefit-risk-favorable-opioid-reduction-goal then every clinician should openly discuss the benefit-risk-favorable-opioid-reduction-goal with the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s38-02',2,box(1),[A,B,C,D,E]),should) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'benefit-risk-favorable-opioid-reduction-goal',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_pp(actual,D,for,B), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s38-02',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s38-02',2,ref(6),[A,B,C,D,E]),manner,openly) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'benefit-risk-favorable-opioid-reduction-goal',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_pp(actual,D,for,B), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s38-02',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s38-02',2,ref(6),[A,B,C,D,E]),discuss) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'benefit-risk-favorable-opioid-reduction-goal',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_pp(actual,D,for,B), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-02',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s38-02',2,ref(6),[A,B,C,D,E]),1,E) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'benefit-risk-favorable-opioid-reduction-goal',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_pp(actual,D,for,B), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-02',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s38-02',2,ref(6),[A,B,C,D,E]),2,C) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'benefit-risk-favorable-opioid-reduction-goal',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_pp(actual,D,for,B), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s38-02',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s38-02',2,ref(6),[A,B,C,D,E]),with,B) :- guideline_entity(actual,A,'opioid-taper',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'benefit-risk-favorable-opioid-reduction-goal',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_pp(actual,D,for,B), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
% S3: Every clinician should individualize a taper-goal for a patient with an individualized-benefit-risk-assessment.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s38-02',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
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
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s38-02',4,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',4,ref(2),[A]),'patient-circumstance',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s38-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',4,ref(3),[A]),'individualized-benefit-risk-assessment',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s38-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s38-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',4,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s38-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s38-02',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s38-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s38-02',4,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s38-02',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
