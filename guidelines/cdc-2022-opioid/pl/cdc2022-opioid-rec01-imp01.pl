% cdc2022-opioid-rec01-imp01.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_schema_version(1).
guideline_document('cdc2022-opioid-rec01-imp01',ace_sha256('6e0aebe35e96bdf3da641fb34355287c16b504575c5262febeafc343bc3490b1'),ulex(sha256(ebe2d3db8559958f9ed44ec7a76cb2fb6ea75017f9f9437c3ea03b07092b0e75)),vocabulary(sha256('9d7632637554f8cbfe72e7d34a9b9d14c29eb8593aebb1c128a870c5dcf77ead'))).
% S1: If a nonopioid-pharmacologic-therapy is appropriate for a specific-condition then every clinician should maximize the nonopioid-pharmacologic-therapy for the specific-condition.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec01-imp01',1,box(1),[A,B,C,D,E]),should) :- guideline_entity(actual,A,'nonopioid-pharmacologic-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'specific-condition',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,appropriate,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_qualifier(actual,D,applicability,appropriate), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',1,ref(6),[A,B,C,D,E]),maximize) :- guideline_entity(actual,A,'nonopioid-pharmacologic-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'specific-condition',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,appropriate,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_qualifier(actual,D,applicability,appropriate), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',1,ref(6),[A,B,C,D,E]),1,E) :- guideline_entity(actual,A,'nonopioid-pharmacologic-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'specific-condition',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,appropriate,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_qualifier(actual,D,applicability,appropriate), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',1,ref(6),[A,B,C,D,E]),2,A) :- guideline_entity(actual,A,'nonopioid-pharmacologic-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'specific-condition',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,appropriate,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_qualifier(actual,D,applicability,appropriate), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',1,ref(6),[A,B,C,D,E]),for,B) :- guideline_entity(actual,A,'nonopioid-pharmacologic-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'specific-condition',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,appropriate,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_qualifier(actual,D,applicability,appropriate), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
% S2: If a nonpharmacologic-therapy is appropriate for a specific-condition then every clinician should maximize the nonpharmacologic-therapy for the specific-condition.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec01-imp01',2,box(1),[A,B,C,D,E]),should) :- guideline_entity(actual,A,'nonpharmacologic-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'specific-condition',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,appropriate,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_qualifier(actual,D,applicability,appropriate), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',2,ref(6),[A,B,C,D,E]),maximize) :- guideline_entity(actual,A,'nonpharmacologic-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'specific-condition',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,appropriate,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_qualifier(actual,D,applicability,appropriate), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',2,ref(6),[A,B,C,D,E]),1,E) :- guideline_entity(actual,A,'nonpharmacologic-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'specific-condition',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,appropriate,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_qualifier(actual,D,applicability,appropriate), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',2,ref(6),[A,B,C,D,E]),2,A) :- guideline_entity(actual,A,'nonpharmacologic-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'specific-condition',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,appropriate,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_qualifier(actual,D,applicability,appropriate), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',2,ref(6),[A,B,C,D,E]),for,B) :- guideline_entity(actual,A,'nonpharmacologic-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'specific-condition',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,appropriate,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_qualifier(actual,D,applicability,appropriate), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
