% cdc2022-opioid-rec05-imp16.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec05-imp16',ace_sha256('3b1efaa19919178f01c5732d34615689b791ecac1f62a982f4a67c45ebc3cd5d'),ulex(sha256('0b669018d532f506c518a6b0edffa4994f771b930477e7e2b36faf50d2b75d33')),temporal(sha256(d7b0d6c9af49f8d4b3dc12370d6b1cc20e923a69a5ce745ee26a1bc0f21effb6))).
% S1: Every clinician should assess a dosage-increase-benefit with a patient before a taper-reversal and should assess a dosage-increase-risk with a patient before a taper-reversal.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp16',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(2),[A]),'dosage-increase-benefit',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(4),[A]),'taper-reversal',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(5),[A]),assess) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(5),[A]),before,'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_order('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(5),[A]),before,'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(5),[A]),with,'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp16',1,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(6),[A]),'dosage-increase-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(6),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(7),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(7),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(8),[A]),'taper-reversal',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(8),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(9),[A]),assess) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(9),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(9),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(6),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(9),[A]),before,'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(8),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_order('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(9),[A]),before,'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(8),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(9),[A]),with,'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',1,ref(7),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should discuss a dosage-increase-benefit with a patient before a taper-reversal and should discuss a dosage-increase-risk with a patient before a taper-reversal.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp16',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(2),[A]),'dosage-increase-benefit',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(4),[A]),'taper-reversal',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(5),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(5),[A]),before,'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_order('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(5),[A]),before,'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(5),[A]),with,'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp16',2,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(6),[A]),'dosage-increase-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(6),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(7),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(7),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(8),[A]),'taper-reversal',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(8),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(9),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(9),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(9),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(6),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(9),[A]),before,'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(8),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_order('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(9),[A]),before,'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(8),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp16',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(9),[A]),with,'$guideline_id'(product,'cdc2022-opioid-rec05-imp16',2,ref(7),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
