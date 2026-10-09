% cdc2022-opioid-s45-01.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s45-01',ace_sha256(cb7dea4ac8df517861f7bd4acb25fb455c5dd5f25e747e2d1c96034115e1caea),ulex(sha256(be24cf56c59049d3c31fba641013faa22821ecda0669443a1fc4a49979ef57ae)),temporal(sha256('29a90281976bb48dfa32d9ef80651076896404c524c9bf1d7a55b06c1880be2a'))).
% S1: Every DAST is a validated-screening-tool.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s45-01',1,ref(2),[A]),'validated-screening-tool',countable) :- guideline_entity(actual,A,'DAST',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s45-01',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'DAST',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s45-01',1,ref(3),[A]),be) :- guideline_entity(actual,A,'DAST',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s45-01',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'DAST',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s45-01',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s45-01',1,ref(2),[A])) :- guideline_entity(actual,A,'DAST',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every TAPS is a validated-screening-tool.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s45-01',2,ref(2),[A]),'validated-screening-tool',countable) :- guideline_entity(actual,A,'TAPS',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s45-01',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'TAPS',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s45-01',2,ref(3),[A]),be) :- guideline_entity(actual,A,'TAPS',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s45-01',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'TAPS',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s45-01',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s45-01',2,ref(2),[A])) :- guideline_entity(actual,A,'TAPS',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every AUDIT-C is a validated-screening-tool.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s45-01',3,ref(2),[A]),'validated-screening-tool',countable) :- guideline_entity(actual,A,'AUDIT-C',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s45-01',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'AUDIT-C',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s45-01',3,ref(3),[A]),be) :- guideline_entity(actual,A,'AUDIT-C',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s45-01',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,'AUDIT-C',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s45-01',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s45-01',3,ref(2),[A])) :- guideline_entity(actual,A,'AUDIT-C',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician may use a validated-screening-tool.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s45-01',4,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s45-01',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s45-01',4,ref(2),[A]),'validated-screening-tool',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s45-01',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s45-01',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s45-01',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s45-01',4,ref(3),[A]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s45-01',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s45-01',4,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s45-01',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s45-01',4,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s45-01',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
