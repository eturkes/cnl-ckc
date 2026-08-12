% cdc2022-opioid-rec05-imp14.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
:- multifile(guideline_schema_version/1).
:- discontiguous(guideline_schema_version/1).
:- multifile(guideline_document/3).
:- discontiguous(guideline_document/3).
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
guideline_schema_version(1).
guideline_document('cdc2022-opioid-rec05-imp14',ace_sha256(d165b315cf8de35816887c2f907f033e9cef9d6994ed0a1c7f6429ffe88cf7b8),ulex(sha256('9d39595833bb1358c343511806091b34e2442561f073b432f531f68ac385a15e'))).
% S1: A clinically-significant-withdrawal-symptom can signal a further-taper-slowing-need.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp14',1,ref(1),[]),'clinically-significant-withdrawal-symptom',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp14',1,ref(1),[]),na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp14',1,box(1),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp14',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp14',1,ref(2),[]),'further-taper-slowing-need',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp14',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp14',1,ref(2),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp14',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp14',1,ref(3),[]),signal).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp14',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp14',1,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp14',1,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp14',1,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp14',1,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp14',1,ref(2),[])).
