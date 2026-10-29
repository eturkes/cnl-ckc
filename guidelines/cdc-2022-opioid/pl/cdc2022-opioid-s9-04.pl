% cdc2022-opioid-s9-04.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s9-04',ace_sha256('71a7053bcdf7ed331c1460cd349650dd12814bafcbf8413f4267fa38d35d8ab8'),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: Every pain-duration-definition is an approximate-guide.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-04',1,ref(2),[A]),'approximate-guide',countable) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-04',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-04',1,ref(3),[A]),be) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-04',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-04',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-04',1,ref(2),[A])) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every pain-duration-definition is not an absolute-boundary.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-04',2,box(1),[A]),-) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',2,ref(2),[A]),'absolute-boundary',countable) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',2,ref(3),[A]),be) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-04',2,ref(2),[A])) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every pain-duration-definition can facilitate a recommendation-consideration for a clinician and can facilitate a recommendation-consideration for a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-04',3,box(1),[A]),can) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-04',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',3,ref(2),[A]),'recommendation-consideration',countable) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-04',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-04',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',3,ref(3),[A]),clinician,countable) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-04',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-04',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',3,ref(4),[A]),facilitate) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-04',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-04',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-04',3,ref(2),[A])) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s9-04',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',3,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s9-04',3,ref(3),[A])) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-04',3,box(2),[A]),can) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-04',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',3,ref(5),[A]),'recommendation-consideration',countable) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-04',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',3,ref(5),[A]),na,eq,1) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-04',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',3,ref(6),[A]),patient,countable) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-04',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',3,ref(6),[A]),na,eq,1) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-04',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',3,ref(7),[A]),facilitate) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-04',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',3,ref(7),[A]),1,A) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-04',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',3,ref(7),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-04',3,ref(5),[A])) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s9-04',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',3,ref(7),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s9-04',3,ref(6),[A])) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every pain-duration-definition can facilitate a practical-recommendation-use for a clinician and can facilitate a practical-recommendation-use for a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-04',4,box(1),[A]),can) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-04',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',4,ref(2),[A]),'practical-recommendation-use',countable) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-04',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-04',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',4,ref(3),[A]),clinician,countable) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-04',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-04',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',4,ref(4),[A]),facilitate) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-04',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-04',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-04',4,ref(2),[A])) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s9-04',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',4,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s9-04',4,ref(3),[A])) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-04',4,box(2),[A]),can) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-04',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',4,ref(5),[A]),'practical-recommendation-use',countable) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-04',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',4,ref(5),[A]),na,eq,1) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-04',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',4,ref(6),[A]),patient,countable) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-04',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',4,ref(6),[A]),na,eq,1) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-04',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',4,ref(7),[A]),facilitate) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-04',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',4,ref(7),[A]),1,A) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-04',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',4,ref(7),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-04',4,ref(5),[A])) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s9-04',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-04',4,ref(7),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s9-04',4,ref(6),[A])) :- guideline_entity(actual,A,'pain-duration-definition',countable), guideline_cardinality(actual,A,na,eq,1).
