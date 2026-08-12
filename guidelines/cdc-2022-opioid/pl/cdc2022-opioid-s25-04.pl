% cdc2022-opioid-s25-04.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
:- multifile(guideline_schema_version/1).
:- discontiguous(guideline_schema_version/1).
:- multifile(guideline_document/3).
:- discontiguous(guideline_document/3).
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
guideline_schema_version(1).
guideline_document('cdc2022-opioid-s25-04',ace_sha256('860f4a2c6031d5949e96f4efd4978320ee026405193ca194a957ff771dfa34fa'),ulex(sha256('06c1c26d6b6abe1436b8c94a276937ad072806062117417de2ab8086fdf9a3e9'))).
% S1: Every clinician should conduct a focused-history during a pain-evaluation.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s25-04',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s25-04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-04',1,ref(2),[A]),'focused-history',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s25-04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-04',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s25-04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-04',1,ref(3),[A]),'pain-evaluation',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s25-04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-04',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s25-04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-04',1,ref(4),[A]),conduct) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s25-04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-04',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s25-04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-04',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s25-04',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s25-04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-04',1,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s25-04',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should conduct a physical-examination during a pain-evaluation.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s25-04',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s25-04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-04',2,ref(2),[A]),'physical-examination',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s25-04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-04',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s25-04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-04',2,ref(3),[A]),'pain-evaluation',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s25-04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-04',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s25-04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-04',2,ref(4),[A]),conduct) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s25-04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-04',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s25-04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-04',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s25-04',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s25-04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s25-04',2,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s25-04',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: If a diagnostic-testing is indicated during a pain-evaluation then every clinician should conduct the diagnostic-testing during the pain-evaluation.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s25-04',3,box(1),[A,B,C,D,E]),should) :- guideline_entity(actual,A,'diagnostic-testing',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'pain-evaluation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,indicated,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_pp(actual,D,during,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s25-04',3,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s25-04',3,ref(6),[A,B,C,D,E]),conduct) :- guideline_entity(actual,A,'diagnostic-testing',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'pain-evaluation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,indicated,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_pp(actual,D,during,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s25-04',3,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s25-04',3,ref(6),[A,B,C,D,E]),1,E) :- guideline_entity(actual,A,'diagnostic-testing',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'pain-evaluation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,indicated,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_pp(actual,D,during,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s25-04',3,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s25-04',3,ref(6),[A,B,C,D,E]),2,A) :- guideline_entity(actual,A,'diagnostic-testing',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'pain-evaluation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,indicated,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_pp(actual,D,during,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s25-04',3,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s25-04',3,ref(6),[A,B,C,D,E]),during,B) :- guideline_entity(actual,A,'diagnostic-testing',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'pain-evaluation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,indicated,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_pp(actual,D,during,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
