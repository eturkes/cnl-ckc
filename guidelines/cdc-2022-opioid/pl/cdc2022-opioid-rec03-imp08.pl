% cdc2022-opioid-rec03-imp08.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec03-imp08',ace_sha256(ec1167e8429d31b055bfc0c1194ba1da22c25866e70d3121645e2b04661893ff),ulex(sha256('3210e31a7a1a232faae4aed00d06481f6bef71ebd89b6c884183bc530be9986e')),vocabulary(sha256(b5cff67a2f9de9bee846e9e7650affaf95276ff3a26fc00befd8ceb076b0f4ac))).
% S1: If a clinician does not provably understand a transdermal-fentanyl-dosing then it is recommended that the clinician does not consider a transdermal-fentanyl-prescription.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec03-imp08',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'transdermal-fentanyl-dosing',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec03-imp08',1,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec03-imp08',1,box(2),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'transdermal-fentanyl-dosing',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec03-imp08',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp08',1,ref(4),[A]),'transdermal-fentanyl-prescription',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'transdermal-fentanyl-dosing',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec03-imp08',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp08',1,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'transdermal-fentanyl-dosing',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec03-imp08',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp08',1,ref(5),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'transdermal-fentanyl-dosing',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec03-imp08',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp08',1,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'transdermal-fentanyl-dosing',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec03-imp08',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp08',1,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec03-imp08',1,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'transdermal-fentanyl-dosing',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
% S2: If a clinician does not provably understand a transdermal-fentanyl-absorption then it is recommended that the clinician does not consider a transdermal-fentanyl-prescription.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec03-imp08',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'transdermal-fentanyl-absorption',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec03-imp08',2,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec03-imp08',2,box(2),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'transdermal-fentanyl-absorption',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec03-imp08',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp08',2,ref(4),[A]),'transdermal-fentanyl-prescription',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'transdermal-fentanyl-absorption',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec03-imp08',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp08',2,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'transdermal-fentanyl-absorption',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec03-imp08',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp08',2,ref(5),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'transdermal-fentanyl-absorption',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec03-imp08',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp08',2,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'transdermal-fentanyl-absorption',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec03-imp08',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp08',2,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec03-imp08',2,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'transdermal-fentanyl-absorption',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,understand), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
% S3: If a clinician does not provably prepare a patient-education then it is recommended that the clinician does not consider a transdermal-fentanyl-prescription.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec03-imp08',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'patient-education',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,prepare), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec03-imp08',3,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec03-imp08',3,box(2),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'patient-education',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,prepare), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec03-imp08',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp08',3,ref(4),[A]),'transdermal-fentanyl-prescription',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'patient-education',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,prepare), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec03-imp08',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp08',3,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'patient-education',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,prepare), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec03-imp08',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp08',3,ref(5),[A]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'patient-education',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,prepare), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec03-imp08',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp08',3,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'patient-education',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,prepare), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec03-imp08',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp08',3,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec03-imp08',3,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ (guideline_entity(actual,B,'patient-education',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,prepare), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B)).
