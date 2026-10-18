% cdc2022-opioid-rec04-imp09.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec04-imp09',ace_sha256('8c8d132b7386d9f32af8ae587fa6764fa32859f6a153b6ab6ad6ce1e229bc65c'),ulex(sha256(ebe2d3db8559958f9ed44ec7a76cb2fb6ea75017f9f9437c3ea03b07092b0e75)),vocabulary(sha256('9d7632637554f8cbfe72e7d34a9b9d14c29eb8593aebb1c128a870c5dcf77ead'))).
% S1: Every clinician should carefully evaluate a further-dosage-increase with an individualized-benefit-risk-assessment.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec04-imp09',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',1,ref(2),[A]),'further-dosage-increase',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',1,ref(3),[A]),'individualized-benefit-risk-assessment',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',1,ref(4),[A]),manner,carefully) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',1,ref(4),[A]),evaluate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',1,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should consider a diagnosis and should consider an incremental-pain-benefit and should consider an incremental-function-benefit during an individualized-benefit-risk-assessment.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec04-imp09',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',2,ref(2),[A]),diagnosis,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',2,ref(3),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec04-imp09',2,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',2,ref(4),[A]),'incremental-pain-benefit',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',2,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',2,ref(5),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',2,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',2,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',2,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec04-imp09',2,box(3),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',2,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',2,ref(6),[A]),'incremental-function-benefit',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',2,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',2,ref(6),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',2,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',2,ref(7),[A]),'individualized-benefit-risk-assessment',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',2,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',2,ref(7),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',2,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',2,ref(8),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',2,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',2,ref(8),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',2,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',2,ref(8),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',2,ref(6),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',2,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',2,ref(8),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',2,ref(7),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should compare a previous-increase-benefit against a previous-increase-risk and should evaluate an alternative-treatment and should evaluate a treatment-effectiveness.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec04-imp09',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',3,ref(2),[A]),'previous-increase-benefit',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',3,ref(3),[A]),'previous-increase-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',3,ref(4),[A]),compare) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',3,ref(4),[A]),against,'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec04-imp09',3,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',3,ref(5),[A]),'alternative-treatment',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',3,ref(5),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',3,ref(6),[A]),evaluate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',3,ref(6),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',3,ref(6),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',3,ref(5),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec04-imp09',3,box(3),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',3,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',3,ref(7),[A]),'treatment-effectiveness',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',3,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',3,ref(7),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',3,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',3,ref(8),[A]),evaluate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',3,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',3,ref(8),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',3,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',3,ref(8),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',3,ref(7),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician should consider a patient-value and should consider a patient-preference.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec04-imp09',4,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',4,ref(2),[A]),'patient-value',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',4,ref(3),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',4,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',4,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec04-imp09',4,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',4,ref(4),[A]),'patient-preference',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',4,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',4,ref(5),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',4,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp09',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',4,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec04-imp09',4,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
