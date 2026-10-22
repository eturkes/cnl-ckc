% cdc2022-opioid-s9-05.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_schema_version(1).
guideline_document('cdc2022-opioid-s9-05',ace_sha256('4aaf8c5bdee6435713eb4a7f0a701863d5f33b0e1186d380d9508e9fb5021e86'),ulex(sha256(e8b9f2411b72a24bb2298ca9b1abaf93606dc5038829e6a3043fa9931bbc5ef8)),vocabulary(sha256(b5cff67a2f9de9bee846e9e7650affaf95276ff3a26fc00befd8ceb076b0f4ac))).
% S1: Every opioid-pain-clinical-practice-guideline is a clinical-tool.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',1,ref(2),[A]),'clinical-tool',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',1,ref(3),[A]),be) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-05',1,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every opioid-pain-clinical-practice-guideline should improve a clinician-patient-communication.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-05',2,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',2,ref(2),[A]),'clinician-patient-communication',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',2,ref(3),[A]),improve) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-05',2,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every opioid-pain-clinical-practice-guideline should support an informed-person-centered-pain-care-decision.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-05',3,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',3,ref(2),[A]),'informed-person-centered-pain-care-decision',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',3,ref(3),[A]),support) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-05',3,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every opioid-pain-clinical-practice-guideline applies to a primary-care-clinician.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',4,ref(2),[A]),'primary-care-clinician',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',4,ref(3),[A]),apply) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',4,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',4,ref(3),[A]),to,'$guideline_id'(product,'cdc2022-opioid-s9-05',4,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every opioid-pain-clinical-practice-guideline applies to a pain-care-clinician.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',5,ref(2),[A]),'pain-care-clinician',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',5,ref(3),[A]),apply) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',5,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',5,ref(3),[A]),to,'$guideline_id'(product,'cdc2022-opioid-s9-05',5,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S6: If an outpatient has an age and the age includes at least 18 years then the outpatient is an adult-outpatient.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',6,ref(6),[A,B,C,D,E]),'adult-outpatient',countable) :- guideline_entity(actual,A,outpatient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,age,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_cardinality(actual,D,na,F,G), guideline_unit(actual,D,H), guideline_satisfies(F,G,H,geq,18,year), guideline_event(actual,E,include), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',6,ref(6),[A,B,C,D,E]),na,eq,1) :- guideline_entity(actual,A,outpatient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,age,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_cardinality(actual,D,na,F,G), guideline_unit(actual,D,H), guideline_satisfies(F,G,H,geq,18,year), guideline_event(actual,E,include), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',6,ref(7),[A,B,C,D,E]),be) :- guideline_entity(actual,A,outpatient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,age,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_cardinality(actual,D,na,F,G), guideline_unit(actual,D,H), guideline_satisfies(F,G,H,geq,18,year), guideline_event(actual,E,include), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',6,ref(7),[A,B,C,D,E]),1,A) :- guideline_entity(actual,A,outpatient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,age,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_cardinality(actual,D,na,F,G), guideline_unit(actual,D,H), guideline_satisfies(F,G,H,geq,18,year), guideline_event(actual,E,include), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',6,ref(7),[A,B,C,D,E]),2,'$guideline_id'(product,'cdc2022-opioid-s9-05',6,ref(6),[A,B,C,D,E])) :- guideline_entity(actual,A,outpatient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,age,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_cardinality(actual,D,na,F,G), guideline_unit(actual,D,H), guideline_satisfies(F,G,H,geq,18,year), guideline_event(actual,E,include), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D).
% S7: Every opioid-pain-clinical-practice-guideline applies to an adult-outpatient that has a pain that lasts for less than 1 month.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',7,ref(2),[A]),'adult-outpatient',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',7,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',7,ref(3),[A]),pain,countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',7,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',7,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s9-05',7,ref(5),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_interval(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',7,ref(4),[A]),duration,'$guideline_id'(product,'cdc2022-opioid-s9-05',7,ref(5),[A]),month,none) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',7,ref(4),[A]),last) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',7,ref(4),[A]),1,'$guideline_id'(product,'cdc2022-opioid-s9-05',7,ref(3),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',7,ref(5),[A]),month,countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',7,ref(5),[A]),na,less,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_unit(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',7,ref(5),[A]),month) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',7,ref(6),[A]),have) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',7,ref(6),[A]),1,'$guideline_id'(product,'cdc2022-opioid-s9-05',7,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',7,ref(6),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-05',7,ref(3),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',7,ref(7),[A]),apply) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',7,ref(7),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',7,ref(7),[A]),to,'$guideline_id'(product,'cdc2022-opioid-s9-05',7,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S8: Every opioid-pain-clinical-practice-guideline applies to an adult-outpatient that has a pain that lasts for at least 1 month for at most 3 months.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(2),[A]),'adult-outpatient',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(3),[A]),pain,countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(4),[A]),month,countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(4),[A]),na,geq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_unit(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(4),[A]),month) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(5),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(4),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_interval(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(5),[A]),duration,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(4),[A]),month,none) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(5),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(6),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_interval(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(5),[A]),duration,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(6),[A]),month,none) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(5),[A]),last) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(5),[A]),1,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(3),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(6),[A]),month,countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(6),[A]),na,leq,3) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_unit(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(6),[A]),month) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(7),[A]),have) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(7),[A]),1,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(7),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(3),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(8),[A]),apply) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(8),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(8),[A]),to,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S9: Every opioid-pain-clinical-practice-guideline applies to an adult-outpatient that has a pain that lasts for more than 3 months.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',9,ref(2),[A]),'adult-outpatient',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',9,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',9,ref(3),[A]),pain,countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',9,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',9,ref(4),[A]),month,countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',9,ref(4),[A]),na,greater,3) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_unit(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',9,ref(4),[A]),month) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',9,ref(5),[A]),last) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',9,ref(5),[A]),1,'$guideline_id'(product,'cdc2022-opioid-s9-05',9,ref(3),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',9,ref(5),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s9-05',9,ref(4),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_interval(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',9,ref(5),[A]),duration,'$guideline_id'(product,'cdc2022-opioid-s9-05',9,ref(4),[A]),month,none) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',9,ref(6),[A]),have) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',9,ref(6),[A]),1,'$guideline_id'(product,'cdc2022-opioid-s9-05',9,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',9,ref(6),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-05',9,ref(3),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',9,ref(7),[A]),apply) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',9,ref(7),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',9,ref(7),[A]),to,'$guideline_id'(product,'cdc2022-opioid-s9-05',9,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S10: Every opioid-pain-clinical-practice-guideline is a flexible-guideline.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',10,ref(2),[A]),'flexible-guideline',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',10,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',10,ref(3),[A]),be) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',10,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',10,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-05',10,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S11: Every opioid-pain-clinical-practice-guideline should support a person-centered-decision.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-05',11,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-05',11,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',11,ref(2),[A]),'person-centered-decision',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-05',11,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',11,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-05',11,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',11,ref(3),[A]),support) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-05',11,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',11,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-05',11,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',11,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-05',11,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S12: Every clinician should consider an expected-health-outcome during a person-centered-decision.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-05',12,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-05',12,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',12,ref(2),[A]),'expected-health-outcome',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-05',12,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',12,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-05',12,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',12,ref(3),[A]),'person-centered-decision',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-05',12,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',12,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-05',12,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',12,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-05',12,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',12,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-05',12,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',12,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-05',12,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s9-05',12,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',12,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s9-05',12,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S13: Every clinician should consider a well-being during a person-centered-decision.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-05',13,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-05',13,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',13,ref(2),[A]),'well-being',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-05',13,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',13,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-05',13,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',13,ref(3),[A]),'person-centered-decision',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-05',13,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',13,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-05',13,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',13,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-05',13,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',13,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-05',13,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',13,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-05',13,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s9-05',13,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',13,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s9-05',13,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
