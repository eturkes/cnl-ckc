% cdc2022-opioid-s54-20.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s54-20',ace_sha256(f37a3aad22e6d51526f3dd0f8e7ab62565cf10f1559b28352bcd41cddbf6d18f),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If a clinician asks a patient about a medication-and-substance-use before a toxicology-test then the patient gains a drug-use-change-information-opportunity.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s54-20',1,ref(6),[A,B,C,D,E]),'drug-use-change-information-opportunity',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'medication-and-substance-use',countable), guideline_cardinality(actual,C,na,eq,1), guideline_entity(actual,D,'toxicology-test',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,ask), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,B), guideline_pp(actual,E,before,D), guideline_order(actual,E,before,D), guideline_pp(actual,E,about,C).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s54-20',1,ref(6),[A,B,C,D,E]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'medication-and-substance-use',countable), guideline_cardinality(actual,C,na,eq,1), guideline_entity(actual,D,'toxicology-test',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,ask), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,B), guideline_pp(actual,E,before,D), guideline_order(actual,E,before,D), guideline_pp(actual,E,about,C).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s54-20',1,ref(7),[A,B,C,D,E]),gain) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'medication-and-substance-use',countable), guideline_cardinality(actual,C,na,eq,1), guideline_entity(actual,D,'toxicology-test',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,ask), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,B), guideline_pp(actual,E,before,D), guideline_order(actual,E,before,D), guideline_pp(actual,E,about,C).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s54-20',1,ref(7),[A,B,C,D,E]),1,B) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'medication-and-substance-use',countable), guideline_cardinality(actual,C,na,eq,1), guideline_entity(actual,D,'toxicology-test',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,ask), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,B), guideline_pp(actual,E,before,D), guideline_order(actual,E,before,D), guideline_pp(actual,E,about,C).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s54-20',1,ref(7),[A,B,C,D,E]),2,'$guideline_id'(product,'cdc2022-opioid-s54-20',1,ref(6),[A,B,C,D,E])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'medication-and-substance-use',countable), guideline_cardinality(actual,C,na,eq,1), guideline_entity(actual,D,'toxicology-test',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,ask), guideline_arg(actual,E,1,A), guideline_arg(actual,E,2,B), guideline_pp(actual,E,before,D), guideline_order(actual,E,before,D), guideline_pp(actual,E,about,C).
