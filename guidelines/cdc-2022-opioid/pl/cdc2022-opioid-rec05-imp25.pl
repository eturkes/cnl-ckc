% cdc2022-opioid-rec05-imp25.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec05-imp25',ace_sha256(b5da131c8105ad8ce074e0624b8e5f00bdd6f5fa4bb1a419d72346d4fade2408),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If there is an opioid-policy-setter then it is recommended that the opioid-policy-setter does not set a rigid-dose-standard with a CDC-clinical-practice-guideline.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(2),[A]),-) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(2),[A]),'rigid-dose-standard',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(3),[A]),'CDC-clinical-practice-guideline',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(4),[A]),set) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(3),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: If there is an opioid-policy-setter then it is recommended that the opioid-policy-setter does not set a rigid-duration-standard with a CDC-clinical-practice-guideline.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(2),[A]),-) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(2),[A]),'rigid-duration-standard',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(3),[A]),'CDC-clinical-practice-guideline',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(4),[A]),set) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(3),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: If there is an opioid-policy-setter then it is recommended that the opioid-policy-setter does not set a dose-or-duration-performance-incentive with a CDC-clinical-practice-guideline.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),-) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(2),[A]),'dose-or-duration-performance-incentive',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(3),[A]),'CDC-clinical-practice-guideline',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(4),[A]),set) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(3),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every opioid-policy-setter should ensure a cautionary-threshold-policy that does not cause a rapid-opioid-taper.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(2),[A]),'cautionary-threshold-policy',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),-) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(3),[A]),'rapid-opioid-taper',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(4),[A]),cause) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(4),[A]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(3),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(5),[A]),ensure) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(5),[A]),1,A) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every opioid-policy-setter should ensure a cautionary-threshold-policy that does not cause an abrupt-opioid-discontinuation.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(2),[A]),'cautionary-threshold-policy',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(2),[A]),-) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(3),[A]),'abrupt-opioid-discontinuation',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(4),[A]),cause) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(4),[A]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(3),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(5),[A]),ensure) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(5),[A]),1,A) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
% S6: If there is an opioid-policy-setter then it is recommended that the opioid-policy-setter does not penalize a clinician for a new-chronic-pain-prescribed-opioid-patient-acceptance.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',6,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',6,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',6,box(2),[A]),-) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',6,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',6,ref(2),[A]),clinician,countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',6,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',6,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',6,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',6,ref(3),[A]),'new-chronic-pain-prescribed-opioid-patient-acceptance',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',6,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',6,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',6,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',6,ref(4),[A]),penalize) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',6,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',6,ref(4),[A]),1,A) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',6,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',6,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',6,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',6,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',6,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',6,ref(3),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
% S7: If there is an opioid-policy-setter then it is recommended that the opioid-policy-setter does not penalize a clinician that does not rapidly taper a long-term-opioid-medication for a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(2),[A]),-) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(2),[A]),clinician,countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(2),[A]),'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(3),[A]),-) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(3),[A]),'long-term-opioid-medication',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(4),[A]),patient,countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(5),[A]),manner,rapidly) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(5),[A]),taper) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(5),[A]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(3),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(5),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(4),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(6),[A]),penalize) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(6),[A]),1,A) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(6),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
% S8: A new-chronic-pain-prescribed-opioid-patient-acceptance-example is a high-dosage-patient-acceptance-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',8,ref(1),[]),'new-chronic-pain-prescribed-opioid-patient-acceptance-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',8,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',8,ref(2),[]),'high-dosage-patient-acceptance-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',8,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',8,ref(3),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',8,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',8,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',8,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',8,ref(2),[])).
% S9: Every payer is an opioid-policy-setter.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',9,ref(2),[A]),'opioid-policy-setter',countable) :- guideline_entity(actual,A,payer,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',9,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,payer,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',9,ref(3),[A]),be) :- guideline_entity(actual,A,payer,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',9,ref(3),[A]),1,A) :- guideline_entity(actual,A,payer,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',9,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',9,ref(2),[A])) :- guideline_entity(actual,A,payer,countable), guideline_cardinality(actual,A,na,eq,1).
% S10: Every health-system is an opioid-policy-setter.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',10,ref(2),[A]),'opioid-policy-setter',countable) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',10,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',10,ref(3),[A]),be) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',10,ref(3),[A]),1,A) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',10,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',10,ref(2),[A])) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
% S11: Every state-medical-board is an opioid-policy-setter.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',11,ref(2),[A]),'opioid-policy-setter',countable) :- guideline_entity(actual,A,'state-medical-board',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',11,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'state-medical-board',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',11,ref(3),[A]),be) :- guideline_entity(actual,A,'state-medical-board',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',11,ref(3),[A]),1,A) :- guideline_entity(actual,A,'state-medical-board',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',11,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',11,ref(2),[A])) :- guideline_entity(actual,A,'state-medical-board',countable), guideline_cardinality(actual,A,na,eq,1).
