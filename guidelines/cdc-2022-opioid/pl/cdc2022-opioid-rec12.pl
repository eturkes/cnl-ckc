% cdc2022-opioid-rec12.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec12',ace_sha256('642769777509d3c2c4d5929b6b6221e37ba1499a9e4baa23ded2873b7c4920b4'),ulex(sha256('3210e31a7a1a232faae4aed00d06481f6bef71ebd89b6c884183bc530be9986e')),vocabulary(sha256(b5cff67a2f9de9bee846e9e7650affaf95276ff3a26fc00befd8ceb076b0f4ac))).
% S1: A recommendation is a category-A-recommendation and is an evidence-type-1-recommendation.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(1),[]),recommendation,countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(2),[]),'category-A-recommendation',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(3),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(2),[])).
guideline_grade(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(1),[]),strength,strong,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(4),[]),'evidence-type-1-recommendation',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(5),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(4),[])).
guideline_grade(actual,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(1),[]),evidence,high,'$guideline_id'(product,'cdc2022-opioid-rec12',1,ref(4),[])).
% S2: Every clinician should offer an evidence-based-medication-treatment for a patient who has an opioid-use-disorder.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec12',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
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
% S3: If a patient has an opioid-use-disorder and there is a clinician then it is recommended that the clinician does not use a medication-free-detoxification for the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec12',3,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec12',3,box(1),[A,B,C,D]),'$guideline_id'(context,'cdc2022-opioid-rec12',3,box(2),[A,B,C,D]),-) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec12',3,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec12',3,ref(5),[A,B,C,D]),'medication-free-detoxification',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec12',3,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec12',3,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec12',3,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec12',3,ref(6),[A,B,C,D]),use) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec12',3,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec12',3,ref(6),[A,B,C,D]),1,D) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec12',3,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec12',3,ref(6),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-rec12',3,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec12',3,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec12',3,ref(6),[A,B,C,D]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
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
