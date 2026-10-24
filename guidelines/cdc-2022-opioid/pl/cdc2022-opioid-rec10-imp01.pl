% cdc2022-opioid-rec10-imp01.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec10-imp01',ace_sha256(b30ef961e3146c06ad2a5b968b95db6b3c4e5e2b2356a45da7b75032a3681b15),ulex(sha256('45608095e3f77853fdabcce148a9eb7633a8ae776353b8f008c753b846886588')),vocabulary(sha256(c36289aa40f75805a83501ca057b29dd1ba7292b025716853f654890b77ec50c))).
% S1: If there is a clinician then it is recommended that the clinician does not use a toxicology-testing punitively.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp01',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',1,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec10-imp01',1,box(2),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',1,ref(2),[A]),'toxicology-testing',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',1,ref(3),[A]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',1,ref(3),[A]),manner,punitively) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should use a toxicology-testing during a clinical-decision.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp01',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',2,ref(2),[A]),'toxicology-testing',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',2,ref(3),[A]),'clinical-decision',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',2,ref(4),[A]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',2,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should consider an other-clinical-information during a clinical-decision.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp01',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',3,ref(2),[A]),'other-clinical-information',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',3,ref(3),[A]),'clinical-decision',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',3,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',3,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician should use a toxicology-testing during a patient-care-improvement.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp01',4,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',4,ref(2),[A]),'toxicology-testing',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',4,ref(3),[A]),'patient-care-improvement',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',4,ref(4),[A]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',4,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every clinician should consider an other-clinical-information during a patient-care-improvement.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp01',5,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',5,ref(2),[A]),'other-clinical-information',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',5,ref(3),[A]),'patient-care-improvement',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',5,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',5,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',5,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',5,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',5,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec10-imp01',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',5,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec10-imp01',5,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
