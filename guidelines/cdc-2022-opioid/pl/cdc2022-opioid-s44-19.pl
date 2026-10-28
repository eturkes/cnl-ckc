% cdc2022-opioid-s44-19.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s44-19',ace_sha256(fe805d9fcf964fbad6beb1274d2c4d617a58cddc0027a343f13c3c33fb81454e),ulex(sha256(de64e41d6ddfdcb89810d9fc156aa0ac54562519e3bd4b94b8f7324f773ceb91)),vocabulary(sha256('06621a3dca592ca4385262a4a36dd8184670cdaef31827cd8b224c8c870c2f7d'))).
% S1: If a clinician evaluates a patient within 3 months of an opioid-initiation then the clinician can possibly have an opioid-use-disorder-and-overdose-risk-identification-and-mitigation-opportunity.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s44-19',1,box(1),[A,B,C,D,E]),can) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,month,countable), guideline_cardinality(actual,C,na,eq,3), guideline_unit(actual,C,month), guideline_entity(actual,D,'opioid-initiation',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,evaluate), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,B), guideline_pp(actual,E,within,C), guideline_interval(actual,E,within,C,month,D).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s44-19',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s44-19',1,ref(6),[A,B,C,D,E]),'opioid-use-disorder-and-overdose-risk-identification-and-mitigation-opportunity',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,month,countable), guideline_cardinality(actual,C,na,eq,3), guideline_unit(actual,C,month), guideline_entity(actual,D,'opioid-initiation',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,evaluate), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,B), guideline_pp(actual,E,within,C), guideline_interval(actual,E,within,C,month,D).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s44-19',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s44-19',1,ref(6),[A,B,C,D,E]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,month,countable), guideline_cardinality(actual,C,na,eq,3), guideline_unit(actual,C,month), guideline_entity(actual,D,'opioid-initiation',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,evaluate), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,B), guideline_pp(actual,E,within,C), guideline_interval(actual,E,within,C,month,D).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s44-19',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s44-19',1,ref(7),[A,B,C,D,E]),possibility,possibly) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,month,countable), guideline_cardinality(actual,C,na,eq,3), guideline_unit(actual,C,month), guideline_entity(actual,D,'opioid-initiation',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,evaluate), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,B), guideline_pp(actual,E,within,C), guideline_interval(actual,E,within,C,month,D).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s44-19',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s44-19',1,ref(7),[A,B,C,D,E]),have) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,month,countable), guideline_cardinality(actual,C,na,eq,3), guideline_unit(actual,C,month), guideline_entity(actual,D,'opioid-initiation',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,evaluate), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,B), guideline_pp(actual,E,within,C), guideline_interval(actual,E,within,C,month,D).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s44-19',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s44-19',1,ref(7),[A,B,C,D,E]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,month,countable), guideline_cardinality(actual,C,na,eq,3), guideline_unit(actual,C,month), guideline_entity(actual,D,'opioid-initiation',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,evaluate), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,B), guideline_pp(actual,E,within,C), guideline_interval(actual,E,within,C,month,D).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s44-19',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s44-19',1,ref(7),[A,B,C,D,E]),2,'$guideline_id'(product,'cdc2022-opioid-s44-19',1,ref(6),[A,B,C,D,E])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,month,countable), guideline_cardinality(actual,C,na,eq,3), guideline_unit(actual,C,month), guideline_entity(actual,D,'opioid-initiation',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,evaluate), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,B), guideline_pp(actual,E,within,C), guideline_interval(actual,E,within,C,month,D).
