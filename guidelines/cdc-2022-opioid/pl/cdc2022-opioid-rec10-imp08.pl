% cdc2022-opioid-rec10-imp08.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec10-imp08',ace_sha256('56bbc26ef5a6568f0f1ad500cab570cc6cb403cf6366fe1f5249ccc5c3ba8b71'),ulex(sha256(be24cf56c59049d3c31fba641013faa22821ecda0669443a1fc4a49979ef57ae)),temporal(sha256('29a90281976bb48dfa32d9ef80651076896404c524c9bf1d7a55b06c1880be2a'))).
% S1: Every clinician may use a relatively-inexpensive-presumptive-immunoassay-panel during a limited-toxicology-screening.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp08',1,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',1,ref(2),[A]),'relatively-inexpensive-presumptive-immunoassay-panel',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',1,ref(3),[A]),'limited-toxicology-screening',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',1,ref(4),[A]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec10-imp08',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',1,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: A relatively-inexpensive-presumptive-immunoassay-panel can assess an opiate-class and can assess a benzodiazepine and can assess a nonprescribed-substance.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',2,ref(1),[]),'relatively-inexpensive-presumptive-immunoassay-panel',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',2,ref(1),[]),na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp08',2,box(1),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp08',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',2,ref(2),[]),'opiate-class',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp08',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',2,ref(2),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp08',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',2,ref(3),[]),assess).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp08',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',2,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',2,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp08',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',2,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',2,ref(2),[])).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp08',2,box(2),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp08',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',2,ref(4),[]),benzodiazepine,countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp08',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',2,ref(4),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp08',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',2,ref(5),[]),assess).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp08',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',2,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',2,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp08',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',2,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',2,ref(4),[])).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp08',2,box(3),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp08',2,box(3),[]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',2,ref(6),[]),'nonprescribed-substance',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp08',2,box(3),[]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',2,ref(6),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp08',2,box(3),[]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',2,ref(7),[]),assess).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp08',2,box(3),[]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',2,ref(7),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',2,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp08',2,box(3),[]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',2,ref(7),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp08',2,ref(6),[])).
