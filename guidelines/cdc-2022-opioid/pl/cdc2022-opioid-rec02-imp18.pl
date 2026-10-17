% cdc2022-opioid-rec02-imp18.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_schema_version(1).
guideline_document('cdc2022-opioid-rec02-imp18',ace_sha256('799b095f9dde8f82d5f60b71d6c3e2d41ef0ac12b39a1d401dd9d1f9d5eb4226'),ulex(sha256(d08a8bd3e80ef6e01301a079bb5496315f35233bf199aa89b2d2ccc93b858776)),vocabulary(sha256('9d7632637554f8cbfe72e7d34a9b9d14c29eb8593aebb1c128a870c5dcf77ead'))).
% S1: If a clinician does not consider an exit-strategy with a patient then it is recommended that the clinician does not initiate an opioid-therapy.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp18',1,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,B,-), guideline_entity(B,C,'exit-strategy',countable), guideline_cardinality(B,C,na,eq,1), guideline_entity(B,D,patient,countable), guideline_cardinality(B,D,na,eq,1), guideline_event(B,E,consider), guideline_arg(B,E,1,A), guideline_arg(B,E,2,C), guideline_pp(B,E,with,D).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec02-imp18',1,box(2),[A]),'$guideline_id'(context,'cdc2022-opioid-rec02-imp18',1,box(3),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,B,-), guideline_entity(B,C,'exit-strategy',countable), guideline_cardinality(B,C,na,eq,1), guideline_entity(B,D,patient,countable), guideline_cardinality(B,D,na,eq,1), guideline_event(B,E,consider), guideline_arg(B,E,1,A), guideline_arg(B,E,2,C), guideline_pp(B,E,with,D).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp18',1,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp18',1,ref(5),[A]),'opioid-therapy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,B,-), guideline_entity(B,C,'exit-strategy',countable), guideline_cardinality(B,C,na,eq,1), guideline_entity(B,D,patient,countable), guideline_cardinality(B,D,na,eq,1), guideline_event(B,E,consider), guideline_arg(B,E,1,A), guideline_arg(B,E,2,C), guideline_pp(B,E,with,D).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp18',1,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp18',1,ref(5),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,B,-), guideline_entity(B,C,'exit-strategy',countable), guideline_cardinality(B,C,na,eq,1), guideline_entity(B,D,patient,countable), guideline_cardinality(B,D,na,eq,1), guideline_event(B,E,consider), guideline_arg(B,E,1,A), guideline_arg(B,E,2,C), guideline_pp(B,E,with,D).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp18',1,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp18',1,ref(6),[A]),initiate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,B,-), guideline_entity(B,C,'exit-strategy',countable), guideline_cardinality(B,C,na,eq,1), guideline_entity(B,D,patient,countable), guideline_cardinality(B,D,na,eq,1), guideline_event(B,E,consider), guideline_arg(B,E,1,A), guideline_arg(B,E,2,C), guideline_pp(B,E,with,D).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp18',1,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp18',1,ref(6),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,B,-), guideline_entity(B,C,'exit-strategy',countable), guideline_cardinality(B,C,na,eq,1), guideline_entity(B,D,patient,countable), guideline_cardinality(B,D,na,eq,1), guideline_event(B,E,consider), guideline_arg(B,E,1,A), guideline_arg(B,E,2,C), guideline_pp(B,E,with,D).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp18',1,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp18',1,ref(6),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp18',1,ref(5),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,B,-), guideline_entity(B,C,'exit-strategy',countable), guideline_cardinality(B,C,na,eq,1), guideline_entity(B,D,patient,countable), guideline_cardinality(B,D,na,eq,1), guideline_event(B,E,consider), guideline_arg(B,E,1,A), guideline_arg(B,E,2,C), guideline_pp(B,E,with,D).
% S2: If an opioid-therapy is unsuccessful then every clinician should use an exit-strategy with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp18',2,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,'opioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,unsuccessful,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp18',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp18',2,ref(5),[A,B,C,D]),'exit-strategy',countable) :- guideline_entity(actual,A,'opioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,unsuccessful,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp18',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp18',2,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,'opioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,unsuccessful,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp18',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp18',2,ref(6),[A,B,C,D]),patient,countable) :- guideline_entity(actual,A,'opioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,unsuccessful,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp18',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp18',2,ref(6),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,'opioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,unsuccessful,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp18',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp18',2,ref(7),[A,B,C,D]),use) :- guideline_entity(actual,A,'opioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,unsuccessful,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp18',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp18',2,ref(7),[A,B,C,D]),1,D) :- guideline_entity(actual,A,'opioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,unsuccessful,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp18',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp18',2,ref(7),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp18',2,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,'opioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,unsuccessful,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec02-imp18',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp18',2,ref(7),[A,B,C,D]),with,'$guideline_id'(product,'cdc2022-opioid-rec02-imp18',2,ref(6),[A,B,C,D])) :- guideline_entity(actual,A,'opioid-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,unsuccessful,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
