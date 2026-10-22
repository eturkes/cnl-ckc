% cdc2022-opioid-s37-13.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s37-13',ace_sha256(a4ac52af9ff1ee41a13bbbc3e122889a3bc1b948ae802f3adbecf7dcc788bb5a),ulex(sha256(e8b9f2411b72a24bb2298ca9b1abaf93606dc5038829e6a3043fa9931bbc5ef8)),vocabulary(sha256(b5cff67a2f9de9bee846e9e7650affaf95276ff3a26fc00befd8ceb076b0f4ac))).
% S1: If a patient does not take a prescribed-opioid then the patient does not require an opioid-taper.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s37-13',1,box(2),[A]),-) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,B,-), guideline_entity(B,C,'prescribed-opioid',countable), guideline_cardinality(B,C,na,eq,1), guideline_event(B,D,take), guideline_arg(B,D,1,A), guideline_arg(B,D,2,C).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s37-13',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-13',1,ref(4),[A]),'opioid-taper',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,B,-), guideline_entity(B,C,'prescribed-opioid',countable), guideline_cardinality(B,C,na,eq,1), guideline_event(B,D,take), guideline_arg(B,D,1,A), guideline_arg(B,D,2,C).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s37-13',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-13',1,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,B,-), guideline_entity(B,C,'prescribed-opioid',countable), guideline_cardinality(B,C,na,eq,1), guideline_event(B,D,take), guideline_arg(B,D,1,A), guideline_arg(B,D,2,C).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s37-13',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-13',1,ref(5),[A]),require) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,B,-), guideline_entity(B,C,'prescribed-opioid',countable), guideline_cardinality(B,C,na,eq,1), guideline_event(B,D,take), guideline_arg(B,D,1,A), guideline_arg(B,D,2,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-13',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-13',1,ref(5),[A]),1,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,B,-), guideline_entity(B,C,'prescribed-opioid',countable), guideline_cardinality(B,C,na,eq,1), guideline_event(B,D,take), guideline_arg(B,D,1,A), guideline_arg(B,D,2,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-13',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-13',1,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s37-13',1,ref(4),[A])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_operator(actual,B,-), guideline_entity(B,C,'prescribed-opioid',countable), guideline_cardinality(B,C,na,eq,1), guideline_event(B,D,take), guideline_arg(B,D,1,A), guideline_arg(B,D,2,C).
