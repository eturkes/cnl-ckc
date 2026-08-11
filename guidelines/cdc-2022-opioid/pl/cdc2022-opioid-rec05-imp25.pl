% cdc2022-opioid-rec05-imp25.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec05-imp25',ace_sha256(d6c8d2783f52c9f41e3ae5955e599bdd1d44b070842f6939ffc439d8395e782a),ulex(sha256(bac3441c3cba94d1d496ba3b04390eae38a0826cd30a0a0f713112b2172476ce))).
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
% S2: Every opioid-policy-setter should ensure a taper-protective-threshold-policy.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(2),[A]),'taper-protective-threshold-policy',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(3),[A]),ensure) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',2,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every opioid-policy-setter should not penalize a clinician during a high-dosage-patient-acceptance.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(1),[A]),-) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),should) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(2),[A]),clinician,countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(3),[A]),'high-dosage-patient-acceptance',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(4),[A]),penalize) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',3,ref(3),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every opioid-policy-setter should not penalize a clinician during a long-term-opioid-taper-restraint.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(1),[A]),-) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),should) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(2),[A]),clinician,countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(3),[A]),'long-term-opioid-taper-restraint',countable) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(4),[A]),penalize) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(2),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp25',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec05-imp25',4,ref(3),[A])) :- guideline_entity(actual,A,'opioid-policy-setter',countable), guideline_cardinality(actual,A,na,eq,1).
