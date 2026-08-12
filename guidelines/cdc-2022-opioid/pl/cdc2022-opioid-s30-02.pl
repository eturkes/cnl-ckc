% cdc2022-opioid-s30-02.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s30-02',ace_sha256('1c99fb49d50b879a6b6fc7fc64d7c685c7961af0faab78d2c2aeca2000c75b8c'),ulex(sha256('2a4a3e829c919b3dd3b20356e7a69585cfb305c3c57dfb531e3f2c1eb97637b6'))).
% S1: If a clinician performs an interventional-procedure then the clinician should have a proper-interventional-procedure-training.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s30-02',1,box(1),[A,B,C]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'interventional-procedure',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,perform), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-02',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s30-02',1,ref(4),[A,B,C]),'proper-interventional-procedure-training',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'interventional-procedure',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,perform), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-02',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s30-02',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'interventional-procedure',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,perform), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s30-02',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s30-02',1,ref(5),[A,B,C]),have) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'interventional-procedure',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,perform), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-02',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s30-02',1,ref(5),[A,B,C]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'interventional-procedure',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,perform), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-02',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s30-02',1,ref(5),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s30-02',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'interventional-procedure',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,perform), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S2: If a clinician performs an interventional-procedure then the clinician should follow a meticulous-infection-control-protocol.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s30-02',2,box(1),[A,B,C]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'interventional-procedure',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,perform), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-02',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s30-02',2,ref(4),[A,B,C]),'meticulous-infection-control-protocol',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'interventional-procedure',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,perform), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-02',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s30-02',2,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'interventional-procedure',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,perform), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s30-02',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s30-02',2,ref(5),[A,B,C]),follow) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'interventional-procedure',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,perform), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-02',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s30-02',2,ref(5),[A,B,C]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'interventional-procedure',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,perform), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-02',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s30-02',2,ref(5),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s30-02',2,ref(4),[A,B,C])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'interventional-procedure',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,perform), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
