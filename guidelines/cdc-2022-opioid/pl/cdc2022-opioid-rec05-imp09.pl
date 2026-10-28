% cdc2022-opioid-rec05-imp09.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec05-imp09',ace_sha256(baf8e28da951a3a219e6716029f253eeaf594343062ce3e34b9a55df6f3da0f3),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: Every taper-team-member can support a clinician during an opioid-taper through a telephone-contact.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp09',1,box(1),[A]),can) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',1,ref(2),[A]),clinician,countable) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',1,ref(3),[A]),'opioid-taper',countable) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',1,ref(4),[A]),'telephone-contact',countable) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',1,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',1,ref(5),[A]),support) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',1,ref(5),[A]),1,A) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',1,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',1,ref(2),[A])) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',1,ref(5),[A]),through,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',1,ref(4),[A])) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',1,ref(5),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',1,ref(3),[A])) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every taper-team-member can support a patient during an opioid-taper through a telephone-contact.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp09',2,box(1),[A]),can) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',2,ref(2),[A]),patient,countable) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',2,ref(3),[A]),'opioid-taper',countable) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',2,ref(4),[A]),'telephone-contact',countable) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',2,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',2,ref(5),[A]),support) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',2,ref(5),[A]),1,A) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',2,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',2,ref(2),[A])) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',2,ref(5),[A]),through,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',2,ref(4),[A])) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',2,ref(5),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',2,ref(3),[A])) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every taper-team-member can support a clinician during an opioid-taper through a telehealth-visit.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp09',3,box(1),[A]),can) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',3,ref(2),[A]),clinician,countable) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',3,ref(3),[A]),'opioid-taper',countable) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',3,ref(4),[A]),'telehealth-visit',countable) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',3,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',3,ref(5),[A]),support) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',3,ref(5),[A]),1,A) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',3,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',3,ref(2),[A])) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',3,ref(5),[A]),through,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',3,ref(4),[A])) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',3,ref(5),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',3,ref(3),[A])) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every taper-team-member can support a patient during an opioid-taper through a telehealth-visit.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp09',4,box(1),[A]),can) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',4,ref(2),[A]),patient,countable) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',4,ref(3),[A]),'opioid-taper',countable) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',4,ref(4),[A]),'telehealth-visit',countable) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',4,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',4,ref(5),[A]),support) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',4,ref(5),[A]),1,A) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',4,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',4,ref(2),[A])) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',4,ref(5),[A]),through,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',4,ref(4),[A])) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',4,ref(5),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',4,ref(3),[A])) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every taper-team-member can support a clinician during an opioid-taper through a face-to-face-visit.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp09',5,box(1),[A]),can) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',5,ref(2),[A]),clinician,countable) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',5,ref(3),[A]),'opioid-taper',countable) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',5,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',5,ref(4),[A]),'face-to-face-visit',countable) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',5,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',5,ref(5),[A]),support) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',5,ref(5),[A]),1,A) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',5,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',5,ref(2),[A])) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',5,ref(5),[A]),through,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',5,ref(4),[A])) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',5,ref(5),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',5,ref(3),[A])) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
% S6: Every taper-team-member can support a patient during an opioid-taper through a face-to-face-visit.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp09',6,box(1),[A]),can) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',6,ref(2),[A]),patient,countable) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',6,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',6,ref(3),[A]),'opioid-taper',countable) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',6,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',6,ref(4),[A]),'face-to-face-visit',countable) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',6,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',6,ref(5),[A]),support) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',6,ref(5),[A]),1,A) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',6,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',6,ref(2),[A])) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',6,ref(5),[A]),through,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',6,ref(4),[A])) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp09',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',6,ref(5),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',6,ref(3),[A])) :- guideline_entity(actual,A,'taper-team-member',countable), guideline_cardinality(actual,A,na,eq,1).
% S7: A taper-team-member-example includes a nurse-example and includes a pharmacist-example and includes a behavioral-health-professional-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',7,ref(1),[]),'taper-team-member-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',7,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',7,ref(2),[]),'nurse-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',7,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',7,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',7,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',7,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',7,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',7,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',7,ref(4),[]),'pharmacist-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',7,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',7,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',7,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',7,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',7,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',7,ref(4),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',7,ref(6),[]),'behavioral-health-professional-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',7,ref(6),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',7,ref(7),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',7,ref(7),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',7,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',7,ref(7),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp09',7,ref(6),[])).
