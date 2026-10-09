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
guideline_schema_version(3).
guideline_document('cdc2022-opioid-rec05-imp09',ace_sha256('58999ae126bc8e2d7c6704b75a63f46ec987bd8aa9055845f07117aa20a7ac01'),ulex(sha256(be24cf56c59049d3c31fba641013faa22821ecda0669443a1fc4a49979ef57ae)),temporal(sha256('29a90281976bb48dfa32d9ef80651076896404c524c9bf1d7a55b06c1880be2a'))).
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
