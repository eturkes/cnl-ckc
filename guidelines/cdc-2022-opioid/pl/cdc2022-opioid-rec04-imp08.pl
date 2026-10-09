% cdc2022-opioid-rec04-imp08.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec04-imp08',ace_sha256(ea4944bf60c52132b639f5a09af9ef3e34a7596219837ba1da8528456a383036),ulex(sha256(be24cf56c59049d3c31fba641013faa22821ecda0669443a1fc4a49979ef57ae)),temporal(sha256('29a90281976bb48dfa32d9ef80651076896404c524c9bf1d7a55b06c1880be2a'))).
% S1: If a clinician decides a dosage-increase then the clinician should heed a caution and should use a smallest-practical-increase.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(1),[A,B,C]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(4),[A,B,C]),caution,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(5),[A,B,C]),heed) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(5),[A,B,C]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(5),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(2),[A,B,C]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(6),[A,B,C]),'smallest-practical-increase',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(6),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(7),[A,B,C]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(7),[A,B,C]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec04-imp08',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(7),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-rec04-imp08',1,ref(6),[A,B,C])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,decide), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
