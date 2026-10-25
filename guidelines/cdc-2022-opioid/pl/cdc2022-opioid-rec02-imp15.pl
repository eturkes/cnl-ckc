% cdc2022-opioid-rec02-imp15.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec02-imp15',ace_sha256(db1f1ba6796c2b5a8546718c82cd6eada9e11132df8f5ff33277fbc986ed828b),ulex(sha256('64c086862c2f48d491a9fde0ba9f0fcf090e25e84456b9749332ec3ace211ce4')),vocabulary(sha256(c36289aa40f75805a83501ca057b29dd1ba7292b025716853f654890b77ec50c))).
% S1: If there is a clinician then it is recommended that the clinician does not impose a nonpharmacologic-failure-requirement before an opioid-therapy.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp15',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',1,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec02-imp15',1,box(2),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',1,ref(2),[A]),'nonpharmacologic-failure-requirement',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',1,ref(3),[A]),'opioid-therapy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',1,ref(4),[A]),impose) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',1,ref(4),[A]),before,'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_order('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',1,ref(4),[A]),before,'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: If there is a clinician then it is recommended that the clinician does not impose a nonopioid-pharmacologic-failure-requirement before an opioid-therapy.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp15',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',2,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec02-imp15',2,box(2),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',2,ref(2),[A]),'nonopioid-pharmacologic-failure-requirement',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',2,ref(3),[A]),'opioid-therapy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',2,ref(4),[A]),impose) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',2,ref(4),[A]),before,'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_order('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',2,ref(4),[A]),before,'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: If there is a clinician then it is recommended that the clinician does not impose a specific-treatment-requirement before an opioid-therapy.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp15',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',3,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec02-imp15',3,box(2),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',3,ref(2),[A]),'specific-treatment-requirement',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',3,ref(3),[A]),'opioid-therapy',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',3,ref(4),[A]),impose) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',3,ref(4),[A]),before,'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_order('$guideline_id'(context,'cdc2022-opioid-rec02-imp15',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',3,ref(4),[A]),before,'$guideline_id'(product,'cdc2022-opioid-rec02-imp15',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
