% cdc2022-opioid-rec05-imp08.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec05-imp08',ace_sha256('8d54372e2215933e970277407db550ba505dfa50b4b014db0d73a8380ec653df'),ulex(sha256('4a882a1996ea910df0fbf91fea897c0805e67d48f9e1ce2fcdbbbd9250c193ae'))).
% S1: If a patient engages in an opioid-taper then every clinician should conduct an at-least-monthly-follow-up with the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp08',1,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,engage), guideline_arg(actual,C,1,A), guideline_pp(actual,C,in,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp08',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp08',1,ref(5),[A,B,C,D]),'at-least-monthly-follow-up',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,engage), guideline_arg(actual,C,1,A), guideline_pp(actual,C,in,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp08',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp08',1,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,engage), guideline_arg(actual,C,1,A), guideline_pp(actual,C,in,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp08',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp08',1,ref(6),[A,B,C,D]),conduct) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,engage), guideline_arg(actual,C,1,A), guideline_pp(actual,C,in,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp08',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp08',1,ref(6),[A,B,C,D]),1,D) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,engage), guideline_arg(actual,C,1,A), guideline_pp(actual,C,in,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp08',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp08',1,ref(6),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp08',1,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,engage), guideline_arg(actual,C,1,A), guideline_pp(actual,C,in,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp08',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp08',1,ref(6),[A,B,C,D]),with,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-taper',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,engage), guideline_arg(actual,C,1,A), guideline_pp(actual,C,in,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
