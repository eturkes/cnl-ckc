% cdc2022-opioid-s30-07.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s30-07',ace_sha256('8c6a54826383a05e5c1783bb56161ddc1cf699165ac0ac117c32a79c3c827d23'),ulex(sha256('0037ea725bdd10cf000798ce586d3739368b01cba88ac29668dbf1e4c9414552')),vocabulary(sha256('9d7632637554f8cbfe72e7d34a9b9d14c29eb8593aebb1c128a870c5dcf77ead'))).
% S1: Every clinician may use a medication-combination during a patient-specific-comorbidity-and-benefit-risk-selection.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s30-07',1,box(1),[A]),may) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-07',1,ref(2),[A]),'medication-combination',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-07',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-07',1,ref(3),[A]),'patient-specific-comorbidity-and-benefit-risk-selection',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-07',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s30-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-07',1,ref(4),[A]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-07',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-07',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s30-07',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s30-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-07',1,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s30-07',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: A medication-combination-example includes exactly 2 nonopioid-medications that differ in a mechanism-of-action.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s30-07',2,ref(1),[]),'medication-combination-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s30-07',2,ref(1),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s30-07',2,ref(2),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s30-07',2,ref(2),[]),1,'$guideline_id'(product,'cdc2022-opioid-s30-07',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s30-07',2,ref(2),[]),2,'$guideline_id'(product,'cdc2022-opioid-s30-07',2,ref(3),[])).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s30-07',2,ref(4),[]),in,'$guideline_id'(product,'cdc2022-opioid-s30-07',2,ref(5),[])).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s30-07',2,ref(4),[]),differ).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s30-07',2,ref(4),[]),1,'$guideline_id'(product,'cdc2022-opioid-s30-07',2,ref(3),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s30-07',2,ref(5),[]),'mechanism-of-action',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s30-07',2,ref(5),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s30-07',2,ref(3),[]),'nonopioid-medication',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s30-07',2,ref(3),[]),na,exactly,2).
% S3: A medication-combination-example includes a nonopioid-medication and includes an opioid.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s30-07',3,ref(1),[]),'medication-combination-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s30-07',3,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s30-07',3,ref(2),[]),'nonopioid-medication',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s30-07',3,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s30-07',3,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s30-07',3,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s30-07',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s30-07',3,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s30-07',3,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s30-07',3,ref(4),[]),opioid,countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s30-07',3,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s30-07',3,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s30-07',3,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s30-07',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s30-07',3,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s30-07',3,ref(4),[])).
