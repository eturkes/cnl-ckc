% cdc2022-opioid-rec05-imp25.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec05-imp25',ace_sha256(ff617f79bf78d28852ecab85f2ca816a26976e0e54f3a52ad9c84363c0ab36ba),ulex(sha256('67d1410100ff45477e758b366ae03bb860a5c23e3fd444d55dc901b2f3c7d8a1')),vocabulary(sha256('9d7632637554f8cbfe72e7d34a9b9d14c29eb8593aebb1c128a870c5dcf77ead'))).
% S1: Every opioid-policy-setter should not impose a rigid-dose-standard and should not impose a rigid-duration-standard and should not impose a performance-incentive.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(1),[A]),-) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(2),[A]),should) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(2),[A]),'rigid-dose-standard',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(3),[A]),impose) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(3),[A]),-) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(3),[A]),'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(4),[A]),should) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(4),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(4),[A]),'rigid-duration-standard',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(4),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(4),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(5),[A]),impose) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(4),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(5),[A]),1,A) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(4),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(4),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(5),[A]),-) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(5),[A]),'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(6),[A]),should) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(6),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(6),[A]),'performance-incentive',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(6),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(6),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(6),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(7),[A]),impose) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(6),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(7),[A]),1,A) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(6),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(7),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(6),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every opioid-policy-setter should ensure a cautionary-threshold-policy that does not cause a rapid-opioid-taper.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(2),[A]),'cautionary-threshold-policy',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(2),[A]),-) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(3),[A]),'rapid-opioid-taper',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(4),[A]),cause) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(4),[A]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(3),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(5),[A]),ensure) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(5),[A]),1,A) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every opioid-policy-setter should ensure a cautionary-threshold-policy that does not cause an abrupt-opioid-discontinuation.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(2),[A]),'cautionary-threshold-policy',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),-) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(3),[A]),'abrupt-opioid-discontinuation',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(4),[A]),cause) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(4),[A]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(3),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(5),[A]),ensure) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(5),[A]),1,A) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every opioid-policy-setter should not penalize a clinician during a high-dosage-patient-acceptance.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(1),[A]),-) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),should) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(2),[A]),clinician,countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(3),[A]),'high-dosage-patient-acceptance',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(4),[A]),penalize) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(3),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every opioid-policy-setter should not penalize a clinician that does not rapidly taper a long-term-opioid-medication for a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(1),[A]),-) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(2),[A]),should) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(2),[A]),clinician,countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(2),[A]),'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(3),[A]),-) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(3),[A]),'long-term-opioid-medication',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(4),[A]),patient,countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(5),[A]),manner,rapidly) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(5),[A]),taper) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(5),[A]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(3),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(5),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(4),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(6),[A]),penalize) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(6),[A]),1,A) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(6),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
