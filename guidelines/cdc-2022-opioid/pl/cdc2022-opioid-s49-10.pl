% cdc2022-opioid-s49-10.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s49-10',ace_sha256('131b8f4e6142593f9515444e5fce59153421e7c81ff982a323e1fa022d756731'),ulex(sha256(be24cf56c59049d3c31fba641013faa22821ecda0669443a1fc4a49979ef57ae)),temporal(sha256('29a90281976bb48dfa32d9ef80651076896404c524c9bf1d7a55b06c1880be2a'))).
% S1: If a clinician prescribes an opioid to a patient and a substance-use-disorder-treatment-provider provides a substance-use-disorder-treatment to the patient then the clinician should communicate with the substance-use-disorder-treatment-provider.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s49-10',1,box(1),[A,B,C,D,E,F,G]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,to,C), guideline_entity(actual,E,'substance-use-disorder-treatment-provider',countable), guideline_cardinality(actual,E,na,eq,1), guideline_entity(actual,F,'substance-use-disorder-treatment',countable), guideline_cardinality(actual,F,na,eq,1), guideline_event(actual,G,provide), guideline_arg(actual,G,1,E), guideline_arg(actual,G,2,F), guideline_arg(actual,G,3,C).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s49-10',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s49-10',1,ref(8),[A,B,C,D,E,F,G]),communicate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,to,C), guideline_entity(actual,E,'substance-use-disorder-treatment-provider',countable), guideline_cardinality(actual,E,na,eq,1), guideline_entity(actual,F,'substance-use-disorder-treatment',countable), guideline_cardinality(actual,F,na,eq,1), guideline_event(actual,G,provide), guideline_arg(actual,G,1,E), guideline_arg(actual,G,2,F), guideline_arg(actual,G,3,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s49-10',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s49-10',1,ref(8),[A,B,C,D,E,F,G]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,to,C), guideline_entity(actual,E,'substance-use-disorder-treatment-provider',countable), guideline_cardinality(actual,E,na,eq,1), guideline_entity(actual,F,'substance-use-disorder-treatment',countable), guideline_cardinality(actual,F,na,eq,1), guideline_event(actual,G,provide), guideline_arg(actual,G,1,E), guideline_arg(actual,G,2,F), guideline_arg(actual,G,3,C).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s49-10',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s49-10',1,ref(8),[A,B,C,D,E,F,G]),with,E) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,to,C), guideline_entity(actual,E,'substance-use-disorder-treatment-provider',countable), guideline_cardinality(actual,E,na,eq,1), guideline_entity(actual,F,'substance-use-disorder-treatment',countable), guideline_cardinality(actual,F,na,eq,1), guideline_event(actual,G,provide), guideline_arg(actual,G,1,E), guideline_arg(actual,G,2,F), guideline_arg(actual,G,3,C).
