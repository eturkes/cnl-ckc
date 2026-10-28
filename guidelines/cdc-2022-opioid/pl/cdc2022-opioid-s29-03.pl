% cdc2022-opioid-s29-03.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s29-03',ace_sha256('8bbd21f5f3dc0d18a1d5290b6a505179ec793d78fad633ba80116a8e45954089'),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: Every clinician should explicitly and realistically discuss an expected-opioid-benefit with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s29-03',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',1,ref(2),[A]),'expected-opioid-benefit',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s29-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',1,ref(4),[A]),manner,realistically) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s29-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',1,ref(4),[A]),manner,explicitly) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s29-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',1,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s29-03',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s29-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',1,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s29-03',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should explain a limited-long-term-opioid-pain-evidence with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s29-03',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',2,ref(2),[A]),'limited-long-term-opioid-pain-evidence',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',2,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s29-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',2,ref(4),[A]),explain) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s29-03',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s29-03',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',2,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s29-03',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should explain a limited-long-term-opioid-function-evidence with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s29-03',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',3,ref(2),[A]),'limited-long-term-opioid-function-evidence',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',3,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s29-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',3,ref(4),[A]),explain) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s29-03',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s29-03',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',3,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s29-03',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician should explain an unlikely-complete-pain-elimination with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s29-03',4,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',4,ref(2),[A]),'unlikely-complete-pain-elimination',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',4,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s29-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',4,ref(4),[A]),explain) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s29-03',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s29-03',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-03',4,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s29-03',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S5: There is no robust-long-term-opioid-pain-improvement-evidence.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s29-03',5,box(1),[]),-).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-03',5,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s29-03',5,ref(1),[]),'robust-long-term-opioid-pain-improvement-evidence',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-03',5,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s29-03',5,ref(1),[]),na,eq,1).
% S6: There is no robust-long-term-opioid-function-improvement-evidence.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s29-03',6,box(1),[]),-).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-03',6,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s29-03',6,ref(1),[]),'robust-long-term-opioid-function-improvement-evidence',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-03',6,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s29-03',6,ref(1),[]),na,eq,1).
% S7: It is possible that an opioid-therapy does not likely eliminate a pain completely.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s29-03',7,box(1),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-03',7,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s29-03',7,ref(1),[]),'opioid-therapy',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-03',7,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s29-03',7,ref(1),[]),na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-s29-03',7,box(1),[]),'$guideline_id'(context,'cdc2022-opioid-s29-03',7,box(2),[]),-).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-03',7,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-s29-03',7,ref(2),[]),pain,countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-03',7,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-s29-03',7,ref(2),[]),na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s29-03',7,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-s29-03',7,ref(3),[]),possibility,likely).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s29-03',7,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-s29-03',7,ref(3),[]),eliminate).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-03',7,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-s29-03',7,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s29-03',7,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-03',7,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-s29-03',7,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s29-03',7,ref(2),[])).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s29-03',7,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-s29-03',7,ref(3),[]),manner,completely).
