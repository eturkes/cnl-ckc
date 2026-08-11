% cdc2022-opioid-rec12.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec12',ace_sha256('81ba979f40a8af94fbddaf2bb5269245aa7d148af8b151e60ef47444d61aa0aa'),ulex(sha256('26c94eb4c3e425db0b6c278b6861f816b856b8629608554394634e35ee8e1f3c'))).
% S1: A recommendation is a category-A-recommendation and is an evidence-type-1-recommendation.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(1),[]),recommendation,countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(2),[]),'category-A-recommendation',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(3),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(4),[]),'evidence-type-1-recommendation',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(5),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(4),[])).
% S2: Every clinician should offer an evidence-based-medication-treatment for a patient who has an opioid-use-disorder.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec12',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec12',2,ref(2),[A]),'evidence-based-medication-treatment',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec12',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec12',2,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec12',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec12',2,ref(4),[A]),'opioid-use-disorder',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec12',2,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec12',2,ref(5),[A]),have) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec12',2,ref(5),[A]),1,'$guideline_id'(product,'cdc2022-opioid-rec12',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec12',2,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec12',2,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec12',2,ref(6),[A]),offer) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec12',2,ref(6),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec12',2,ref(6),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec12',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec12',2,ref(6),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec12',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: A medication-free-detoxification is not a recommended-treatment for an opioid-use-disorder.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',3,ref(1),[]),'medication-free-detoxification',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',3,ref(1),[]),na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec12',3,box(1),[]),-).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec12',3,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec12',3,ref(2),[]),'recommended-treatment',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec12',3,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec12',3,ref(2),[]),na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec12',3,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec12',3,ref(3),[]),'opioid-use-disorder',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec12',3,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec12',3,ref(3),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec12',3,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec12',3,ref(4),[]),be).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec12',3,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec12',3,ref(4),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec12',3,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec12',3,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec12',3,ref(4),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec12',3,ref(2),[])).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec12',3,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec12',3,ref(4),[]),for,'$guideline_id'(product,'cdc2022-opioid-rec12',3,ref(3),[])).
% S4: Every medication-free-detoxification increases a risk for a drug-use-resumption and increases a risk for an overdose and increases a risk for an overdose-death.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(2),[A]),risk,countable) :- guideline_entity(actual,A,'medication-free-detoxification',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'medication-free-detoxification',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(3),[A]),'drug-use-resumption',countable) :- guideline_entity(actual,A,'medication-free-detoxification',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'medication-free-detoxification',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(4),[A]),increase) :- guideline_entity(actual,A,'medication-free-detoxification',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,'medication-free-detoxification',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(2),[A])) :- guideline_entity(actual,A,'medication-free-detoxification',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(3),[A])) :- guideline_entity(actual,A,'medication-free-detoxification',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(5),[A]),risk,countable) :- guideline_entity(actual,A,'medication-free-detoxification',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(5),[A]),na,eq,1) :- guideline_entity(actual,A,'medication-free-detoxification',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(6),[A]),overdose,countable) :- guideline_entity(actual,A,'medication-free-detoxification',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(6),[A]),na,eq,1) :- guideline_entity(actual,A,'medication-free-detoxification',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(7),[A]),increase) :- guideline_entity(actual,A,'medication-free-detoxification',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(7),[A]),1,A) :- guideline_entity(actual,A,'medication-free-detoxification',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(7),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(5),[A])) :- guideline_entity(actual,A,'medication-free-detoxification',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(7),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(6),[A])) :- guideline_entity(actual,A,'medication-free-detoxification',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(8),[A]),risk,countable) :- guideline_entity(actual,A,'medication-free-detoxification',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(8),[A]),na,eq,1) :- guideline_entity(actual,A,'medication-free-detoxification',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(9),[A]),'overdose-death',countable) :- guideline_entity(actual,A,'medication-free-detoxification',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(9),[A]),na,eq,1) :- guideline_entity(actual,A,'medication-free-detoxification',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(10),[A]),increase) :- guideline_entity(actual,A,'medication-free-detoxification',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(10),[A]),1,A) :- guideline_entity(actual,A,'medication-free-detoxification',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(10),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(8),[A])) :- guideline_entity(actual,A,'medication-free-detoxification',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(10),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec12',4,ref(9),[A])) :- guideline_entity(actual,A,'medication-free-detoxification',countable), guideline_cardinality(actual,A,na,eq,1).
