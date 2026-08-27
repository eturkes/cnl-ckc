% cdc2022-opioid-rec09-imp08.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
:- multifile(guideline_schema_version/1).
:- discontiguous(guideline_schema_version/1).
:- multifile(guideline_document/3).
:- discontiguous(guideline_document/3).
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
guideline_schema_version(1).
guideline_document('cdc2022-opioid-rec09-imp08',ace_sha256('3aa96d7fb274a878e1e65b020f3573c9e0b873e82c19d0133735d260b80beb1c'),ulex(sha256('4a882a1996ea910df0fbf91fea897c0805e67d48f9e1ce2fcdbbbd9250c193ae'))).
% S1: If a clinician reviews a state-PDMP-record for a patient then the clinician should not dismiss the patient from a practice.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec09-imp08',1,box(1),[A,B,C,D]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'state-PDMP-record',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,review), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec09-imp08',1,box(1),[A,B,C,D]),'$guideline_id'(context,'cdc2022-opioid-rec09-imp08',1,box(2),[A,B,C,D]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'state-PDMP-record',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,review), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec09-imp08',1,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp08',1,ref(5),[A,B,C,D]),practice,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'state-PDMP-record',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,review), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec09-imp08',1,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp08',1,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'state-PDMP-record',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,review), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec09-imp08',1,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp08',1,ref(6),[A,B,C,D]),dismiss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'state-PDMP-record',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,review), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec09-imp08',1,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp08',1,ref(6),[A,B,C,D]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'state-PDMP-record',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,review), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec09-imp08',1,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp08',1,ref(6),[A,B,C,D]),2,C) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'state-PDMP-record',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,review), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec09-imp08',1,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp08',1,ref(6),[A,B,C,D]),from,'$guideline_id'(product,'cdc2022-opioid-rec09-imp08',1,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'state-PDMP-record',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,patient,countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,review), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
