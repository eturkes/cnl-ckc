% cdc2022-opioid-s9-05.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s9-05',ace_sha256(d6c5c2777ee4172c29bbc13269ca5d67a1c6da6e271e22d1fbd8b77328e9f485),ulex(sha256(ef57ba7180864095bab88b95f11ddb1a2c858593eba715dbfbe874db09506c5b))).
% S1: Every opioid-pain-clinical-practice-guideline is a clinical-tool.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',1,ref(2),[A]),'clinical-tool',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',1,ref(3),[A]),be) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-05',1,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every opioid-pain-clinical-practice-guideline should improve a clinician-patient-communication.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-05',2,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',2,ref(2),[A]),'clinician-patient-communication',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',2,ref(3),[A]),improve) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-05',2,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every opioid-pain-clinical-practice-guideline should support an informed-person-centered-pain-care-decision.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-05',3,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
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
% S6: Every opioid-pain-clinical-practice-guideline applies during an adult-outpatient-acute-pain-under-one-month.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',6,ref(2),[A]),'adult-outpatient-acute-pain-under-one-month',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',6,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',6,ref(3),[A]),apply) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',6,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',6,ref(3),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s9-05',6,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S7: Every opioid-pain-clinical-practice-guideline applies during an adult-outpatient-subacute-pain-one-to-three-months.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',7,ref(2),[A]),'adult-outpatient-subacute-pain-one-to-three-months',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',7,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',7,ref(3),[A]),apply) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',7,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',7,ref(3),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s9-05',7,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S8: Every opioid-pain-clinical-practice-guideline applies during an adult-outpatient-chronic-pain-over-three-months.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(2),[A]),'adult-outpatient-chronic-pain-over-three-months',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(3),[A]),apply) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(3),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s9-05',8,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S9: Every opioid-pain-clinical-practice-guideline is a flexible-guideline.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',9,ref(2),[A]),'flexible-guideline',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',9,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',9,ref(3),[A]),be) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',9,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-05',9,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-05',9,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S10: Every opioid-pain-clinical-practice-guideline should support a person-centered-decision.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-05',10,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-05',10,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',10,ref(2),[A]),'person-centered-decision',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-05',10,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',10,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-05',10,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',10,ref(3),[A]),support) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-05',10,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',10,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-05',10,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',10,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-05',10,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S11: Every clinician should consider an expected-health-outcome during a person-centered-decision.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-05',11,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-05',11,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',11,ref(2),[A]),'expected-health-outcome',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-05',11,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',11,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-05',11,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',11,ref(3),[A]),'person-centered-decision',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-05',11,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',11,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-05',11,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',11,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-05',11,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',11,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-05',11,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',11,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-05',11,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s9-05',11,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',11,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s9-05',11,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S12: Every clinician should consider a well-being during a person-centered-decision.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-05',12,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-05',12,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',12,ref(2),[A]),'well-being',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-05',12,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',12,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-05',12,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',12,ref(3),[A]),'person-centered-decision',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-05',12,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',12,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-05',12,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',12,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-05',12,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',12,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-05',12,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',12,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-05',12,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s9-05',12,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-05',12,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s9-05',12,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
