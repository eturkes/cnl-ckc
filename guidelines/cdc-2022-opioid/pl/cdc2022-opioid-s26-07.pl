% cdc2022-opioid-s26-07.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s26-07',ace_sha256('9a425a88e749f7d34e7535e434dbddd40e31da7e42cf6831ab4c1f8218c0c22f'),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: Every primary-care-clinician may integrate a psychosocial-therapy-element.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s26-07',1,box(1),[A]),may) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',1,ref(2),[A]),'psychosocial-therapy-element',countable) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s26-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',1,ref(3),[A]),integrate) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s26-07',1,ref(2),[A])) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: A psychosocial-therapy-example is a cognitive-behavioral-therapy-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s26-07',2,ref(1),[]),'psychosocial-therapy-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s26-07',2,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s26-07',2,ref(2),[]),'cognitive-behavioral-therapy-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s26-07',2,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s26-07',2,ref(3),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s26-07',2,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s26-07',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s26-07',2,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s26-07',2,ref(2),[])).
% S3: Every cognitive-behavioral-therapy can address a psychosocial-pain-contributor and can improve a function.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(1),[A]),can) :- guideline_entity(actual,A,'cognitive-behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(2),[A]),'psychosocial-pain-contributor',countable) :- guideline_entity(actual,A,'cognitive-behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'cognitive-behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(3),[A]),address) :- guideline_entity(actual,A,'cognitive-behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,'cognitive-behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(2),[A])) :- guideline_entity(actual,A,'cognitive-behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(2),[A]),can) :- guideline_entity(actual,A,'cognitive-behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(4),[A]),function,countable) :- guideline_entity(actual,A,'cognitive-behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'cognitive-behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(5),[A]),improve) :- guideline_entity(actual,A,'cognitive-behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(5),[A]),1,A) :- guideline_entity(actual,A,'cognitive-behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s26-07',3,ref(4),[A])) :- guideline_entity(actual,A,'cognitive-behavioral-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every primary-care-clinician may encourage an active-care-plan-role for a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s26-07',4,box(1),[A]),may) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(2),[A]),'active-care-plan-role',countable) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s26-07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(4),[A]),encourage) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(2),[A])) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s26-07',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s26-07',4,ref(3),[A])) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every primary-care-clinician may support a beneficial-activity-engagement for a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s26-07',5,box(1),[A]),may) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-07',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',5,ref(2),[A]),'beneficial-activity-engagement',countable) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-07',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-07',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',5,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-07',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',5,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s26-07',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',5,ref(4),[A]),support) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',5,ref(4),[A]),1,A) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',5,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s26-07',5,ref(2),[A])) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s26-07',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',5,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s26-07',5,ref(3),[A])) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
% S6: A beneficial-activity-example is an exercise-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s26-07',6,ref(1),[]),'beneficial-activity-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s26-07',6,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s26-07',6,ref(2),[]),'exercise-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s26-07',6,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s26-07',6,ref(3),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s26-07',6,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s26-07',6,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s26-07',6,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s26-07',6,ref(2),[])).
% S7: Every exercise can typically benefit a patient and can possibly relate to an initial-pain-exacerbation-fear.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s26-07',7,box(1),[A]),can) :- guideline_entity(actual,A,exercise,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-07',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',7,ref(2),[A]),patient,countable) :- guideline_entity(actual,A,exercise,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-07',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',7,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,exercise,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s26-07',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',7,ref(3),[A]),usuality,typically) :- guideline_entity(actual,A,exercise,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s26-07',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',7,ref(3),[A]),benefit) :- guideline_entity(actual,A,exercise,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',7,ref(3),[A]),1,A) :- guideline_entity(actual,A,exercise,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',7,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s26-07',7,ref(2),[A])) :- guideline_entity(actual,A,exercise,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s26-07',7,box(2),[A]),can) :- guideline_entity(actual,A,exercise,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-07',7,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',7,ref(4),[A]),'initial-pain-exacerbation-fear',countable) :- guideline_entity(actual,A,exercise,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-07',7,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',7,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,exercise,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s26-07',7,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',7,ref(5),[A]),possibility,possibly) :- guideline_entity(actual,A,exercise,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s26-07',7,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',7,ref(5),[A]),relate) :- guideline_entity(actual,A,exercise,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',7,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',7,ref(5),[A]),1,A) :- guideline_entity(actual,A,exercise,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s26-07',7,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',7,ref(5),[A]),to,'$guideline_id'(product,'cdc2022-opioid-s26-07',7,ref(4),[A])) :- guideline_entity(actual,A,exercise,countable), guideline_cardinality(actual,A,na,eq,1).
% S8: Every primary-care-clinician may provide a relaxation-technique-education to a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s26-07',8,box(1),[A]),may) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-07',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',8,ref(2),[A]),'relaxation-technique-education',countable) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-07',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',8,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-07',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',8,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-07',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',8,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s26-07',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',8,ref(4),[A]),provide) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',8,ref(4),[A]),1,A) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',8,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s26-07',8,ref(2),[A])) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',8,ref(4),[A]),3,'$guideline_id'(product,'cdc2022-opioid-s26-07',8,ref(3),[A])) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
% S9: Every primary-care-clinician may provide a coping-strategy-education to a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s26-07',9,box(1),[A]),may) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-07',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',9,ref(2),[A]),'coping-strategy-education',countable) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-07',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',9,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-07',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',9,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-07',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',9,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s26-07',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',9,ref(4),[A]),provide) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',9,ref(4),[A]),1,A) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',9,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s26-07',9,ref(2),[A])) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-07',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-07',9,ref(4),[A]),3,'$guideline_id'(product,'cdc2022-opioid-s26-07',9,ref(3),[A])) :- guideline_entity(actual,A,'primary-care-clinician',countable), guideline_cardinality(actual,A,na,eq,1).
