% cdc2022-opioid-s23-12.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s23-12',ace_sha256('9c46c03a9540a6a1b0deaabbb9ec8316d3131d40143275c3c109f6eb5b1a16b2'),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: Every clinician should discuss a higher-opioid-dosage-opioid-use-disorder-risk with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-12',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',1,ref(2),[A]),'higher-opioid-dosage-opioid-use-disorder-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',1,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-12',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',1,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-12',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should discuss a higher-opioid-dosage-respiratory-depression-risk with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-12',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',2,ref(2),[A]),'higher-opioid-dosage-respiratory-depression-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',2,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',2,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-12',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',2,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-12',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should discuss a higher-opioid-dosage-death-risk with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-12',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',3,ref(2),[A]),'higher-opioid-dosage-death-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',3,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',3,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-12',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',3,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-12',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician should discuss a prescribed-opioid-amount-and-frequency-adherence with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-12',4,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-12',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',4,ref(2),[A]),'prescribed-opioid-amount-and-frequency-adherence',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-12',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-12',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',4,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-12',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-12',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',4,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-12',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-12',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-12',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-12',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',4,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-12',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every higher-opioid-dosage can increase an opioid-use-disorder-risk and can increase a respiratory-depression-risk and can increase a death-risk.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-12',5,box(1),[A]),can) :- guideline_entity(actual,A,'higher-opioid-dosage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-12',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',5,ref(2),[A]),'opioid-use-disorder-risk',countable) :- guideline_entity(actual,A,'higher-opioid-dosage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-12',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'higher-opioid-dosage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-12',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',5,ref(3),[A]),increase) :- guideline_entity(actual,A,'higher-opioid-dosage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-12',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',5,ref(3),[A]),1,A) :- guideline_entity(actual,A,'higher-opioid-dosage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-12',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',5,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-12',5,ref(2),[A])) :- guideline_entity(actual,A,'higher-opioid-dosage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-12',5,box(2),[A]),can) :- guideline_entity(actual,A,'higher-opioid-dosage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-12',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',5,ref(4),[A]),'respiratory-depression-risk',countable) :- guideline_entity(actual,A,'higher-opioid-dosage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-12',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',5,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'higher-opioid-dosage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-12',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',5,ref(5),[A]),increase) :- guideline_entity(actual,A,'higher-opioid-dosage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-12',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',5,ref(5),[A]),1,A) :- guideline_entity(actual,A,'higher-opioid-dosage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-12',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',5,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-12',5,ref(4),[A])) :- guideline_entity(actual,A,'higher-opioid-dosage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-12',5,box(3),[A]),can) :- guideline_entity(actual,A,'higher-opioid-dosage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-12',5,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',5,ref(6),[A]),'death-risk',countable) :- guideline_entity(actual,A,'higher-opioid-dosage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-12',5,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',5,ref(6),[A]),na,eq,1) :- guideline_entity(actual,A,'higher-opioid-dosage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-12',5,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',5,ref(7),[A]),increase) :- guideline_entity(actual,A,'higher-opioid-dosage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-12',5,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',5,ref(7),[A]),1,A) :- guideline_entity(actual,A,'higher-opioid-dosage',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-12',5,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-12',5,ref(7),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-12',5,ref(6),[A])) :- guideline_entity(actual,A,'higher-opioid-dosage',countable), guideline_cardinality(actual,A,na,eq,1).
