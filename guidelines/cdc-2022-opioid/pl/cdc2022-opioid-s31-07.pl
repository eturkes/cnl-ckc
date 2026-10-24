% cdc2022-opioid-s31-07.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s31-07',ace_sha256(ba09d94053a0d60bc645bc171d9dc11fb290053fdd63270e684ed32a56f1b8c0),ulex(sha256('45608095e3f77853fdabcce148a9eb7633a8ae776353b8f008c753b846886588')),vocabulary(sha256(c36289aa40f75805a83501ca057b29dd1ba7292b025716853f654890b77ec50c))).
% S1: Every methadone is an ER-LA-opioid.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',1,ref(2),[A]),'ER-LA-opioid',countable) :- guideline_entity(actual,A,methadone,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,methadone,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',1,ref(3),[A]),be) :- guideline_entity(actual,A,methadone,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,methadone,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s31-07',1,ref(2),[A])) :- guideline_entity(actual,A,methadone,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every transdermal-fentanyl is an ER-LA-opioid.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',2,ref(2),[A]),'ER-LA-opioid',countable) :- guideline_entity(actual,A,'transdermal-fentanyl',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'transdermal-fentanyl',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',2,ref(3),[A]),be) :- guideline_entity(actual,A,'transdermal-fentanyl',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'transdermal-fentanyl',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s31-07',2,ref(2),[A])) :- guideline_entity(actual,A,'transdermal-fentanyl',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every extended-release-opioid is an ER-LA-opioid.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',3,ref(2),[A]),'ER-LA-opioid',countable) :- guideline_entity(actual,A,'extended-release-opioid',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'extended-release-opioid',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',3,ref(3),[A]),be) :- guideline_entity(actual,A,'extended-release-opioid',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,'extended-release-opioid',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s31-07',3,ref(2),[A])) :- guideline_entity(actual,A,'extended-release-opioid',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: An extended-release-opioid-example includes an extended-release-oxycodone-example and includes an extended-release-hydromorphone-example and includes an extended-release-hydrocodone-example and includes an extended-release-morphine-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(1),[]),'extended-release-opioid-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(2),[]),'extended-release-oxycodone-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(4),[]),'extended-release-hydromorphone-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(4),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(6),[]),'extended-release-hydrocodone-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(6),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(7),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(7),[]),1,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(7),[]),2,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(6),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(8),[]),'extended-release-morphine-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(8),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(9),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(9),[]),1,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(9),[]),2,'$guideline_id'(product,'cdc2022-opioid-s31-07',4,ref(8),[])).
