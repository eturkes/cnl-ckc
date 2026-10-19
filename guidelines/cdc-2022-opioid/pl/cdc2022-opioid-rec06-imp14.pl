% cdc2022-opioid-rec06-imp14.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec06-imp14',ace_sha256(f98ca6cc5007282eb5a9bb0714022bc63f22f29698b74da9a16336bd7776a9b9),ulex(sha256('0037ea725bdd10cf000798ce586d3739368b01cba88ac29668dbf1e4c9414552')),vocabulary(sha256('9d7632637554f8cbfe72e7d34a9b9d14c29eb8593aebb1c128a870c5dcf77ead'))).
% S1: If a patient has an initial-opioid-prescription and the initial-opioid-prescription has a duration and the patient needs a taper then a taper-duration can possibly need an adjustment for the duration.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec06-imp14',1,ref(8),[A,B,C,D,E,F,G]),'taper-duration',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'initial-opioid-prescription',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,duration,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_entity(actual,F,taper,countable), guideline_cardinality(actual,F,na,eq,1), guideline_event(actual,G,need), guideline_arg(actual,G,1,A), guideline_arg(actual,G,2,F).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec06-imp14',1,ref(8),[A,B,C,D,E,F,G]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'initial-opioid-prescription',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,duration,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_entity(actual,F,taper,countable), guideline_cardinality(actual,F,na,eq,1), guideline_event(actual,G,need), guideline_arg(actual,G,1,A), guideline_arg(actual,G,2,F).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec06-imp14',1,box(1),[A,B,C,D,E,F,G]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'initial-opioid-prescription',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,duration,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_entity(actual,F,taper,countable), guideline_cardinality(actual,F,na,eq,1), guideline_event(actual,G,need), guideline_arg(actual,G,1,A), guideline_arg(actual,G,2,F).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec06-imp14',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-rec06-imp14',1,ref(9),[A,B,C,D,E,F,G]),adjustment,countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'initial-opioid-prescription',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,duration,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_entity(actual,F,taper,countable), guideline_cardinality(actual,F,na,eq,1), guideline_event(actual,G,need), guideline_arg(actual,G,1,A), guideline_arg(actual,G,2,F).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec06-imp14',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-rec06-imp14',1,ref(9),[A,B,C,D,E,F,G]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'initial-opioid-prescription',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,duration,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_entity(actual,F,taper,countable), guideline_cardinality(actual,F,na,eq,1), guideline_event(actual,G,need), guideline_arg(actual,G,1,A), guideline_arg(actual,G,2,F).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-rec06-imp14',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-rec06-imp14',1,ref(10),[A,B,C,D,E,F,G]),possibility,possibly) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'initial-opioid-prescription',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,duration,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_entity(actual,F,taper,countable), guideline_cardinality(actual,F,na,eq,1), guideline_event(actual,G,need), guideline_arg(actual,G,1,A), guideline_arg(actual,G,2,F).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec06-imp14',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-rec06-imp14',1,ref(10),[A,B,C,D,E,F,G]),need) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'initial-opioid-prescription',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,duration,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_entity(actual,F,taper,countable), guideline_cardinality(actual,F,na,eq,1), guideline_event(actual,G,need), guideline_arg(actual,G,1,A), guideline_arg(actual,G,2,F).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec06-imp14',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-rec06-imp14',1,ref(10),[A,B,C,D,E,F,G]),1,'$guideline_id'(product,'cdc2022-opioid-rec06-imp14',1,ref(8),[A,B,C,D,E,F,G])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'initial-opioid-prescription',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,duration,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_entity(actual,F,taper,countable), guideline_cardinality(actual,F,na,eq,1), guideline_event(actual,G,need), guideline_arg(actual,G,1,A), guideline_arg(actual,G,2,F).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec06-imp14',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-rec06-imp14',1,ref(10),[A,B,C,D,E,F,G]),2,'$guideline_id'(product,'cdc2022-opioid-rec06-imp14',1,ref(9),[A,B,C,D,E,F,G])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'initial-opioid-prescription',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,duration,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_entity(actual,F,taper,countable), guideline_cardinality(actual,F,na,eq,1), guideline_event(actual,G,need), guideline_arg(actual,G,1,A), guideline_arg(actual,G,2,F).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec06-imp14',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-rec06-imp14',1,ref(10),[A,B,C,D,E,F,G]),for,D) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'initial-opioid-prescription',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,duration,countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,E,have), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_entity(actual,F,taper,countable), guideline_cardinality(actual,F,na,eq,1), guideline_event(actual,G,need), guideline_arg(actual,G,1,A), guideline_arg(actual,G,2,F).
