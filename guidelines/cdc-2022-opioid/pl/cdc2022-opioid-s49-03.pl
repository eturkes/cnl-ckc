% cdc2022-opioid-s49-03.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_schema_version(1).
guideline_document('cdc2022-opioid-s49-03',ace_sha256('04445cd9e7fbb9d9fe3913425ba6e112af6518ff0bb81349e711fb2c2807f4c8'),ulex(sha256(e8b9f2411b72a24bb2298ca9b1abaf93606dc5038829e6a3043fa9931bbc5ef8)),vocabulary(sha256(b5cff67a2f9de9bee846e9e7650affaf95276ff3a26fc00befd8ceb076b0f4ac))).
% S1: If a clinician considers an opioid then the clinician should always consider the opioid cautiously.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s49-03',1,box(1),[A,B,C]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,consider), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), \+ guideline_exclusion('cdc-2022-opioid').
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s49-03',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s49-03',1,ref(4),[A,B,C]),usuality,always) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,consider), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s49-03',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s49-03',1,ref(4),[A,B,C]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,consider), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s49-03',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s49-03',1,ref(4),[A,B,C]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,consider), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s49-03',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s49-03',1,ref(4),[A,B,C]),2,B) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,consider), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s49-03',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s49-03',1,ref(4),[A,B,C]),manner,cautiously) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,consider), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S2: If a clinician prescribes an opioid then the clinician should always prescribe the opioid cautiously.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s49-03',2,box(1),[A,B,C]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,prescribe), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), \+ guideline_exclusion('cdc-2022-opioid').
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s49-03',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s49-03',2,ref(4),[A,B,C]),usuality,always) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,prescribe), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s49-03',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s49-03',2,ref(4),[A,B,C]),prescribe) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,prescribe), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s49-03',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s49-03',2,ref(4),[A,B,C]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,prescribe), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s49-03',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s49-03',2,ref(4),[A,B,C]),2,B) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,prescribe), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s49-03',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s49-03',2,ref(4),[A,B,C]),manner,cautiously) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,prescribe), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S3: If an opioid-risk-stratification-tool is available and has a risk-ruling-out-ability for a long-term-opioid-therapy and there is a clinician then it is recommended that the clinician does not overestimate the risk-ruling-out-ability.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s49-03',3,box(1),[A,B,C,D,E,F,G]),should) :- guideline_entity(actual,A,'opioid-risk-stratification-tool',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,available,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_qualifier(actual,C,applicability,available), guideline_entity(actual,D,'risk-ruling-out-ability',countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,'long-term-opioid-therapy',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,D), guideline_pp(actual,F,for,E), guideline_entity(actual,G,clinician,countable), guideline_cardinality(actual,G,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-s49-03',3,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(context,'cdc2022-opioid-s49-03',3,box(2),[A,B,C,D,E,F,G]),-) :- guideline_entity(actual,A,'opioid-risk-stratification-tool',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,available,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_qualifier(actual,C,applicability,available), guideline_entity(actual,D,'risk-ruling-out-ability',countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,'long-term-opioid-therapy',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,D), guideline_pp(actual,F,for,E), guideline_entity(actual,G,clinician,countable), guideline_cardinality(actual,G,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s49-03',3,box(2),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s49-03',3,ref(8),[A,B,C,D,E,F,G]),overestimate) :- guideline_entity(actual,A,'opioid-risk-stratification-tool',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,available,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_qualifier(actual,C,applicability,available), guideline_entity(actual,D,'risk-ruling-out-ability',countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,'long-term-opioid-therapy',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,D), guideline_pp(actual,F,for,E), guideline_entity(actual,G,clinician,countable), guideline_cardinality(actual,G,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s49-03',3,box(2),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s49-03',3,ref(8),[A,B,C,D,E,F,G]),1,G) :- guideline_entity(actual,A,'opioid-risk-stratification-tool',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,available,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_qualifier(actual,C,applicability,available), guideline_entity(actual,D,'risk-ruling-out-ability',countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,'long-term-opioid-therapy',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,D), guideline_pp(actual,F,for,E), guideline_entity(actual,G,clinician,countable), guideline_cardinality(actual,G,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s49-03',3,box(2),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s49-03',3,ref(8),[A,B,C,D,E,F,G]),2,D) :- guideline_entity(actual,A,'opioid-risk-stratification-tool',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,available,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_qualifier(actual,C,applicability,available), guideline_entity(actual,D,'risk-ruling-out-ability',countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,'long-term-opioid-therapy',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,D), guideline_pp(actual,F,for,E), guideline_entity(actual,G,clinician,countable), guideline_cardinality(actual,G,na,eq,1).
