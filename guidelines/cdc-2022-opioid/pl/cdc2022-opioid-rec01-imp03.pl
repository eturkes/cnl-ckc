% cdc2022-opioid-rec01-imp03.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec01-imp03',ace_sha256('8ae2b3b1dd404f80436814e9fecce91a9e5c828428070ec7cc56d861d415f17c'),ulex(sha256('0037ea725bdd10cf000798ce586d3739368b01cba88ac29668dbf1e4c9414552')),vocabulary(sha256('9d7632637554f8cbfe72e7d34a9b9d14c29eb8593aebb1c128a870c5dcf77ead'))).
% S1: Every clinician should prescribe an as-needed-opioid instead-of a scheduled-opioid and should advise an as-needed-opioid instead-of a scheduled-opioid.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec01-imp03',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec01-imp03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',1,ref(2),[A]),'as-needed-opioid',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec01-imp03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec01-imp03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',1,ref(3),[A]),'scheduled-opioid',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec01-imp03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec01-imp03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',1,ref(4),[A]),prescribe) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec01-imp03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec01-imp03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec01-imp03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',1,ref(4),[A]),'instead-of','$guideline_id'(product,'cdc2022-opioid-rec01-imp03',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_preference('$guideline_id'(context,'cdc2022-opioid-rec01-imp03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',1,ref(4),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',1,ref(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec01-imp03',1,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec01-imp03',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',1,ref(5),[A]),'as-needed-opioid',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec01-imp03',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',1,ref(5),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec01-imp03',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',1,ref(6),[A]),'scheduled-opioid',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec01-imp03',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',1,ref(6),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec01-imp03',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',1,ref(7),[A]),advise) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec01-imp03',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',1,ref(7),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec01-imp03',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',1,ref(7),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',1,ref(5),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec01-imp03',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',1,ref(7),[A]),'instead-of','$guideline_id'(product,'cdc2022-opioid-rec01-imp03',1,ref(6),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_preference('$guideline_id'(context,'cdc2022-opioid-rec01-imp03',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',1,ref(7),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',1,ref(5),[A]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',1,ref(6),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: An as-needed-opioid-example contains 5 hydrocodone-milligrams and contains 325 acetaminophen-milligrams and uses 1 tablet at an interval of at least 4 hours for a moderate-to-severe-pain.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(1),[]),'as-needed-opioid-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(2),[]),'hydrocodone-milligram',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(2),[]),na,eq,5).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(3),[]),contain).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(4),[]),'acetaminophen-milligram',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(4),[]),na,eq,325).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(5),[]),contain).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(4),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(6),[]),tablet,countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(6),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(7),[]),interval,countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(7),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(8),[]),hour,countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(8),[]),na,geq,4).
guideline_unit(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(8),[]),hour).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(9),[]),'moderate-to-severe-pain',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(9),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(10),[]),use).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(10),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(10),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(6),[])).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(10),[]),for,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(9),[])).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(10),[]),at,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(7),[])).
guideline_recurrence(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(10),[]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',2,ref(8),[]),hour).
% S3: A scheduled-opioid-example uses 1 tablet at an interval of 4 hours.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',3,ref(1),[]),'scheduled-opioid-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',3,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',3,ref(2),[]),tablet,countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',3,ref(2),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',3,ref(3),[]),interval,countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',3,ref(3),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',3,ref(4),[]),hour,countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',3,ref(4),[]),na,eq,4).
guideline_unit(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',3,ref(4),[]),hour).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',3,ref(5),[]),use).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',3,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',3,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',3,ref(2),[])).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',3,ref(5),[]),at,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',3,ref(3),[])).
guideline_recurrence(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',3,ref(5),[]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',3,ref(4),[]),hour).
% S4: If a patient has a prolonged-around-the-clock-opioid-use for a few-day-period then every clinician should encourage an opioid-taper and should recommend an opioid-taper.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec01-imp03',4,box(1),[A,B,C,D,E]),should) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-around-the-clock-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec01-imp03',4,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',4,ref(6),[A,B,C,D,E]),'opioid-taper',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-around-the-clock-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec01-imp03',4,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',4,ref(6),[A,B,C,D,E]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-around-the-clock-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec01-imp03',4,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',4,ref(7),[A,B,C,D,E]),encourage) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-around-the-clock-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec01-imp03',4,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',4,ref(7),[A,B,C,D,E]),1,E) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-around-the-clock-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec01-imp03',4,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',4,ref(7),[A,B,C,D,E]),2,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',4,ref(6),[A,B,C,D,E])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-around-the-clock-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec01-imp03',4,box(2),[A,B,C,D,E]),should) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-around-the-clock-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec01-imp03',4,box(2),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',4,ref(8),[A,B,C,D,E]),'opioid-taper',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-around-the-clock-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec01-imp03',4,box(2),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',4,ref(8),[A,B,C,D,E]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-around-the-clock-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec01-imp03',4,box(2),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',4,ref(9),[A,B,C,D,E]),recommend) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-around-the-clock-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec01-imp03',4,box(2),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',4,ref(9),[A,B,C,D,E]),1,E) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-around-the-clock-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec01-imp03',4,box(2),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',4,ref(9),[A,B,C,D,E]),2,'$guideline_id'(product,'cdc2022-opioid-rec01-imp03',4,ref(8),[A,B,C,D,E])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-around-the-clock-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
