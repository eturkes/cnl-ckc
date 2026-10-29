% cdc2022-opioid-s37-05.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
:- multifile(guideline_supersession/4).
:- discontiguous(guideline_supersession/4).
:- multifile(guideline_evaluation_date/1).
:- discontiguous(guideline_evaluation_date/1).
guideline_schema_version(1).
guideline_document('cdc2022-opioid-s37-05',ace_sha256('7f3aa360cfab9275730419d1fabf803ff9e4dad414fad869d4aa8b4eab06791f'),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If a clinician does not provably detect a life-threatening-warning then it is possible that a rapid-opioid-taper-benefit does not likely outweigh a substantial-rapid-opioid-taper-risk.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s37-05',1,box(1),[A]),can) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'life-threatening-warning',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,detect), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s37-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-05',1,ref(4),[A]),'rapid-opioid-taper-benefit',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'life-threatening-warning',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,detect), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s37-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-05',1,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'life-threatening-warning',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,detect), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-s37-05',1,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-s37-05',1,box(2),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'life-threatening-warning',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,detect), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s37-05',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-05',1,ref(5),[A]),'substantial-rapid-opioid-taper-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'life-threatening-warning',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,detect), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s37-05',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-05',1,ref(5),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'life-threatening-warning',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,detect), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s37-05',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-05',1,ref(6),[A]),possibility,likely) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'life-threatening-warning',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,detect), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s37-05',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-05',1,ref(6),[A]),outweigh) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'life-threatening-warning',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,detect), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-05',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-05',1,ref(6),[A]),1,'$guideline_id'(product,'cdc2022-opioid-s37-05',1,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'life-threatening-warning',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,detect), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-05',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-05',1,ref(6),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s37-05',1,ref(5),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'life-threatening-warning',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,detect), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
% S2: If a clinician does not provably detect a life-threatening-warning then it is possible that an abrupt-opioid-discontinuation-benefit does not likely outweigh a substantial-abrupt-opioid-discontinuation-risk.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s37-05',2,box(1),[A]),can) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'life-threatening-warning',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,detect), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s37-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-05',2,ref(4),[A]),'abrupt-opioid-discontinuation-benefit',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'life-threatening-warning',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,detect), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s37-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-05',2,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'life-threatening-warning',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,detect), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-s37-05',2,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-s37-05',2,box(2),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'life-threatening-warning',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,detect), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s37-05',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-05',2,ref(5),[A]),'substantial-abrupt-opioid-discontinuation-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'life-threatening-warning',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,detect), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s37-05',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-05',2,ref(5),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'life-threatening-warning',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,detect), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s37-05',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-05',2,ref(6),[A]),possibility,likely) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'life-threatening-warning',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,detect), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s37-05',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-05',2,ref(6),[A]),outweigh) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'life-threatening-warning',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,detect), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-05',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-05',2,ref(6),[A]),1,'$guideline_id'(product,'cdc2022-opioid-s37-05',2,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'life-threatening-warning',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,detect), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-05',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-05',2,ref(6),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s37-05',2,ref(5),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'life-threatening-warning',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,detect), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
% S3: A life-threatening-warning-example is an imminent-overdose-warning-sign-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s37-05',3,ref(1),[]),'life-threatening-warning-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s37-05',3,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s37-05',3,ref(2),[]),'imminent-overdose-warning-sign-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s37-05',3,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s37-05',3,ref(3),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s37-05',3,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s37-05',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s37-05',3,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s37-05',3,ref(2),[])).
