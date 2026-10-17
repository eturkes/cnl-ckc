% cdc2022-opioid-s27-03.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_schema_version(1).
guideline_document('cdc2022-opioid-s27-03',ace_sha256('284c69146967cfe2f1732c77f17e32041012e165cd1934bdff705b82474c77ca'),ulex(sha256(d08a8bd3e80ef6e01301a079bb5496315f35233bf199aa89b2d2ccc93b858776)),vocabulary(sha256('9d7632637554f8cbfe72e7d34a9b9d14c29eb8593aebb1c128a870c5dcf77ead'))).
% S1: Every clinician should use a systemic-NSAID at a lowest-effective-dose during a shortest-needed-duration.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s27-03',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s27-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s27-03',1,ref(2),[A]),'systemic-NSAID',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s27-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s27-03',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s27-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s27-03',1,ref(3),[A]),'lowest-effective-dose',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s27-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s27-03',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s27-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s27-03',1,ref(4),[A]),'shortest-needed-duration',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s27-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s27-03',1,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s27-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s27-03',1,ref(5),[A]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s27-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s27-03',1,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s27-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s27-03',1,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s27-03',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s27-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s27-03',1,ref(5),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s27-03',1,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s27-03',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s27-03',1,ref(5),[A]),at,'$guideline_id'(product,'cdc2022-opioid-s27-03',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: A systemic-NSAID-risk can possibly increase during a longer-NSAID-use and can possibly increase at a higher-NSAID-dosage.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s27-03',2,ref(1),[]),'systemic-NSAID-risk',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s27-03',2,ref(1),[]),na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s27-03',2,box(1),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s27-03',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s27-03',2,ref(2),[]),'longer-NSAID-use',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s27-03',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s27-03',2,ref(2),[]),na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s27-03',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s27-03',2,ref(3),[]),possibility,possibly).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s27-03',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s27-03',2,ref(3),[]),increase).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s27-03',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s27-03',2,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s27-03',2,ref(1),[])).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s27-03',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s27-03',2,ref(3),[]),during,'$guideline_id'(product,'cdc2022-opioid-s27-03',2,ref(2),[])).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s27-03',2,box(2),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s27-03',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-s27-03',2,ref(4),[]),'higher-NSAID-dosage',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s27-03',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-s27-03',2,ref(4),[]),na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s27-03',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-s27-03',2,ref(5),[]),possibility,possibly).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s27-03',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-s27-03',2,ref(5),[]),increase).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s27-03',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-s27-03',2,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s27-03',2,ref(1),[])).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s27-03',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-s27-03',2,ref(5),[]),at,'$guideline_id'(product,'cdc2022-opioid-s27-03',2,ref(4),[])).
