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
:- multifile(guideline_unit/3).
:- discontiguous(guideline_unit/3).
:- multifile(guideline_exclusion/1).
:- discontiguous(guideline_exclusion/1).
:- multifile(guideline_supersession/4).
:- discontiguous(guideline_supersession/4).
:- multifile(guideline_evaluation_date/1).
:- discontiguous(guideline_evaluation_date/1).
guideline_schema_version(1).
guideline_document('cdc2022-opioid-rec05-imp25',ace_sha256('3edd35994ca8abfb5998de531993d5e05e42a9d7a65e18c742af975c10bc6f44'),ulex(sha256(de64e41d6ddfdcb89810d9fc156aa0ac54562519e3bd4b94b8f7324f773ceb91)),vocabulary(sha256('06621a3dca592ca4385262a4a36dd8184670cdaef31827cd8b224c8c870c2f7d'))).
% S1: If there is an opioid-policy-setter then it is recommended that the opioid-policy-setter does not impose a rigid-dose-standard.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(2),[A]),-) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(2),[A]),'rigid-dose-standard',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(3),[A]),impose) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',1,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: If there is an opioid-policy-setter then it is recommended that the opioid-policy-setter does not impose a rigid-duration-standard.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(2),[A]),-) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(2),[A]),'rigid-duration-standard',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(3),[A]),impose) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: If there is an opioid-policy-setter then it is recommended that the opioid-policy-setter does not impose a performance-incentive.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),-) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(2),[A]),'performance-incentive',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(3),[A]),impose) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every opioid-policy-setter should ensure a cautionary-threshold-policy that does not cause a rapid-opioid-taper.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(2),[A]),'cautionary-threshold-policy',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),-) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(3),[A]),'rapid-opioid-taper',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(4),[A]),cause) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(4),[A]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(3),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(5),[A]),ensure) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(5),[A]),1,A) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every opioid-policy-setter should ensure a cautionary-threshold-policy that does not cause an abrupt-opioid-discontinuation.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(2),[A]),'cautionary-threshold-policy',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(2),[A]),-) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(3),[A]),'abrupt-opioid-discontinuation',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(4),[A]),cause) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(4),[A]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(3),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(5),[A]),ensure) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(5),[A]),1,A) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',5,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
% S6: If there is an opioid-policy-setter then it is recommended that the opioid-policy-setter does not penalize a clinician during a high-dosage-patient-acceptance.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',6,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',6,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',6,box(2),[A]),-) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',6,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',6,ref(2),[A]),clinician,countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',6,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',6,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',6,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',6,ref(3),[A]),'high-dosage-patient-acceptance',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',6,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',6,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',6,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',6,ref(4),[A]),penalize) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',6,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',6,ref(4),[A]),1,A) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',6,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',6,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',6,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',6,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',6,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',6,ref(3),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
% S7: If there is an opioid-policy-setter then it is recommended that the opioid-policy-setter does not penalize a clinician that does not rapidly taper a long-term-opioid-medication for a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(2),[A]),-) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(2),[A]),clinician,countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(2),[A]),'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(3),[A]),-) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(3),[A]),'long-term-opioid-medication',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(4),[A]),patient,countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(5),[A]),manner,rapidly) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(5),[A]),taper) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(5),[A]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(3),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(5),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(4),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(6),[A]),penalize) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(6),[A]),1,A) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',7,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(6),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',7,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
