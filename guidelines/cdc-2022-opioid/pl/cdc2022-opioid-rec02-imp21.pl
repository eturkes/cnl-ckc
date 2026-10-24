% cdc2022-opioid-rec02-imp21.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec02-imp21',ace_sha256(ed168ce3713c99a5b57c8f3e49f705e76663881838f7cdc4246dd20df487f0a5),ulex(sha256('64c086862c2f48d491a9fde0ba9f0fcf090e25e84456b9749332ec3ace211ce4')),vocabulary(sha256(c36289aa40f75805a83501ca057b29dd1ba7292b025716853f654890b77ec50c))).
% S1: An opioid-continuation can possibly initiate a long-term-opioid-therapy after 30 days.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',1,ref(1),[]),'opioid-continuation',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',1,ref(1),[]),na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp21',1,box(1),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp21',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',1,ref(2),[]),'long-term-opioid-therapy',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp21',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',1,ref(2),[]),na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp21',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',1,ref(3),[]),day,countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp21',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',1,ref(3),[]),na,eq,30).
guideline_unit('$guideline_id'(context,'cdc2022-opioid-rec02-imp21',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',1,ref(3),[]),day).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-rec02-imp21',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',1,ref(4),[]),possibility,possibly).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp21',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',1,ref(4),[]),initiate).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp21',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',1,ref(4),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',1,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp21',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',1,ref(4),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',1,ref(2),[])).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec02-imp21',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',1,ref(4),[]),after,'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',1,ref(3),[])).
guideline_interval('$guideline_id'(context,'cdc2022-opioid-rec02-imp21',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',1,ref(4),[]),after,'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',1,ref(3),[]),day,none).
% S2: If a clinician does not provably make an intentional-benefit-risk-decision with a patient then it is recommended that the clinician does not initiate a long-term-opioid-therapy.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp21',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'intentional-benefit-risk-decision',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,make), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C)), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec02-imp21',2,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec02-imp21',2,box(2),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'intentional-benefit-risk-decision',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,make), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C)).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp21',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',2,ref(5),[A]),'long-term-opioid-therapy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'intentional-benefit-risk-decision',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,make), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C)).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp21',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',2,ref(5),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'intentional-benefit-risk-decision',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,make), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C)).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp21',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',2,ref(6),[A]),initiate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'intentional-benefit-risk-decision',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,make), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C)).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp21',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',2,ref(6),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'intentional-benefit-risk-decision',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,make), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C)).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp21',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',2,ref(6),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',2,ref(5),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'intentional-benefit-risk-decision',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,make), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C)).
% S3: If a clinician does not provably conduct an informed-discussion with a patient then it is recommended that the clinician does not initiate a long-term-opioid-therapy.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp21',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'informed-discussion',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,conduct), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C)), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec02-imp21',3,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec02-imp21',3,box(2),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'informed-discussion',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,conduct), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C)).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp21',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',3,ref(5),[A]),'long-term-opioid-therapy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'informed-discussion',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,conduct), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C)).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp21',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',3,ref(5),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'informed-discussion',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,conduct), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C)).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp21',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',3,ref(6),[A]),initiate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'informed-discussion',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,conduct), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C)).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp21',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',3,ref(6),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'informed-discussion',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,conduct), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C)).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp21',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',3,ref(6),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',3,ref(5),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'informed-discussion',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,conduct), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C)).
% S4: If a clinician does not provably use a comprehensive-pain-management then it is recommended that the clinician does not initiate a long-term-opioid-therapy.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp21',4,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'comprehensive-pain-management',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec02-imp21',4,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec02-imp21',4,box(2),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'comprehensive-pain-management',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp21',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',4,ref(4),[A]),'long-term-opioid-therapy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'comprehensive-pain-management',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp21',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',4,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'comprehensive-pain-management',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp21',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',4,ref(5),[A]),initiate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'comprehensive-pain-management',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp21',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',4,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'comprehensive-pain-management',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp21',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',4,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp21',4,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'comprehensive-pain-management',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,use), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
