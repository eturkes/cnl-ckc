% cdc2022-opioid-rec05-imp27.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_schema_version(1).
guideline_document('cdc2022-opioid-rec05-imp27',ace_sha256('11fb29ba1b6b8e81d58698c7e85eaa59c7ea82c277a5e1020ce554abd8651a4e'),ulex(sha256(ebe2d3db8559958f9ed44ec7a76cb2fb6ea75017f9f9437c3ea03b07092b0e75)),vocabulary(sha256('9d7632637554f8cbfe72e7d34a9b9d14c29eb8593aebb1c128a870c5dcf77ead'))).
% S1: If a patient takes an opioid during at least 1 year then a ten-percent-per-month-or-slower-taper can likely outperform a rapid-taper during a tolerability-assessment.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp27',1,ref(5),[A,B,C,D]),'ten-percent-per-month-or-slower-taper',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_cardinality(actual,C,na,E,F), guideline_unit(actual,C,G), guideline_satisfies(E,F,G,geq,1,year), guideline_event(actual,D,take), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,during,C), guideline_interval(actual,D,duration,C,G,none).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp27',1,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_cardinality(actual,C,na,E,F), guideline_unit(actual,C,G), guideline_satisfies(E,F,G,geq,1,year), guideline_event(actual,D,take), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,during,C), guideline_interval(actual,D,duration,C,G,none).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp27',1,box(1),[A,B,C,D]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_cardinality(actual,C,na,E,F), guideline_unit(actual,C,G), guideline_satisfies(E,F,G,geq,1,year), guideline_event(actual,D,take), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,during,C), guideline_interval(actual,D,duration,C,G,none).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp27',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp27',1,ref(6),[A,B,C,D]),'rapid-taper',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_cardinality(actual,C,na,E,F), guideline_unit(actual,C,G), guideline_satisfies(E,F,G,geq,1,year), guideline_event(actual,D,take), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,during,C), guideline_interval(actual,D,duration,C,G,none).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp27',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp27',1,ref(6),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_cardinality(actual,C,na,E,F), guideline_unit(actual,C,G), guideline_satisfies(E,F,G,geq,1,year), guideline_event(actual,D,take), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,during,C), guideline_interval(actual,D,duration,C,G,none).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp27',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp27',1,ref(7),[A,B,C,D]),'tolerability-assessment',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_cardinality(actual,C,na,E,F), guideline_unit(actual,C,G), guideline_satisfies(E,F,G,geq,1,year), guideline_event(actual,D,take), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,during,C), guideline_interval(actual,D,duration,C,G,none).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp27',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp27',1,ref(7),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_cardinality(actual,C,na,E,F), guideline_unit(actual,C,G), guideline_satisfies(E,F,G,geq,1,year), guideline_event(actual,D,take), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,during,C), guideline_interval(actual,D,duration,C,G,none).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-rec05-imp27',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp27',1,ref(8),[A,B,C,D]),possibility,likely) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_cardinality(actual,C,na,E,F), guideline_unit(actual,C,G), guideline_satisfies(E,F,G,geq,1,year), guideline_event(actual,D,take), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,during,C), guideline_interval(actual,D,duration,C,G,none).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp27',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp27',1,ref(8),[A,B,C,D]),outperform) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_cardinality(actual,C,na,E,F), guideline_unit(actual,C,G), guideline_satisfies(E,F,G,geq,1,year), guideline_event(actual,D,take), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,during,C), guideline_interval(actual,D,duration,C,G,none).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp27',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp27',1,ref(8),[A,B,C,D]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp27',1,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_cardinality(actual,C,na,E,F), guideline_unit(actual,C,G), guideline_satisfies(E,F,G,geq,1,year), guideline_event(actual,D,take), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,during,C), guideline_interval(actual,D,duration,C,G,none).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp27',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp27',1,ref(8),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp27',1,ref(6),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_cardinality(actual,C,na,E,F), guideline_unit(actual,C,G), guideline_satisfies(E,F,G,geq,1,year), guideline_event(actual,D,take), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,during,C), guideline_interval(actual,D,duration,C,G,none).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp27',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp27',1,ref(8),[A,B,C,D]),during,'$guideline_id'(product,'cdc2022-opioid-rec05-imp27',1,ref(7),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_cardinality(actual,C,na,E,F), guideline_unit(actual,C,G), guideline_satisfies(E,F,G,geq,1,year), guideline_event(actual,D,take), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,during,C), guideline_interval(actual,D,duration,C,G,none).
