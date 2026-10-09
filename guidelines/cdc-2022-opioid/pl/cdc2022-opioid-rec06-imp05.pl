% cdc2022-opioid-rec06-imp05.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec06-imp05',ace_sha256('568f5e22dae75bdcb70aefd60d16430e92c0bb5fb10fb4e19ff2f8e36bef5343'),ulex(sha256(be24cf56c59049d3c31fba641013faa22821ecda0669443a1fc4a49979ef57ae)),temporal(sha256('29a90281976bb48dfa32d9ef80651076896404c524c9bf1d7a55b06c1880be2a'))).
% S1: Every prolonged-severe-acute-pain-care-mechanism should enable a timely-diagnostic-reevaluation.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec06-imp05',1,box(1),[A]),should) :- guideline_entity(actual,A,'prolonged-severe-acute-pain-care-mechanism',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec06-imp05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec06-imp05',1,ref(2),[A]),'timely-diagnostic-reevaluation',countable) :- guideline_entity(actual,A,'prolonged-severe-acute-pain-care-mechanism',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec06-imp05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec06-imp05',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'prolonged-severe-acute-pain-care-mechanism',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec06-imp05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec06-imp05',1,ref(3),[A]),enable) :- guideline_entity(actual,A,'prolonged-severe-acute-pain-care-mechanism',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec06-imp05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec06-imp05',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'prolonged-severe-acute-pain-care-mechanism',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec06-imp05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec06-imp05',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec06-imp05',1,ref(2),[A])) :- guideline_entity(actual,A,'prolonged-severe-acute-pain-care-mechanism',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every timely-diagnostic-reevaluation may confirm an initial-diagnosis.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec06-imp05',2,box(1),[A]),may) :- guideline_entity(actual,A,'timely-diagnostic-reevaluation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec06-imp05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec06-imp05',2,ref(2),[A]),'initial-diagnosis',countable) :- guideline_entity(actual,A,'timely-diagnostic-reevaluation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec06-imp05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec06-imp05',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'timely-diagnostic-reevaluation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec06-imp05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec06-imp05',2,ref(3),[A]),confirm) :- guideline_entity(actual,A,'timely-diagnostic-reevaluation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec06-imp05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec06-imp05',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'timely-diagnostic-reevaluation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec06-imp05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec06-imp05',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec06-imp05',2,ref(2),[A])) :- guideline_entity(actual,A,'timely-diagnostic-reevaluation',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every timely-diagnostic-reevaluation may revise an initial-diagnosis.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec06-imp05',3,box(1),[A]),may) :- guideline_entity(actual,A,'timely-diagnostic-reevaluation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec06-imp05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec06-imp05',3,ref(2),[A]),'initial-diagnosis',countable) :- guideline_entity(actual,A,'timely-diagnostic-reevaluation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec06-imp05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec06-imp05',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'timely-diagnostic-reevaluation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec06-imp05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec06-imp05',3,ref(3),[A]),revise) :- guideline_entity(actual,A,'timely-diagnostic-reevaluation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec06-imp05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec06-imp05',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,'timely-diagnostic-reevaluation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec06-imp05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec06-imp05',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec06-imp05',3,ref(2),[A])) :- guideline_entity(actual,A,'timely-diagnostic-reevaluation',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every prolonged-severe-acute-pain-care-mechanism should enable a pain-management-adjustment.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec06-imp05',4,box(1),[A]),should) :- guideline_entity(actual,A,'prolonged-severe-acute-pain-care-mechanism',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec06-imp05',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec06-imp05',4,ref(2),[A]),'pain-management-adjustment',countable) :- guideline_entity(actual,A,'prolonged-severe-acute-pain-care-mechanism',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec06-imp05',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec06-imp05',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'prolonged-severe-acute-pain-care-mechanism',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec06-imp05',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec06-imp05',4,ref(3),[A]),enable) :- guideline_entity(actual,A,'prolonged-severe-acute-pain-care-mechanism',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec06-imp05',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec06-imp05',4,ref(3),[A]),1,A) :- guideline_entity(actual,A,'prolonged-severe-acute-pain-care-mechanism',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec06-imp05',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec06-imp05',4,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec06-imp05',4,ref(2),[A])) :- guideline_entity(actual,A,'prolonged-severe-acute-pain-care-mechanism',countable), guideline_cardinality(actual,A,na,eq,1).
