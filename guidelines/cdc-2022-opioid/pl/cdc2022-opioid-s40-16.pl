% cdc2022-opioid-s40-16.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s40-16',ace_sha256('6ffffa4a6ecbfda7b1cfb63df883ec3f46ecfe287aacc3bf47c4e6163e53bee2'),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: Every buprenorphine-withdrawal-suppression-and-opioid-use-disorder-stabilization-duration exceeds a buprenorphine-analgesia-duration.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s40-16',1,ref(2),[A]),'buprenorphine-analgesia-duration',countable) :- guideline_entity(actual,A,'buprenorphine-withdrawal-suppression-and-opioid-use-disorder-stabilization-duration',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s40-16',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'buprenorphine-withdrawal-suppression-and-opioid-use-disorder-stabilization-duration',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s40-16',1,ref(3),[A]),exceed) :- guideline_entity(actual,A,'buprenorphine-withdrawal-suppression-and-opioid-use-disorder-stabilization-duration',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s40-16',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'buprenorphine-withdrawal-suppression-and-opioid-use-disorder-stabilization-duration',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s40-16',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s40-16',1,ref(2),[A])) :- guideline_entity(actual,A,'buprenorphine-withdrawal-suppression-and-opioid-use-disorder-stabilization-duration',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every buprenorphine-pain-dosing can typically use a multiple-times-daily-dosing instead-of a once-a-day-dosing.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s40-16',2,box(1),[A]),can) :- guideline_entity(actual,A,'buprenorphine-pain-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s40-16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s40-16',2,ref(2),[A]),'multiple-times-daily-dosing',countable) :- guideline_entity(actual,A,'buprenorphine-pain-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s40-16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s40-16',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'buprenorphine-pain-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s40-16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s40-16',2,ref(3),[A]),'once-a-day-dosing',countable) :- guideline_entity(actual,A,'buprenorphine-pain-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s40-16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s40-16',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'buprenorphine-pain-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s40-16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s40-16',2,ref(4),[A]),usuality,typically) :- guideline_entity(actual,A,'buprenorphine-pain-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s40-16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s40-16',2,ref(4),[A]),use) :- guideline_entity(actual,A,'buprenorphine-pain-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s40-16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s40-16',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,'buprenorphine-pain-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s40-16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s40-16',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s40-16',2,ref(2),[A])) :- guideline_entity(actual,A,'buprenorphine-pain-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s40-16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s40-16',2,ref(4),[A]),'instead-of','$guideline_id'(product,'cdc2022-opioid-s40-16',2,ref(3),[A])) :- guideline_entity(actual,A,'buprenorphine-pain-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_preference('$guideline_id'(context,'cdc2022-opioid-s40-16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s40-16',2,ref(4),[A]),'$guideline_id'(product,'cdc2022-opioid-s40-16',2,ref(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s40-16',2,ref(3),[A])) :- guideline_entity(actual,A,'buprenorphine-pain-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every opioid-use-disorder-buprenorphine-treatment uses a once-a-day-dosing.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s40-16',3,ref(2),[A]),'once-a-day-dosing',countable) :- guideline_entity(actual,A,'opioid-use-disorder-buprenorphine-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s40-16',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-use-disorder-buprenorphine-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s40-16',3,ref(3),[A]),use) :- guideline_entity(actual,A,'opioid-use-disorder-buprenorphine-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s40-16',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-use-disorder-buprenorphine-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s40-16',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s40-16',3,ref(2),[A])) :- guideline_entity(actual,A,'opioid-use-disorder-buprenorphine-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every buprenorphine-pain-dosing can typically give at least 2 buprenorphine-doses per 1 day.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s40-16',4,box(1),[A]),can) :- guideline_entity(actual,A,'buprenorphine-pain-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s40-16',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s40-16',4,ref(2),[A]),'buprenorphine-dose',countable) :- guideline_entity(actual,A,'buprenorphine-pain-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s40-16',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s40-16',4,ref(2),[A]),na,geq,2) :- guideline_entity(actual,A,'buprenorphine-pain-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s40-16',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s40-16',4,ref(3),[A]),day,countable) :- guideline_entity(actual,A,'buprenorphine-pain-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s40-16',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s40-16',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'buprenorphine-pain-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_unit('$guideline_id'(context,'cdc2022-opioid-s40-16',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s40-16',4,ref(3),[A]),day) :- guideline_entity(actual,A,'buprenorphine-pain-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s40-16',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s40-16',4,ref(4),[A]),usuality,typically) :- guideline_entity(actual,A,'buprenorphine-pain-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s40-16',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s40-16',4,ref(4),[A]),give) :- guideline_entity(actual,A,'buprenorphine-pain-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s40-16',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s40-16',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,'buprenorphine-pain-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s40-16',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s40-16',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s40-16',4,ref(2),[A])) :- guideline_entity(actual,A,'buprenorphine-pain-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s40-16',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s40-16',4,ref(4),[A]),per,'$guideline_id'(product,'cdc2022-opioid-s40-16',4,ref(3),[A])) :- guideline_entity(actual,A,'buprenorphine-pain-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_frequency('$guideline_id'(context,'cdc2022-opioid-s40-16',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s40-16',4,ref(4),[A]),'$guideline_id'(product,'cdc2022-opioid-s40-16',4,ref(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s40-16',4,ref(3),[A]),day) :- guideline_entity(actual,A,'buprenorphine-pain-dosing',countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every opioid-use-disorder-buprenorphine-treatment gives exactly 1 buprenorphine-dose per 1 day.
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s40-16',5,ref(2),[A]),per,'$guideline_id'(product,'cdc2022-opioid-s40-16',5,ref(3),[A])) :- guideline_entity(actual,A,'opioid-use-disorder-buprenorphine-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_frequency(actual,'$guideline_id'(product,'cdc2022-opioid-s40-16',5,ref(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s40-16',5,ref(4),[A]),'$guideline_id'(product,'cdc2022-opioid-s40-16',5,ref(3),[A]),day) :- guideline_entity(actual,A,'opioid-use-disorder-buprenorphine-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s40-16',5,ref(2),[A]),give) :- guideline_entity(actual,A,'opioid-use-disorder-buprenorphine-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s40-16',5,ref(2),[A]),1,A) :- guideline_entity(actual,A,'opioid-use-disorder-buprenorphine-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s40-16',5,ref(2),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s40-16',5,ref(4),[A])) :- guideline_entity(actual,A,'opioid-use-disorder-buprenorphine-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s40-16',5,ref(3),[A]),day,countable) :- guideline_entity(actual,A,'opioid-use-disorder-buprenorphine-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s40-16',5,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-use-disorder-buprenorphine-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_unit(actual,'$guideline_id'(product,'cdc2022-opioid-s40-16',5,ref(3),[A]),day) :- guideline_entity(actual,A,'opioid-use-disorder-buprenorphine-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s40-16',5,ref(4),[A]),'buprenorphine-dose',countable) :- guideline_entity(actual,A,'opioid-use-disorder-buprenorphine-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s40-16',5,ref(4),[A]),na,exactly,1) :- guideline_entity(actual,A,'opioid-use-disorder-buprenorphine-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
