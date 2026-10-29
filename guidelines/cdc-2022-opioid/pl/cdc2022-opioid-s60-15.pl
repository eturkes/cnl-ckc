% cdc2022-opioid-s60-15.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s60-15',ace_sha256(b36123bdccef17243148fcca420f9529e6f75e48e5ba6a63f0b60c15c0693dc2),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If a patient receives a buprenorphine-treatment for an opioid-use-disorder and has a severe-acute-pain then every clinician may consider an additional-as-needed-buprenorphine-dose for the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s60-15',1,box(1),[A,B,C,D,E,F,G]),may) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'buprenorphine-treatment',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'opioid-use-disorder',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,receive), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,'severe-acute-pain',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,E), guideline_entity(actual,G,clinician,countable), guideline_cardinality(actual,G,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s60-15',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s60-15',1,ref(8),[A,B,C,D,E,F,G]),'additional-as-needed-buprenorphine-dose',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'buprenorphine-treatment',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'opioid-use-disorder',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,receive), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,'severe-acute-pain',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,E), guideline_entity(actual,G,clinician,countable), guideline_cardinality(actual,G,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s60-15',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s60-15',1,ref(8),[A,B,C,D,E,F,G]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'buprenorphine-treatment',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'opioid-use-disorder',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,receive), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,'severe-acute-pain',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,E), guideline_entity(actual,G,clinician,countable), guideline_cardinality(actual,G,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s60-15',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s60-15',1,ref(9),[A,B,C,D,E,F,G]),consider) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'buprenorphine-treatment',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'opioid-use-disorder',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,receive), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,'severe-acute-pain',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,E), guideline_entity(actual,G,clinician,countable), guideline_cardinality(actual,G,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s60-15',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s60-15',1,ref(9),[A,B,C,D,E,F,G]),1,G) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'buprenorphine-treatment',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'opioid-use-disorder',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,receive), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,'severe-acute-pain',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,E), guideline_entity(actual,G,clinician,countable), guideline_cardinality(actual,G,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s60-15',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s60-15',1,ref(9),[A,B,C,D,E,F,G]),2,'$guideline_id'(product,'cdc2022-opioid-s60-15',1,ref(8),[A,B,C,D,E,F,G])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'buprenorphine-treatment',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'opioid-use-disorder',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,receive), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,'severe-acute-pain',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,E), guideline_entity(actual,G,clinician,countable), guideline_cardinality(actual,G,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s60-15',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s60-15',1,ref(9),[A,B,C,D,E,F,G]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'buprenorphine-treatment',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'opioid-use-disorder',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,receive), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C), guideline_entity(actual,E,'severe-acute-pain',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,E), guideline_entity(actual,G,clinician,countable), guideline_cardinality(actual,G,na,eq,1).
% S2: A severe-acute-pain-cause-example includes a trauma-example and includes an unplanned-major-surgery-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s60-15',2,ref(1),[]),'severe-acute-pain-cause-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s60-15',2,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s60-15',2,ref(2),[]),'trauma-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s60-15',2,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s60-15',2,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s60-15',2,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s60-15',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s60-15',2,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s60-15',2,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s60-15',2,ref(4),[]),'unplanned-major-surgery-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s60-15',2,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s60-15',2,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s60-15',2,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s60-15',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s60-15',2,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s60-15',2,ref(4),[])).
