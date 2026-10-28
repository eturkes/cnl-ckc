% cdc2022-opioid-s34-16.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s34-16',ace_sha256('8cb62711000ebbee0ac39772cfd9adddc17e8f2b3e07ded90782b1678d62ad22'),ulex(sha256('64c086862c2f48d491a9fde0ba9f0fcf090e25e84456b9749332ec3ace211ce4')),vocabulary(sha256('06621a3dca592ca4385262a4a36dd8184670cdaef31827cd8b224c8c870c2f7d'))).
% S1: Every cited-dose-response-overdose-risk-study examines a full-agonist-opioid-group.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s34-16',1,ref(2),[A]),'full-agonist-opioid-group',countable) :- guideline_entity(actual,A,'cited-dose-response-overdose-risk-study',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s34-16',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'cited-dose-response-overdose-risk-study',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s34-16',1,ref(3),[A]),examine) :- guideline_entity(actual,A,'cited-dose-response-overdose-risk-study',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s34-16',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'cited-dose-response-overdose-risk-study',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s34-16',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s34-16',1,ref(2),[A])) :- guideline_entity(actual,A,'cited-dose-response-overdose-risk-study',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every cited-dose-response-overdose-risk-study does not examine a partial-agonist-opioid-group.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s34-16',2,box(1),[A]),-) :- guideline_entity(actual,A,'cited-dose-response-overdose-risk-study',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s34-16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s34-16',2,ref(2),[A]),'partial-agonist-opioid-group',countable) :- guideline_entity(actual,A,'cited-dose-response-overdose-risk-study',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s34-16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s34-16',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'cited-dose-response-overdose-risk-study',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s34-16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s34-16',2,ref(3),[A]),examine) :- guideline_entity(actual,A,'cited-dose-response-overdose-risk-study',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s34-16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s34-16',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'cited-dose-response-overdose-risk-study',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s34-16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s34-16',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s34-16',2,ref(2),[A])) :- guideline_entity(actual,A,'cited-dose-response-overdose-risk-study',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every buprenorphine is a partial-agonist-opioid.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s34-16',3,ref(2),[A]),'partial-agonist-opioid',countable) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s34-16',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s34-16',3,ref(3),[A]),be) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s34-16',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s34-16',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s34-16',3,ref(2),[A])) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: If there is a buprenorphine then it is possible that the buprenorphine does not likely have a continuous-dosage-overdose-risk-association.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s34-16',4,box(1),[A]),can) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-s34-16',4,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-s34-16',4,box(2),[A]),-) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s34-16',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s34-16',4,ref(2),[A]),'continuous-dosage-overdose-risk-association',countable) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s34-16',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s34-16',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s34-16',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s34-16',4,ref(3),[A]),possibility,likely) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s34-16',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s34-16',4,ref(3),[A]),have) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s34-16',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s34-16',4,ref(3),[A]),1,A) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s34-16',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s34-16',4,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s34-16',4,ref(2),[A])) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every buprenorphine-respiratory-depressant-effect reaches a plateau.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s34-16',5,ref(2),[A]),plateau,countable) :- guideline_entity(actual,A,'buprenorphine-respiratory-depressant-effect',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s34-16',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'buprenorphine-respiratory-depressant-effect',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s34-16',5,ref(3),[A]),reach) :- guideline_entity(actual,A,'buprenorphine-respiratory-depressant-effect',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s34-16',5,ref(3),[A]),1,A) :- guideline_entity(actual,A,'buprenorphine-respiratory-depressant-effect',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s34-16',5,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s34-16',5,ref(2),[A])) :- guideline_entity(actual,A,'buprenorphine-respiratory-depressant-effect',countable), guideline_cardinality(actual,A,na,eq,1).
