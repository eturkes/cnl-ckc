% cdc2022-opioid-s57-07.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_schema_version(1).
guideline_document('cdc2022-opioid-s57-07',ace_sha256('5ab015f779f3acc6b61f878185a60ff8ea67fa9abd38817fd024d7ca577487a7'),ulex(sha256('45608095e3f77853fdabcce148a9eb7633a8ae776353b8f008c753b846886588')),vocabulary(sha256(c36289aa40f75805a83501ca057b29dd1ba7292b025716853f654890b77ec50c))).
% S1: If a patient meets at least 2 opioid-use-disorder-criteria and meets at most 3 opioid-use-disorder-criteria then the patient has a mild-opioid-use-disorder.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s57-07',1,ref(6),[A,B,C,D,E]),'mild-opioid-use-disorder',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder-criterion',countable), guideline_cardinality(actual,B,na,F,G), guideline_satisfies(F,G,geq,2), guideline_event(actual,C,meet), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,meet), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D), guideline_entity(actual,D,'opioid-use-disorder-criterion',countable), guideline_cardinality(actual,D,na,H,I), guideline_satisfies(H,I,leq,3).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s57-07',1,ref(6),[A,B,C,D,E]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder-criterion',countable), guideline_cardinality(actual,B,na,F,G), guideline_satisfies(F,G,geq,2), guideline_event(actual,C,meet), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,meet), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D), guideline_entity(actual,D,'opioid-use-disorder-criterion',countable), guideline_cardinality(actual,D,na,H,I), guideline_satisfies(H,I,leq,3).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s57-07',1,ref(7),[A,B,C,D,E]),have) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder-criterion',countable), guideline_cardinality(actual,B,na,F,G), guideline_satisfies(F,G,geq,2), guideline_event(actual,C,meet), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,meet), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D), guideline_entity(actual,D,'opioid-use-disorder-criterion',countable), guideline_cardinality(actual,D,na,H,I), guideline_satisfies(H,I,leq,3).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s57-07',1,ref(7),[A,B,C,D,E]),1,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder-criterion',countable), guideline_cardinality(actual,B,na,F,G), guideline_satisfies(F,G,geq,2), guideline_event(actual,C,meet), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,meet), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D), guideline_entity(actual,D,'opioid-use-disorder-criterion',countable), guideline_cardinality(actual,D,na,H,I), guideline_satisfies(H,I,leq,3).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s57-07',1,ref(7),[A,B,C,D,E]),2,'$guideline_id'(product,'cdc2022-opioid-s57-07',1,ref(6),[A,B,C,D,E])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder-criterion',countable), guideline_cardinality(actual,B,na,F,G), guideline_satisfies(F,G,geq,2), guideline_event(actual,C,meet), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,meet), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D), guideline_entity(actual,D,'opioid-use-disorder-criterion',countable), guideline_cardinality(actual,D,na,H,I), guideline_satisfies(H,I,leq,3).
% S2: If a patient meets at least 4 opioid-use-disorder-criteria and meets at most 5 opioid-use-disorder-criteria then the patient has a moderate-opioid-use-disorder.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s57-07',2,ref(6),[A,B,C,D,E]),'moderate-opioid-use-disorder',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder-criterion',countable), guideline_cardinality(actual,B,na,F,G), guideline_satisfies(F,G,geq,4), guideline_event(actual,C,meet), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,meet), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D), guideline_entity(actual,D,'opioid-use-disorder-criterion',countable), guideline_cardinality(actual,D,na,H,I), guideline_satisfies(H,I,leq,5).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s57-07',2,ref(6),[A,B,C,D,E]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder-criterion',countable), guideline_cardinality(actual,B,na,F,G), guideline_satisfies(F,G,geq,4), guideline_event(actual,C,meet), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,meet), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D), guideline_entity(actual,D,'opioid-use-disorder-criterion',countable), guideline_cardinality(actual,D,na,H,I), guideline_satisfies(H,I,leq,5).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s57-07',2,ref(7),[A,B,C,D,E]),have) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder-criterion',countable), guideline_cardinality(actual,B,na,F,G), guideline_satisfies(F,G,geq,4), guideline_event(actual,C,meet), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,meet), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D), guideline_entity(actual,D,'opioid-use-disorder-criterion',countable), guideline_cardinality(actual,D,na,H,I), guideline_satisfies(H,I,leq,5).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s57-07',2,ref(7),[A,B,C,D,E]),1,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder-criterion',countable), guideline_cardinality(actual,B,na,F,G), guideline_satisfies(F,G,geq,4), guideline_event(actual,C,meet), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,meet), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D), guideline_entity(actual,D,'opioid-use-disorder-criterion',countable), guideline_cardinality(actual,D,na,H,I), guideline_satisfies(H,I,leq,5).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s57-07',2,ref(7),[A,B,C,D,E]),2,'$guideline_id'(product,'cdc2022-opioid-s57-07',2,ref(6),[A,B,C,D,E])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder-criterion',countable), guideline_cardinality(actual,B,na,F,G), guideline_satisfies(F,G,geq,4), guideline_event(actual,C,meet), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,meet), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,D), guideline_entity(actual,D,'opioid-use-disorder-criterion',countable), guideline_cardinality(actual,D,na,H,I), guideline_satisfies(H,I,leq,5).
% S3: If a patient meets at least 6 opioid-use-disorder-criteria then the patient has a severe-opioid-use-disorder.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s57-07',3,ref(4),[A,B,C]),'severe-opioid-use-disorder',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder-criterion',countable), guideline_cardinality(actual,B,na,D,E), guideline_satisfies(D,E,geq,6), guideline_event(actual,C,meet), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s57-07',3,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder-criterion',countable), guideline_cardinality(actual,B,na,D,E), guideline_satisfies(D,E,geq,6), guideline_event(actual,C,meet), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s57-07',3,ref(5),[A,B,C]),have) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder-criterion',countable), guideline_cardinality(actual,B,na,D,E), guideline_satisfies(D,E,geq,6), guideline_event(actual,C,meet), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s57-07',3,ref(5),[A,B,C]),1,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder-criterion',countable), guideline_cardinality(actual,B,na,D,E), guideline_satisfies(D,E,geq,6), guideline_event(actual,C,meet), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s57-07',3,ref(5),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s57-07',3,ref(4),[A,B,C])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-use-disorder-criterion',countable), guideline_cardinality(actual,B,na,D,E), guideline_satisfies(D,E,geq,6), guideline_event(actual,C,meet), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
