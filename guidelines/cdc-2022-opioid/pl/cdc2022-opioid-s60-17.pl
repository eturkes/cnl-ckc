% cdc2022-opioid-s60-17.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s60-17',ace_sha256('4453cccaa764abe6071a196e365c076c3fb7326fef541a3d55bd604d26aa28cb'),ulex(sha256(be24cf56c59049d3c31fba641013faa22821ecda0669443a1fc4a49979ef57ae)),temporal(sha256('29a90281976bb48dfa32d9ef80651076896404c524c9bf1d7a55b06c1880be2a'))).
% S1: If a patient receives a naltrexone-treatment for an opioid-use-disorder and has a severe-acute-pain then every clinician may consider a short-term-higher-potency-nonopioid-analgesic for the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s60-17',1,box(1),[A,B,C,D,E,F,G]),may) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'naltrexone-treatment',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'opioid-use-disorder',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,receive), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,'severe-acute-pain',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,E), guideline_entity(actual,G,clinician,countable), guideline_cardinality(actual,G,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s60-17',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s60-17',1,ref(8),[A,B,C,D,E,F,G]),'short-term-higher-potency-nonopioid-analgesic',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'naltrexone-treatment',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'opioid-use-disorder',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,receive), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,'severe-acute-pain',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,E), guideline_entity(actual,G,clinician,countable), guideline_cardinality(actual,G,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s60-17',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s60-17',1,ref(8),[A,B,C,D,E,F,G]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'naltrexone-treatment',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'opioid-use-disorder',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,receive), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,'severe-acute-pain',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,E), guideline_entity(actual,G,clinician,countable), guideline_cardinality(actual,G,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s60-17',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s60-17',1,ref(9),[A,B,C,D,E,F,G]),consider) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'naltrexone-treatment',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'opioid-use-disorder',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,receive), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,'severe-acute-pain',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,E), guideline_entity(actual,G,clinician,countable), guideline_cardinality(actual,G,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s60-17',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s60-17',1,ref(9),[A,B,C,D,E,F,G]),1,G) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'naltrexone-treatment',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'opioid-use-disorder',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,receive), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,'severe-acute-pain',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,E), guideline_entity(actual,G,clinician,countable), guideline_cardinality(actual,G,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s60-17',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s60-17',1,ref(9),[A,B,C,D,E,F,G]),2,'$guideline_id'(product,'cdc2022-opioid-s60-17',1,ref(8),[A,B,C,D,E,F,G])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'naltrexone-treatment',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'opioid-use-disorder',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,receive), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,'severe-acute-pain',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,E), guideline_entity(actual,G,clinician,countable), guideline_cardinality(actual,G,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s60-17',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s60-17',1,ref(9),[A,B,C,D,E,F,G]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'naltrexone-treatment',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'opioid-use-disorder',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,receive), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,'severe-acute-pain',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,E), guideline_entity(actual,G,clinician,countable), guideline_cardinality(actual,G,na,eq,1).
