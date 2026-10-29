% cdc2022-opioid-s50-17.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s50-17',ace_sha256('91783460babbe319a269a05d77acebd9ae2e2ee81b907580ab3a56944efeb030'),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: Every naloxone is an opioid-antagonist.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s50-17',1,ref(2),[A]),'opioid-antagonist',countable) :- guideline_entity(actual,A,naloxone,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s50-17',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,naloxone,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s50-17',1,ref(3),[A]),be) :- guideline_entity(actual,A,naloxone,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s50-17',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,naloxone,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s50-17',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s50-17',1,ref(2),[A])) :- guideline_entity(actual,A,naloxone,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every naloxone can reverse a severe-respiratory-depression.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s50-17',2,box(1),[A]),can) :- guideline_entity(actual,A,naloxone,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s50-17',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s50-17',2,ref(2),[A]),'severe-respiratory-depression',countable) :- guideline_entity(actual,A,naloxone,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s50-17',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s50-17',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,naloxone,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s50-17',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s50-17',2,ref(3),[A]),reverse) :- guideline_entity(actual,A,naloxone,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s50-17',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s50-17',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,naloxone,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s50-17',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s50-17',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s50-17',2,ref(2),[A])) :- guideline_entity(actual,A,naloxone,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: If a layperson administers a naloxone to a person that experiences an opioid-overdose then the layperson can save the person.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s50-17',3,box(1),[A,B,C,D,E,F]),can) :- guideline_entity(actual,A,layperson,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,naloxone,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,person,countable), guideline_cardinality(actual,C,na,eq,1), guideline_entity(actual,D,'opioid-overdose',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,experience), guideline_arg(actual,E,1,C), guideline_arg(actual,E,2,D), guideline_event(actual,F,administer), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,B), guideline_pp(actual,F,to,C).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s50-17',3,box(1),[A,B,C,D,E,F]),'$guideline_id'(product,'cdc2022-opioid-s50-17',3,ref(7),[A,B,C,D,E,F]),save) :- guideline_entity(actual,A,layperson,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,naloxone,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,person,countable), guideline_cardinality(actual,C,na,eq,1), guideline_entity(actual,D,'opioid-overdose',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,experience), guideline_arg(actual,E,1,C), guideline_arg(actual,E,2,D), guideline_event(actual,F,administer), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,B), guideline_pp(actual,F,to,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s50-17',3,box(1),[A,B,C,D,E,F]),'$guideline_id'(product,'cdc2022-opioid-s50-17',3,ref(7),[A,B,C,D,E,F]),1,A) :- guideline_entity(actual,A,layperson,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,naloxone,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,person,countable), guideline_cardinality(actual,C,na,eq,1), guideline_entity(actual,D,'opioid-overdose',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,experience), guideline_arg(actual,E,1,C), guideline_arg(actual,E,2,D), guideline_event(actual,F,administer), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,B), guideline_pp(actual,F,to,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s50-17',3,box(1),[A,B,C,D,E,F]),'$guideline_id'(product,'cdc2022-opioid-s50-17',3,ref(7),[A,B,C,D,E,F]),2,C) :- guideline_entity(actual,A,layperson,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,naloxone,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,person,countable), guideline_cardinality(actual,C,na,eq,1), guideline_entity(actual,D,'opioid-overdose',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,experience), guideline_arg(actual,E,1,C), guideline_arg(actual,E,2,D), guideline_event(actual,F,administer), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,B), guideline_pp(actual,F,to,C).
% S4: A layperson-example includes a friend-example and includes a family-member-example and includes a caregiver-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s50-17',4,ref(1),[]),'layperson-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s50-17',4,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s50-17',4,ref(2),[]),'friend-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s50-17',4,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s50-17',4,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s50-17',4,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s50-17',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s50-17',4,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s50-17',4,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s50-17',4,ref(4),[]),'family-member-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s50-17',4,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s50-17',4,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s50-17',4,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s50-17',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s50-17',4,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s50-17',4,ref(4),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s50-17',4,ref(6),[]),'caregiver-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s50-17',4,ref(6),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s50-17',4,ref(7),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s50-17',4,ref(7),[]),1,'$guideline_id'(product,'cdc2022-opioid-s50-17',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s50-17',4,ref(7),[]),2,'$guideline_id'(product,'cdc2022-opioid-s50-17',4,ref(6),[])).
