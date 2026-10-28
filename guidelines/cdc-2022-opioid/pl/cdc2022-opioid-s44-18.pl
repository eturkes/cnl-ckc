% cdc2022-opioid-s44-18.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s44-18',ace_sha256('47b58a839514a918a6d8ce3869bd95b8c1f8d7deb12f1373e85bab2a75929e21'),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: Every clinician should ask a patient about a common-opioid-adverse-effect.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s44-18',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s44-18',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',1,ref(2),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s44-18',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s44-18',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',1,ref(3),[A]),'common-opioid-adverse-effect',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s44-18',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s44-18',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',1,ref(4),[A]),ask) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s44-18',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s44-18',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s44-18',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s44-18',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',1,ref(4),[A]),about,'$guideline_id'(product,'cdc2022-opioid-s44-18',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: If there is a clinician then it is recommended that the clinician asks a patient about a possible-early-overdose-warning-sign and assesses the possible-early-overdose-warning-sign for the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s44-18',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s44-18',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',2,ref(2),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s44-18',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s44-18',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',2,ref(3),[A]),'possible-early-overdose-warning-sign',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s44-18',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s44-18',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',2,ref(4),[A]),ask) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s44-18',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s44-18',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s44-18',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s44-18',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',2,ref(4),[A]),about,'$guideline_id'(product,'cdc2022-opioid-s44-18',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s44-18',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',2,ref(5),[A]),assess) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s44-18',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',2,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s44-18',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',2,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s44-18',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s44-18',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',2,ref(5),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s44-18',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: If there is a clinician then it is recommended that the clinician asks a patient about a possible-early-opioid-use-disorder-warning-sign and assesses the possible-early-opioid-use-disorder-warning-sign for the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s44-18',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s44-18',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',3,ref(2),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s44-18',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s44-18',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',3,ref(3),[A]),'possible-early-opioid-use-disorder-warning-sign',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s44-18',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s44-18',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',3,ref(4),[A]),ask) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s44-18',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s44-18',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s44-18',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s44-18',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',3,ref(4),[A]),about,'$guideline_id'(product,'cdc2022-opioid-s44-18',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s44-18',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',3,ref(5),[A]),assess) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s44-18',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',3,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s44-18',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',3,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s44-18',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s44-18',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s44-18',3,ref(5),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s44-18',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: A common-opioid-adverse-effect-example includes a constipation-example and includes a drowsiness-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',4,ref(1),[]),'common-opioid-adverse-effect-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',4,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',4,ref(2),[]),'constipation-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',4,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',4,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',4,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s44-18',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',4,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s44-18',4,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',4,ref(4),[]),'drowsiness-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',4,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',4,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',4,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s44-18',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',4,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s44-18',4,ref(4),[])).
% S5: A possible-early-overdose-warning-sign-example includes a sedation-example and includes a slurred-speech-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',5,ref(1),[]),'possible-early-overdose-warning-sign-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',5,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',5,ref(2),[]),'sedation-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',5,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',5,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',5,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s44-18',5,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',5,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s44-18',5,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',5,ref(4),[]),'slurred-speech-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',5,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',5,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',5,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s44-18',5,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',5,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s44-18',5,ref(4),[])).
% S6: A possible-early-opioid-use-disorder-warning-sign-example includes a craving-example and includes a greater-than-prescribed-quantity-or-frequency-wish-example and includes an opioid-use-control-difficulty-example and includes an opioid-use-related-work-social-or-family-problem-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(1),[]),'possible-early-opioid-use-disorder-warning-sign-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(2),[]),'craving-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(4),[]),'greater-than-prescribed-quantity-or-frequency-wish-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(4),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(6),[]),'opioid-use-control-difficulty-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(6),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(7),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(7),[]),1,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(7),[]),2,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(6),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(8),[]),'opioid-use-related-work-social-or-family-problem-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(8),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(9),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(9),[]),1,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(9),[]),2,'$guideline_id'(product,'cdc2022-opioid-s44-18',6,ref(8),[])).
