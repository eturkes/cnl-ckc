% cdc2022-opioid-s29-02.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s29-02',ace_sha256('9fd8f425698151073ad799f3b105e893546d4e98d32d7bad0e3b535fdbb156ee'),ulex(sha256(de64e41d6ddfdcb89810d9fc156aa0ac54562519e3bd4b94b8f7324f773ceb91)),vocabulary(sha256('06621a3dca592ca4385262a4a36dd8184670cdaef31827cd8b224c8c870c2f7d'))).
% S1: Every clinician should review a care-access-consideration before an opioid-initiation.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s29-02',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',1,ref(2),[A]),'care-access-consideration',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',1,ref(3),[A]),'opioid-initiation',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s29-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',1,ref(4),[A]),review) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s29-02',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s29-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',1,ref(4),[A]),before,'$guideline_id'(product,'cdc2022-opioid-s29-02',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_order('$guideline_id'(context,'cdc2022-opioid-s29-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',1,ref(4),[A]),before,'$guideline_id'(product,'cdc2022-opioid-s29-02',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should review a care-access-consideration before an opioid-continuation.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s29-02',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',2,ref(2),[A]),'care-access-consideration',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',2,ref(3),[A]),'opioid-continuation',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s29-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',2,ref(4),[A]),review) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s29-02',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s29-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',2,ref(4),[A]),before,'$guideline_id'(product,'cdc2022-opioid-s29-02',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_order('$guideline_id'(context,'cdc2022-opioid-s29-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',2,ref(4),[A]),before,'$guideline_id'(product,'cdc2022-opioid-s29-02',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should review a care-access-consideration before a pain-treatment-initiation.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s29-02',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',3,ref(2),[A]),'care-access-consideration',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',3,ref(3),[A]),'pain-treatment-initiation',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s29-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',3,ref(4),[A]),review) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s29-02',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s29-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',3,ref(4),[A]),before,'$guideline_id'(product,'cdc2022-opioid-s29-02',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_order('$guideline_id'(context,'cdc2022-opioid-s29-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',3,ref(4),[A]),before,'$guideline_id'(product,'cdc2022-opioid-s29-02',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician should review a care-access-consideration before a pain-treatment-continuation.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s29-02',4,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',4,ref(2),[A]),'care-access-consideration',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',4,ref(3),[A]),'pain-treatment-continuation',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s29-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',4,ref(4),[A]),review) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s29-02',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s29-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',4,ref(4),[A]),before,'$guideline_id'(product,'cdc2022-opioid-s29-02',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_order('$guideline_id'(context,'cdc2022-opioid-s29-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',4,ref(4),[A]),before,'$guideline_id'(product,'cdc2022-opioid-s29-02',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every opioid-initiation needs a clinical-oversight.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s29-02',5,ref(2),[A]),'clinical-oversight',countable) :- guideline_entity(actual,A,'opioid-initiation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s29-02',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-initiation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s29-02',5,ref(3),[A]),need) :- guideline_entity(actual,A,'opioid-initiation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s29-02',5,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-initiation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s29-02',5,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s29-02',5,ref(2),[A])) :- guideline_entity(actual,A,'opioid-initiation',countable), guideline_cardinality(actual,A,na,eq,1).
% S6: Every opioid-continuation needs a clinical-oversight.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s29-02',6,ref(2),[A]),'clinical-oversight',countable) :- guideline_entity(actual,A,'opioid-continuation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s29-02',6,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-continuation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s29-02',6,ref(3),[A]),need) :- guideline_entity(actual,A,'opioid-continuation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s29-02',6,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-continuation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s29-02',6,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s29-02',6,ref(2),[A])) :- guideline_entity(actual,A,'opioid-continuation',countable), guideline_cardinality(actual,A,na,eq,1).
% S7: Every pain-treatment-initiation needs a clinical-oversight.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s29-02',7,ref(2),[A]),'clinical-oversight',countable) :- guideline_entity(actual,A,'pain-treatment-initiation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s29-02',7,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'pain-treatment-initiation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s29-02',7,ref(3),[A]),need) :- guideline_entity(actual,A,'pain-treatment-initiation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s29-02',7,ref(3),[A]),1,A) :- guideline_entity(actual,A,'pain-treatment-initiation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s29-02',7,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s29-02',7,ref(2),[A])) :- guideline_entity(actual,A,'pain-treatment-initiation',countable), guideline_cardinality(actual,A,na,eq,1).
% S8: Every pain-treatment-continuation needs a clinical-oversight.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s29-02',8,ref(2),[A]),'clinical-oversight',countable) :- guideline_entity(actual,A,'pain-treatment-continuation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s29-02',8,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'pain-treatment-continuation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s29-02',8,ref(3),[A]),need) :- guideline_entity(actual,A,'pain-treatment-continuation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s29-02',8,ref(3),[A]),1,A) :- guideline_entity(actual,A,'pain-treatment-continuation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s29-02',8,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s29-02',8,ref(2),[A])) :- guideline_entity(actual,A,'pain-treatment-continuation',countable), guideline_cardinality(actual,A,na,eq,1).
