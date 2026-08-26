% cdc2022-opioid-s30-05.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s30-05',ace_sha256('16ce2b46e972e18d509df003146d8c17b65308e06dad602acbd8c76e48df3a70'),ulex(sha256('4c55bed7525c11e80d870254a4f876c23b6aa3c2a077242833ec32997784b01c'))).
% S1: If a multimodal-therapy is needed then every clinician should combine a medication with a nonpharmacologic-therapy.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s30-05',1,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,'multimodal-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,needed,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-05',1,ref(5),[A,B,C,D]),medication,countable) :- guideline_entity(actual,A,'multimodal-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,needed,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-05',1,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,'multimodal-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,needed,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-05',1,ref(6),[A,B,C,D]),'nonpharmacologic-therapy',countable) :- guideline_entity(actual,A,'multimodal-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,needed,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-05',1,ref(6),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,'multimodal-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,needed,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s30-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-05',1,ref(7),[A,B,C,D]),combine) :- guideline_entity(actual,A,'multimodal-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,needed,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-05',1,ref(7),[A,B,C,D]),1,D) :- guideline_entity(actual,A,'multimodal-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,needed,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-05',1,ref(7),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-s30-05',1,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,'multimodal-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,needed,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s30-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s30-05',1,ref(7),[A,B,C,D]),with,'$guideline_id'(product,'cdc2022-opioid-s30-05',1,ref(6),[A,B,C,D])) :- guideline_entity(actual,A,'multimodal-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,needed,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1).
% S2: A medication-nonpharmacologic-therapy-combination can improve a pain and can improve a patient-function.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(1),[]),'medication-nonpharmacologic-therapy-combination',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(1),[]),na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s30-05',2,box(1),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-05',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(2),[]),pain,countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-05',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(2),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s30-05',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(3),[]),improve).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-05',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-05',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(2),[])).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s30-05',2,box(2),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-05',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(4),[]),'patient-function',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-05',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(4),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s30-05',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(5),[]),improve).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-05',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-05',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s30-05',2,ref(4),[])).
