% cdc2022-opioid-s9-10.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s9-10',ace_sha256('6dd48795ad6ad9c5344a8b57f246a2fe8706548ca67ea95f00767dc630f81fc0'),ulex(sha256('45608095e3f77853fdabcce148a9eb7633a8ae776353b8f008c753b846886588')),vocabulary(sha256(c36289aa40f75805a83501ca057b29dd1ba7292b025716853f654890b77ec50c))).
% S1: Every clinician can possibly work within a team-based-care.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-10',1,box(1),[A]),can) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-10',1,ref(2),[A]),'team-based-care',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-10',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s9-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-10',1,ref(3),[A]),possibility,possibly) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-10',1,ref(3),[A]),work) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-10',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s9-10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-10',1,ref(3),[A]),within,'$guideline_id'(product,'cdc2022-opioid-s9-10',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every opioid-pain-clinical-practice-guideline addresses an integrated-pain-management.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',2,ref(2),[A]),'integrated-pain-management',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',2,ref(3),[A]),address) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-10',2,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every opioid-pain-clinical-practice-guideline promotes an integrated-pain-management.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',3,ref(2),[A]),'integrated-pain-management',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',3,ref(3),[A]),promote) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-10',3,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every opioid-pain-clinical-practice-guideline addresses a collaborative-clinician-working-relationship.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',4,ref(2),[A]),'collaborative-clinician-working-relationship',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',4,ref(3),[A]),address) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',4,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',4,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-10',4,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every opioid-pain-clinical-practice-guideline promotes a collaborative-clinician-working-relationship.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',5,ref(2),[A]),'collaborative-clinician-working-relationship',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',5,ref(3),[A]),promote) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',5,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',5,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-10',5,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S6: A collaborating-clinician-example includes a behavioral-health-specialist-example and includes a pharmacist-example and includes a registered-nurse-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',6,ref(1),[]),'collaborating-clinician-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',6,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',6,ref(2),[]),'behavioral-health-specialist-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',6,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',6,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',6,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s9-10',6,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',6,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s9-10',6,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',6,ref(4),[]),'pharmacist-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',6,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',6,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',6,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s9-10',6,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',6,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s9-10',6,ref(4),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',6,ref(6),[]),'registered-nurse-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',6,ref(6),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',6,ref(7),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',6,ref(7),[]),1,'$guideline_id'(product,'cdc2022-opioid-s9-10',6,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',6,ref(7),[]),2,'$guideline_id'(product,'cdc2022-opioid-s9-10',6,ref(6),[])).
% S7: A behavioral-health-specialist-example includes a social-worker-example and includes a psychologist-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',7,ref(1),[]),'behavioral-health-specialist-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',7,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',7,ref(2),[]),'social-worker-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',7,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',7,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',7,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s9-10',7,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',7,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s9-10',7,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',7,ref(4),[]),'psychologist-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',7,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',7,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',7,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s9-10',7,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-10',7,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s9-10',7,ref(4),[])).
