% cdc2022-opioid-s37-13.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s37-13',ace_sha256(a4ac52af9ff1ee41a13bbbc3e122889a3bc1b948ae802f3adbecf7dcc788bb5a),ulex(sha256('2a4a3e829c919b3dd3b20356e7a69585cfb305c3c57dfb531e3f2c1eb97637b6'))).
% S1: If a patient does not take a prescribed-opioid then the patient does not require an opioid-taper.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s37-13',1,box(2),[A]),-) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,B,-), guideline_entity(B,C,'prescribed-opioid',countable), guideline_cardinality(B,C,na,eq,1), guideline_event(B,D,take), guideline_arg(B,D,1,A), guideline_arg(B,D,2,C).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s37-13',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-13',1,ref(4),[A]),'opioid-taper',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,B,-), guideline_entity(B,C,'prescribed-opioid',countable), guideline_cardinality(B,C,na,eq,1), guideline_event(B,D,take), guideline_arg(B,D,1,A), guideline_arg(B,D,2,C).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s37-13',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-13',1,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,B,-), guideline_entity(B,C,'prescribed-opioid',countable), guideline_cardinality(B,C,na,eq,1), guideline_event(B,D,take), guideline_arg(B,D,1,A), guideline_arg(B,D,2,C).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s37-13',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-13',1,ref(5),[A]),require) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,B,-), guideline_entity(B,C,'prescribed-opioid',countable), guideline_cardinality(B,C,na,eq,1), guideline_event(B,D,take), guideline_arg(B,D,1,A), guideline_arg(B,D,2,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-13',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-13',1,ref(5),[A]),1,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,B,-), guideline_entity(B,C,'prescribed-opioid',countable), guideline_cardinality(B,C,na,eq,1), guideline_event(B,D,take), guideline_arg(B,D,1,A), guideline_arg(B,D,2,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-13',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-13',1,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s37-13',1,ref(4),[A])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,B,-), guideline_entity(B,C,'prescribed-opioid',countable), guideline_cardinality(B,C,na,eq,1), guideline_event(B,D,take), guideline_arg(B,D,1,A), guideline_arg(B,D,2,C).
