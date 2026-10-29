% cdc2022-opioid-rec02-imp10.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec02-imp10',ace_sha256('3351560ff74a4cea709f578efbfffba98826954e5dbc28d76f9ad38b405225dd'),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: Every clinician may consider a tricyclic-antidepressant for a neuropathic-pain.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp10',1,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',1,ref(2),[A]),'tricyclic-antidepressant',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',1,ref(3),[A]),'neuropathic-pain',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',1,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',1,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician may consider a tetracyclic-antidepressant for a neuropathic-pain.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp10',2,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',2,ref(2),[A]),'tetracyclic-antidepressant',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',2,ref(3),[A]),'neuropathic-pain',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',2,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',2,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician may consider an SNRI-antidepressant for a neuropathic-pain.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp10',3,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',3,ref(2),[A]),'SNRI-antidepressant',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',3,ref(3),[A]),'neuropathic-pain',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',3,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',3,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician may consider a selected-anticonvulsant for a neuropathic-pain.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp10',4,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',4,ref(2),[A]),'selected-anticonvulsant',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',4,ref(3),[A]),'neuropathic-pain',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',4,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',4,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every clinician may consider a capsaicin-patch for a neuropathic-pain.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp10',5,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',5,ref(2),[A]),'capsaicin-patch',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',5,ref(3),[A]),'neuropathic-pain',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',5,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',5,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',5,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',5,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',5,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',5,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',5,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S6: Every clinician may consider a lidocaine-patch for a neuropathic-pain.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp10',6,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',6,ref(2),[A]),'lidocaine-patch',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',6,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',6,ref(3),[A]),'neuropathic-pain',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',6,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',6,ref(4),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',6,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',6,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',6,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec02-imp10',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',6,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',6,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S7: A selected-anticonvulsant-example includes a pregabalin-example and includes a gabapentin-enacarbil-example and includes an oxcarbazepine-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',7,ref(1),[]),'selected-anticonvulsant-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',7,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',7,ref(2),[]),'pregabalin-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',7,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',7,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',7,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',7,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',7,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',7,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',7,ref(4),[]),'gabapentin-enacarbil-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',7,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',7,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',7,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',7,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',7,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',7,ref(4),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',7,ref(6),[]),'oxcarbazepine-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',7,ref(6),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',7,ref(7),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',7,ref(7),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',7,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',7,ref(7),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp10',7,ref(6),[])).
