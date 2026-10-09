% cdc2022-opioid-rec05-imp14.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec05-imp14',ace_sha256(d165b315cf8de35816887c2f907f033e9cef9d6994ed0a1c7f6429ffe88cf7b8),ulex(sha256(be24cf56c59049d3c31fba641013faa22821ecda0669443a1fc4a49979ef57ae)),temporal(sha256('29a90281976bb48dfa32d9ef80651076896404c524c9bf1d7a55b06c1880be2a'))).
% S1: A clinically-significant-withdrawal-symptom can signal a further-taper-slowing-need.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp14',1,ref(1),[]),'clinically-significant-withdrawal-symptom',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp14',1,ref(1),[]),na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp14',1,box(1),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp14',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp14',1,ref(2),[]),'further-taper-slowing-need',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp14',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp14',1,ref(2),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp14',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp14',1,ref(3),[]),signal).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp14',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp14',1,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp14',1,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp14',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp14',1,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp14',1,ref(2),[])).
